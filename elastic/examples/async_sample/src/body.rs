use std::collections::VecDeque;
use std::path::Path;
use std::io::Error as IoError;

use tokio_proto::streaming::Body;
use futures::{Future, Stream, Sink, Poll, Async};
use futures::future::{ok, err, lazy, FutureResult};
use futures::sync::mpsc::SendError;
use memmap::{Mmap, MmapViewSync, Protection};
use hyper::Error as HyperError;

use error::*;

/// A stream of file chunks.
pub type MappedFileBody = Body<Chunk, HyperError>;

/// A chunk of a file.
pub struct Chunk(MmapViewSync);

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] {
        unsafe { self.0.as_slice() }
    }
}

type ChunkResult = Result<Chunk, HyperError>;

struct ChunkStream(VecDeque<ChunkResult>);

impl Stream for ChunkStream {
    type Item = ChunkResult;
    type Error = SendError<ChunkResult>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(slice) = self.0.pop_front() {
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
pub fn mapped_file<P>
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

fn map_file_to_chunks<P>(path: P) -> FutureResult<VecDeque<ChunkResult>, RequestError>
    where P: AsRef<Path>
{
    let file = match Mmap::open_path(path, Protection::Read) {
        Ok(file) => file,
        Err(e) => return err(e.into()),
    };

    let file = file.into_view_sync();

    let total_len = file.len();

    if total_len == 0 {
        return ok(VecDeque::new());
    }

    let slice_size = 1024 * 16;

    let num_slices = (total_len as f32 / slice_size as f32).ceil() as usize;

    let mut slices = VecDeque::with_capacity(num_slices);

    let mut next = Some(file);
    while let Some(rest) = next {
        next = match rest.len() {
            // >1 chunk size left
            len if len > slice_size => {
                let (slice, rest) = match rest.split_at(slice_size) {
                    Ok(split) => split,
                    Err(e) => return err(e.into()),
                };

                slices.push_back(Ok(Chunk(slice)));

                Some(rest)
            }
            // EOF
            0 => None,
            // Last chunk
            _ => {
                slices.push_back(Ok(Chunk(rest)));

                None
            }
        };
    }

    ok(slices)
}
