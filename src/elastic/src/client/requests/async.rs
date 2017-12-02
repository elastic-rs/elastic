use bytes::Bytes;
use serde_json::Value;
use std::borrow::Cow;
use std::io::{self, Read, Cursor};

use tokio_io::AsyncRead;
use http::{HttpRequest, AsyncBody as Body};

pub type AsyncHttpRequest = HttpRequest<AsyncBody>;

/** A type that can be converted into a request body. */
pub struct AsyncBody(AsyncBodyInner);

enum AsyncBodyInner {
    Shared(Bytes),
    Bytes(Cow<'static, [u8]>),
    Str(Cow<'static, str>),
}

impl AsRef<[u8]> for AsyncBodyInner {
    fn as_ref(&self) -> &[u8] {
        match *self {
            AsyncBodyInner::Shared(ref bytes) => bytes.as_ref(),
            AsyncBodyInner::Bytes(ref bytes) => bytes.as_ref(),
            AsyncBodyInner::Str(ref string) => string.as_bytes()
        }
    }
}

/**
A read adapter for an `AsyncBody`.
*/
pub struct AsyncBodyReader<'a> {
    inner: Cursor<&'a AsyncBodyInner>,
}

impl<'a> Read for AsyncBodyReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.inner.read(buf)
    }
}

impl<'a> AsyncRead for AsyncBodyReader<'a> {
    
}

impl AsyncBody {
    /** Convert the body into its inner value. */
    pub(crate) fn into_inner(self) -> Body {
        match self.0 {
            AsyncBodyInner::Shared(bytes) => bytes.into(),
            AsyncBodyInner::Bytes(bytes) => match bytes {
                Cow::Owned(bytes) => bytes.into(),
                Cow::Borrowed(bytes) => bytes.into(),
            },
            AsyncBodyInner::Str(string) => match string {
                Cow::Owned(string) => string.into(),
                Cow::Borrowed(string) => string.into(),
            }
        }
    }
}

impl From<Bytes> for AsyncBody {
    fn from(body: Bytes) -> AsyncBody {
        AsyncBody(AsyncBodyInner::Shared(body))
    }
}

impl From<Vec<u8>> for AsyncBody {
    fn from(body: Vec<u8>) -> AsyncBody {
        AsyncBody(AsyncBodyInner::Bytes(body.into()))
    }
}

impl From<String> for AsyncBody {
    fn from(body: String) -> AsyncBody {
        AsyncBody(AsyncBodyInner::Str(body.into()))
    }
}

impl From<Value> for AsyncBody {
    fn from(body: Value) -> AsyncBody {
        AsyncBody(AsyncBodyInner::Str(body.to_string().into()))
    }
}

impl From<&'static [u8]> for AsyncBody {
    fn from(body: &'static [u8]) -> AsyncBody {
        AsyncBody(AsyncBodyInner::Bytes(body.into()))
    }
}

impl From<&'static str> for AsyncBody {
    fn from(body: &'static str) -> AsyncBody {
        AsyncBody(AsyncBodyInner::Str(body.into()))
    }
}
