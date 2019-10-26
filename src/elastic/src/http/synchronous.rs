use bytes::Bytes;
use serde_json::Value;
use std::{
    borrow::Cow,
    fs::File,
    io::{
        self,
        Cursor,
        Read,
    },
};

use reqwest::{
    Body,
    Response as RawResponse,
};

use crate::http::{
    HttpRequest,
    StatusCode,
};

/** A http request with a synchronous body. */
pub type SyncHttpRequest = HttpRequest<SyncBody>;

/** A type that can be converted into a request body. */
pub struct SyncBody(SyncBodyInner);

enum SyncBodyInner {
    UnBuffered(Box<dyn Read + Send>),
    Buffered(BufferedSyncBodyInner<'static>),
}

#[derive(Clone)]
enum BufferedSyncBodyInner<'a> {
    Shared(Bytes),
    Bytes(Cow<'a, [u8]>),
    Str(Cow<'a, str>),
}

impl<'a> AsRef<[u8]> for BufferedSyncBodyInner<'a> {
    fn as_ref(&self) -> &[u8] {
        match *self {
            BufferedSyncBodyInner::Shared(ref bytes) => bytes.as_ref(),
            BufferedSyncBodyInner::Bytes(ref bytes) => bytes.as_ref(),
            BufferedSyncBodyInner::Str(ref string) => string.as_bytes(),
        }
    }
}

impl<'a> BufferedSyncBodyInner<'a> {
    fn as_ref(&self) -> BufferedSyncBodyInner {
        match *self {
            BufferedSyncBodyInner::Shared(ref bytes) => {
                BufferedSyncBodyInner::Shared(bytes.clone())
            }
            BufferedSyncBodyInner::Bytes(ref bytes) => {
                BufferedSyncBodyInner::Bytes(Cow::Borrowed(bytes))
            }
            BufferedSyncBodyInner::Str(ref string) => {
                BufferedSyncBodyInner::Str(Cow::Borrowed(string))
            }
        }
    }
}

impl SyncBody {
    /** Convert the body into its inner value. */
    pub(crate) fn into_inner(self) -> Body {
        match self.0 {
            SyncBodyInner::UnBuffered(reader) => Body::new(reader),
            SyncBodyInner::Buffered(BufferedSyncBodyInner::Shared(bytes)) => bytes.to_vec().into(),
            SyncBodyInner::Buffered(BufferedSyncBodyInner::Bytes(bytes)) => match bytes {
                Cow::Owned(bytes) => bytes.into(),
                Cow::Borrowed(bytes) => bytes.into(),
            },
            SyncBodyInner::Buffered(BufferedSyncBodyInner::Str(string)) => match string {
                Cow::Owned(string) => string.into(),
                Cow::Borrowed(string) => string.into(),
            },
        }
    }

    /**
    Get a reader over the synchronous body.

    If the body can only be read once then it will be buffered first so subsequent reads won't fail.
    */
    pub fn reader(&mut self) -> SyncBodyReader {
        let state = match self.0 {
            SyncBodyInner::UnBuffered(ref mut reader) => SyncBodyReaderState::UnBuffered(reader),
            SyncBodyInner::Buffered(ref inner) => {
                SyncBodyReaderState::Buffered(Cursor::new(inner.as_ref()))
            }
        };

        SyncBodyReader { state }
    }
}

/**
A read adapter for a `SyncBody`.
*/
pub struct SyncBodyReader<'a> {
    state: SyncBodyReaderState<'a>,
}

enum SyncBodyReaderState<'a> {
    #[allow(clippy::borrowed_box)]
    UnBuffered(&'a mut Box<dyn Read + Send>),
    Buffered(Cursor<BufferedSyncBodyInner<'a>>),
}

impl<'a> SyncBodyReaderState<'a> {
    fn buffer(&mut self) -> Result<&mut Cursor<BufferedSyncBodyInner<'a>>, io::Error> {
        let new_buf = match *self {
            SyncBodyReaderState::Buffered(ref mut buf) => return Ok(buf),
            SyncBodyReaderState::UnBuffered(ref mut reader) => {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;

                SyncBodyReaderState::Buffered(Cursor::new(BufferedSyncBodyInner::Bytes(
                    Cow::Owned(buf),
                )))
            }
        };

        *self = new_buf;

        self.buffer()
    }
}

impl<'a> Read for SyncBodyReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let reader = self.state.buffer()?;

        reader.read(buf)
    }
}

impl From<Bytes> for SyncBody {
    fn from(body: Bytes) -> SyncBody {
        SyncBody(SyncBodyInner::Buffered(BufferedSyncBodyInner::Shared(body)))
    }
}

impl From<Vec<u8>> for SyncBody {
    fn from(body: Vec<u8>) -> SyncBody {
        SyncBody(SyncBodyInner::Buffered(BufferedSyncBodyInner::Bytes(
            body.into(),
        )))
    }
}

impl From<String> for SyncBody {
    fn from(body: String) -> SyncBody {
        SyncBody(SyncBodyInner::Buffered(BufferedSyncBodyInner::Str(
            body.into(),
        )))
    }
}

impl From<Value> for SyncBody {
    fn from(body: Value) -> SyncBody {
        SyncBody(SyncBodyInner::Buffered(BufferedSyncBodyInner::Str(
            body.to_string().into(),
        )))
    }
}

impl From<&'static [u8]> for SyncBody {
    fn from(body: &'static [u8]) -> SyncBody {
        SyncBody(SyncBodyInner::Buffered(BufferedSyncBodyInner::Bytes(
            body.into(),
        )))
    }
}

impl From<&'static str> for SyncBody {
    fn from(body: &'static str) -> SyncBody {
        SyncBody(SyncBodyInner::Buffered(BufferedSyncBodyInner::Str(
            body.into(),
        )))
    }
}

impl From<File> for SyncBody {
    fn from(body: File) -> SyncBody {
        SyncBody(SyncBodyInner::UnBuffered(Box::new(body)))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::empty_body;

    #[test]
    fn owned_string_into_body() {
        SyncBody::from(String::new());
    }

    #[test]
    fn borrowed_string_into_body() {
        SyncBody::from("abc");
    }

    #[test]
    fn owned_vec_into_body() {
        SyncBody::from(Vec::new());
    }

    #[test]
    fn borrowed_vec_into_body() {
        static BODY: &[u8] = &[0, 1, 2];

        SyncBody::from(BODY);
    }

    #[test]
    fn empty_body_into_body() {
        SyncBody::from(empty_body());
    }

    #[test]
    fn json_value_into_body() {
        SyncBody::from(json!({}));
    }
}
