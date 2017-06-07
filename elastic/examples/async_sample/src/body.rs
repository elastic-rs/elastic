use std::cmp;
use std::collections::VecDeque;
use std::path::Path;
use std::io::{Read, Error as IoError};

use tokio_proto::streaming::Body as BodyStream;
use futures::{IntoFuture, Future, Stream, Sink, Poll, Async};
use futures::future::lazy;
use futures::sync::mpsc::SendError;
use memmap::{Mmap, MmapViewSync, Protection};
use hyper::{Chunk as HyperChunk, Error as HyperError};

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
     -> Result<(impl Future<Item = (), Error = Error> + Send, FileBody), IoError>
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

    Ok((tx_future, rx))
}

// mmap a file and push its chunks into a queue
fn map_file_to_chunks<P>(path: P) -> Result<VecDeque<FileChunkResult>, Error>
    where P: AsRef<Path>
{
    let file = match Mmap::open_path(path, Protection::Read) {
        Ok(file) => file,
        Err(e) => return Err(e.into()),
    };

    let file = file.into_view_sync();

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
                let (slice, rest) = match rest.split_at(SLICE_SIZE) {
                    Ok(split) => split,
                    Err(e) => return Err(e.into()),
                };

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

/// A readable wrapper around a `Chunk`.
///
/// This type is basically the same as `std::io::Cursor`.
pub struct Chunk {
    buf: HyperChunk,
    len: usize,
    pos: usize,
}

impl From<HyperChunk> for Chunk {
    fn from(chunk: HyperChunk) -> Self {
        Chunk {
            len: chunk.len(),
            pos: 0,
            buf: chunk,
        }
    }
}

impl Read for Chunk {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let amt = cmp::min(self.pos, self.len);
        let mut chunk = &self.buf[amt..];

        let read = chunk.read(buf)?;
        self.pos += read;

        Ok(read)
    }
}

type ChunkSequence = VecDeque<Chunk>;

/// A builder for a sequence of chunks.
pub struct ChunkBodyBuilder(ChunkSequence);

/// A readable response body where bytes are read from all chunks without exposing them directly.
pub struct ChunkBody(ChunkSequence);

impl ChunkBodyBuilder {
    pub fn new() -> Self {
        ChunkBodyBuilder(ChunkSequence::new())
    }

    pub fn append<I>(&mut self, chunk: I)
        where I: Into<Chunk>
    {
        self.0.push_back(chunk.into());
    }

    pub fn build(self) -> ChunkBody {
        let mut chunks = self.0;
        chunks.shrink_to_fit();

        ChunkBody(chunks)
    }
}

impl Read for ChunkBody {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        let mut pop = false;

        let read = if let Some(mut chunk) = self.0.front_mut() {
            let read = chunk.read(buf)?;

            if chunk.pos >= chunk.len {
                pop = true;
            }

            read
        } else {
            0
        };

        if pop {
            self.0.pop_front();
        }

        Ok(read)
    }
}
