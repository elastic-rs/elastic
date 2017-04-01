#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate json_str;

extern crate elastic_requests as req;
extern crate elastic_responses as res;
extern crate elastic_types;

extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate tokio_proto;
extern crate tokio_core;

extern crate memmap;
extern crate hyper;
extern crate futures_cpupool;

#[macro_use]
extern crate quick_error;

use std::path::Path;
use std::str::{self, FromStr};
use std::io::{Read, Error as IoError};

use req::BulkRequest;

use tokio_core::reactor::Core;
use tokio_proto::streaming::Body;
use futures::{Future, Stream, Sink, Poll, Async};
use futures::future::{ok, err, lazy, FutureResult};
use futures::sync::mpsc::SendError;
use futures_cpupool::CpuPool;

use memmap::{Mmap, MmapViewSync, Protection};
use hyper::{Method, Error as HyperError};
use hyper::header::ContentType;
use hyper::client::Request;

quick_error!{
    #[derive(Debug)]
    enum RequestError {
        Io(err: IoError) {
            from()
        }
        Body(err: SendError<ChunkResult>) {
            from()
        }
        Request(err: HyperError) {
            from()
        }
    }
}

fn main() {
    let url = "http://localhost:9200";

    // Create an IO core and thread pool
    let mut core = Core::new().unwrap();
    let pool = CpuPool::new(4);

    // Create a `hyper` client expecting `MappedFileBody` bodies
    let client = hyper::client::Config::default()
        .body::<MappedFileBody>()
        .build(&core.handle());

    // Get a future to buffer a bulk file
    let (buffer_future, body) = mapped_file("./data/accounts.json").unwrap();
    let buffer_future = pool.spawn(buffer_future);

    // Get a future to send a bulk request
    let req = BulkRequest::new(body);
    let req_future = client.request(hyper_req(&url, req))
        .and_then(|res| {
            res.body()
                .fold(Vec::new(), |mut buf, chunk| {
                    chunk.as_ref()
                        .read_to_end(&mut buf)
                        .map(|_| buf)
                })
                .and_then(|buf| {
                    // TODO: Deserialize the response on the cpu pool

                    println!("{:?}", str::from_utf8(&buf).unwrap());

                    ok(())
                })
        })
        .map_err(|e| e.into());

    // Join the future to buffer the request body with the future to send the request
    let req_future = buffer_future.join(req_future);

    core.run(req_future).unwrap();
}

// - Get a mapped file
// - Split by chunk size
// - For each chunk, stream
//

struct Chunk(MmapViewSync);

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] {
        unsafe { self.0.as_slice() }
    }
}

type MappedFileBody = Body<Chunk, HyperError>;
type ChunkResult = Result<Chunk, HyperError>;

struct ChunkStream(Vec<ChunkResult>);

impl Stream for ChunkStream {
    type Item = ChunkResult;
    type Error = SendError<ChunkResult>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(slice) = self.0.pop() {
            Ok(Async::Ready(Some(slice)))
        } else {
            Ok(Async::Ready(None))
        }
    }
}

/// A future for streaming the request body from a file.
///
/// The first item is a future that will stream chunks from the mapped file.
/// The second item is the async body to use in the request.
fn mapped_file<P>
    (path: P)
     -> Result<(Box<Future<Item = (), Error = RequestError> + Send>, MappedFileBody), IoError>
    where P: AsRef<Path> + Send + 'static
{
    let (tx, rx) = MappedFileBody::pair();

    let tx_future = lazy(move || {
        let slices_future = map_file_to_chunks(path);

        slices_future.and_then(|slices| {
            let streamed = ChunkStream(slices);

            tx.send_all(streamed).map_err(|e| e.into())
        })
    });

    let tx_future = tx_future.and_then(|_| ok(()));

    Ok((tx_future.boxed(), rx))
}

fn map_file_to_chunks<P>(path: P) -> FutureResult<Vec<ChunkResult>, RequestError>
    where P: AsRef<Path>
{
    let file = match Mmap::open_path(path, Protection::Read) {
        Ok(file) => file,
        Err(e) => return err(e.into()),
    };

    let file = file.into_view_sync();

    let total_len = file.len();

    if total_len == 0 {
        return ok(vec![])
    }

    let slice_size = total_len;
    // TODO: Chunked has issues deriving xcontent?
    // Looks like it needs to be aware of doc boundaries
    // let slice_size = 16384;

    let num_slices = (total_len as f32 / slice_size as f32).ceil() as usize;

    let mut slices: Vec<ChunkResult> = Vec::with_capacity(num_slices);

    let mut next = Some(file);
    while let Some(rest) = next {
        let (slice, rest) = match rest.split_at(slice_size) {
            Ok(split) => split,
            Err(e) => return err(e.into()),
        };

        slices.push(Ok(Chunk(slice)));

        next = match rest.len() {
            // >1 remaining chunk, continue loop
            len if len > slice_size => Some(rest),
            // EOF, break
            0 => None,
            // Last chunk, push and break
            _ => {
                slices.push(Ok(Chunk(rest)));
                None
            }
        }
    }

    println!("len: {}, slices: {}", total_len, slices.len());

    ok(slices)
}

/// Build a `hyper` request from an `elastic` request.
fn hyper_req<I, B>(base_url: &str, req: I) -> Request<B>
    where I: Into<req::HttpRequest<'static, B>>,
          B: Stream<Error = hyper::Error> + 'static + ::std::fmt::Debug,
          B::Item: AsRef<[u8]>
{
    let req = req.into();

    let mut url = String::with_capacity(base_url.len() + req.url.len());

    url.push_str(base_url);
    url.push_str(&req.url);

    let url = hyper::Uri::from_str(&url).unwrap();

    let method = req.method;
    let body = req.body;

    let mut req = match method {
        req::HttpMethod::Get => Request::<B>::new(Method::Get, url),
        req::HttpMethod::Post => {
            let mut req = Request::<B>::new(Method::Post, url);

            if let Some(body) = body {
                req.set_body(body);
            }

            req
        }
        _ => unimplemented!(),
    };

    {
        let mut headers = req.headers_mut();
        headers.set(ContentType::json())
    }

    println!("built req");

    req
}
