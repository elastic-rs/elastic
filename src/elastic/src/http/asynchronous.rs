use bytes::Bytes;
use serde_json::Value;
use std::{
    borrow::Cow,
    io::{
        self,
        Cursor,
        Read,
    },
};

use futures::{
    Poll,
    Stream,
};
use reqwest::r#async::{
    Body,
    Response as RawResponse,
};
use tokio::io::AsyncRead;

use crate::{
    error::{
        self,
        Error,
    },
    http::{
        HttpRequest,
        StatusCode,
    },
};

pub use reqwest::r#async::Chunk as AsyncChunk;

/** A http request with an asynchronous body; */
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
            AsyncBodyInner::Str(ref string) => string.as_bytes(),
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

impl<'a> AsyncRead for AsyncBodyReader<'a> {}

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
            },
        }
    }

    /**
    Get a reader over the asynchronous body.
    */
    pub fn reader(&mut self) -> AsyncBodyReader {
        AsyncBodyReader {
            inner: Cursor::new(&self.0),
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

/** A raw HTTP response that can be buffered using `Read`. */
pub struct AsyncHttpResponse(StatusCode, RawResponse);

impl AsyncHttpResponse {
    pub(crate) fn from_raw(status: StatusCode, response: RawResponse) -> Self {
        AsyncHttpResponse(status, response)
    }
}

impl Stream for AsyncHttpResponse {
    type Item = AsyncChunk;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.1
            .body_mut()
            .poll()
            .map_err(|e| error::response(self.0, e))
    }
}

impl AsyncHttpResponse {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> StatusCode {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::empty_body;

    #[test]
    fn owned_string_into_body() {
        AsyncBody::from(String::new());
    }

    #[test]
    fn borrowed_string_into_body() {
        AsyncBody::from("abc");
    }

    #[test]
    fn owned_vec_into_body() {
        AsyncBody::from(Vec::new());
    }

    #[test]
    fn borrowed_vec_into_body() {
        static BODY: &[u8] = &[0, 1, 2];

        AsyncBody::from(BODY);
    }

    #[test]
    fn empty_body_into_body() {
        AsyncBody::from(empty_body());
    }

    #[test]
    fn json_value_into_body() {
        AsyncBody::from(json!({}));
    }
}
