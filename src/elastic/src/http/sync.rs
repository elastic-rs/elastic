use bytes::Bytes;
use serde_json::Value;
use std::borrow::Cow;
use std::io::{self, Read, Cursor};

use reqwest::{Body, Response as RawResponse};

use http::{HttpRequest, StatusCode};

/** A http request with a synchronous body. */
pub type SyncHttpRequest = HttpRequest<SyncBody>;

/** A type that can be converted into a request body. */
pub struct SyncBody(SyncBodyInner);

// TODO: Add streamed that builds a set of in-memory chunks when being read

enum SyncBodyInner {
    Shared(Bytes),
    Bytes(Cow<'static, [u8]>),
    Str(Cow<'static, str>),
}

impl AsRef<[u8]> for SyncBodyInner {
    fn as_ref(&self) -> &[u8] {
        match *self {
            SyncBodyInner::Shared(ref bytes) => bytes.as_ref(),
            SyncBodyInner::Bytes(ref bytes) => bytes.as_ref(),
            SyncBodyInner::Str(ref string) => string.as_bytes()
        }
    }
}

/**
A read adapter for a `SyncBody`.
*/
pub struct SyncBodyReader<'a> {
    inner: Cursor<&'a SyncBodyInner>,
}

impl<'a> Read for SyncBodyReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.inner.read(buf)
    }
}

impl SyncBody {
    /** Convert the body into its inner value. */
    pub fn into_inner(self) -> Body {
        match self.0 {
            SyncBodyInner::Shared(bytes) => bytes.to_vec().into(),
            SyncBodyInner::Bytes(bytes) => match bytes {
                Cow::Owned(bytes) => bytes.into(),
                Cow::Borrowed(bytes) => bytes.into(),
            },
            SyncBodyInner::Str(string) => match string {
                Cow::Owned(string) => string.into(),
                Cow::Borrowed(string) => string.into(),
            }
        }
    }
}

impl From<Bytes> for SyncBody {
    fn from(body: Bytes) -> SyncBody {
        SyncBody(SyncBodyInner::Shared(body))
    }
}

impl From<Vec<u8>> for SyncBody {
    fn from(body: Vec<u8>) -> SyncBody {
        SyncBody(SyncBodyInner::Bytes(body.into()))
    }
}

impl From<String> for SyncBody {
    fn from(body: String) -> SyncBody {
        SyncBody(SyncBodyInner::Str(body.into()))
    }
}

impl From<Value> for SyncBody {
    fn from(body: Value) -> SyncBody {
        SyncBody(SyncBodyInner::Str(body.to_string().into()))
    }
}

impl From<&'static [u8]> for SyncBody {
    fn from(body: &'static [u8]) -> SyncBody {
        SyncBody(SyncBodyInner::Bytes(body.into()))
    }
}

impl From<&'static str> for SyncBody {
    fn from(body: &'static str) -> SyncBody {
        SyncBody(SyncBodyInner::Str(body.into()))
    }
}

/** A raw HTTP response that can be buffered using `Read`. */
pub struct SyncHttpResponse(StatusCode, RawResponse);

impl SyncHttpResponse {
    pub(crate) fn from_raw(status: StatusCode, response: RawResponse) -> Self {
        SyncHttpResponse(status, response)
    }

    /** Get the HTTP status for the response. */
    pub fn status(&self) -> StatusCode {
        self.0
    }
}

impl Read for SyncHttpResponse {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.1.read(buf)
    }
}
