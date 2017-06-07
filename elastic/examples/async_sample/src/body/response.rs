use std::cmp;
use std::collections::VecDeque;
use std::io::{Read, Error as IoError};

use futures::{Sink, Poll, Async};
use hyper::Chunk as HyperChunk;

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
