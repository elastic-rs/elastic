use std::collections::VecDeque;
use std::path::Path;
use std::io::Error as IoError;

use tokio_proto::streaming::Body as BodyStream;
use futures::{IntoFuture, Future, Stream, Sink, Poll, Async};
use futures::future::lazy;
use futures::sync::mpsc::SendError;
use memmap::{Mmap, MmapViewSync, Protection};
use hyper::Error as HyperError;

use error::*;

const SLICE_SIZE: usize = 1024 * 16;

/// A stream of file chunks.
pub type FileBody = BodyStream<FileChunk, HyperError>;

/// A chunk of a file.
pub struct FileChunk(MmapViewSync);

impl AsRef<[u8]> for FileChunk {
    fn as_ref(&self) -> &[u8] {
        // Safe because the file is immutable
        unsafe { self.0.as_slice() }
    }
}

// Plumbing for streaming a mapped file in chunks
pub type FileChunkResult = Result<FileChunk, HyperError>;

struct FileChunkStream(VecDeque<FileChunkResult>);

impl Stream for FileChunkStream {
    type Item = FileChunkResult;
    type Error = SendError<FileChunkResult>;

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
     -> (impl Future<Item = (), Error = Error> + Send, FileBody)
    where P: AsRef<Path> + Send + 'static
{
    let (tx, rx) = FileBody::pair();

    let tx_future = lazy(move || {
        let slices_future = map_file_to_chunks(path).into_future();

        slices_future.and_then(|slices| {
            let streamed = FileChunkStream(slices);

            tx.send_all(streamed).map_err(Into::into)
        })
    });

    let tx_future = tx_future.map(|_| ());

    (tx_future, rx)
}

// mmap a file and push its chunks into a queue
fn map_file_to_chunks<P>(path: P) -> Result<VecDeque<FileChunkResult>, Error>
    where P: AsRef<Path>
{
    let file = Mmap::open_path(path, Protection::Read)?.into_view_sync();

    let total_len = file.len();
    if total_len == 0 {
        return Ok(VecDeque::new());
    }

    let num_slices = (total_len as f32 / SLICE_SIZE as f32).ceil() as usize;
    let mut slices = VecDeque::with_capacity(num_slices);

    let mut next = Some(file);
    while let Some(rest) = next {
        next = match rest.len() {
            // >1 chunk size left
            len if len > SLICE_SIZE => {
                let (slice, rest) = rest.split_at(SLICE_SIZE)?;

                slices.push_back(Ok(FileChunk(slice)));

                Some(rest)
            }
            // EOF
            0 => None,
            // Last chunk
            _ => {
                slices.push_back(Ok(FileChunk(rest)));

                None
            }
        };
    }

    Ok(slices)
}
