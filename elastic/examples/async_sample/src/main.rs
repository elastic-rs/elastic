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
use futures::future::ok;
use futures::sync::mpsc::SendError;

use memmap::{Mmap, MmapViewSync, Protection};
use hyper::{Method, Error as HyperError};
use hyper::header::ContentType;
use hyper::client::Request;

quick_error!{
    #[derive(Debug)]
    enum RequestError {
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

    let mut core = Core::new().unwrap();

    let client = hyper::client::Config::default()
        .body::<MappedFileBody>()
        .build(&core.handle());

    let (buffer, body) = mapped_file("./data/accounts.json").unwrap();

    let buffer = buffer.map_err(|e| RequestError::from(e));

    let req = BulkRequest::new(body);
    let req = client.request(hyper_req(&url, req))
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

                                futures::finished(())
                           })
                    })
                    .map_err(|e| RequestError::from(e));

    let do_req = req.join(buffer);

    core.run(do_req).unwrap();
}

/*
 - Get a mapped file
 - Split by chunk size
 - For each chunk, stream
*/

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
        }
        else {
            Ok(Async::Ready(None))
        }
    }
}

/// A future for streaming the request body from a file.
/// 
/// The first item is a future that will stream chunks from the mapped file.
/// The second item is the async body to use in the request.
fn mapped_file<P>(path: P) -> Result<(Box<Future<Item = (), Error = SendError<ChunkResult>> + Send>, MappedFileBody), IoError> 
    where P: AsRef<Path>
{
    let file = Mmap::open_path(path, Protection::Read)?.into_view_sync();

    let total_len = file.len();

    let slice_size = total_len;
    // TODO: Chunked has issues deriving xcontent?
    // Looks like it needs to be aware of doc boundaries
    // let slice_size = 16000;
    
    let num_slices = (total_len as f32 / slice_size as f32).ceil() as usize;

    let mut slices: Vec<ChunkResult> = Vec::with_capacity(num_slices);

    let mut next = Some(file);
    while let Some(rest) = next {
        let (slice, rest) = rest.split_at(slice_size)?;

        slices.push(Ok(Chunk(slice)));

        next = match rest.len() {
            // >1 remaining chunk
            len if len > slice_size => {
                Some(rest)
            },
            // EOF
            0 => {
                None
            },
            // Last chunk
            _ => {
                slices.push(Ok(Chunk(rest)));
                None
            }
        }
    }

    println!("len: {}, slices: {}", total_len, slices.len());

    let (tx, rx) = MappedFileBody::pair();

    let streamed = ChunkStream(slices);

    let tx_future = tx.send_all(streamed)
                      .and_then(|_| ok(()));

    Ok((tx_future.boxed(), rx))
}

/// Build a `hyper` request from an `elastic` request.
fn hyper_req<I, B>(base_url: &str, req: I) -> Request<B>
    where I: Into<req::HttpRequest<'static, B>>,
          B: Stream<Error=hyper::Error> + 'static,
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
        req::HttpMethod::Get => {
            Request::<B>::new(Method::Get, url)
        },
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

    req
}
