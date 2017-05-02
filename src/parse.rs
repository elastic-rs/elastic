use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{self, Value};

use std::io::{Cursor, Read, Result as IoResult};

use error::*;
use self::private::{ParseResponse};

pub type ApiResult<T> = Result<T, ResponseError>;

/// The non-body component of the http response.
pub struct HttpResponseHead {
    code: u16,
}

impl HttpResponseHead {
    pub fn status(&self) -> u16 {
        self.code
    }
}

/// A raw HTTP response with enough information to parse
/// a concrete type from it.
pub struct HttpResponse<B> {
    head: HttpResponseHead,
    body: B,
}

impl<B> HttpResponse<B> {
    /// Create a new HTTP response from the given status code
    /// and body.
    fn new(status: u16, body: B) -> Self {
        HttpResponse {
            head: HttpResponseHead {
                code: status,
            },
            body: body,
        }
    }

    /// Get the status code.
    pub fn status(&self) -> u16 {
        self.head.code
    }
}

impl<B: AsRef<[u8]>> AsRef<[u8]> for HttpResponse<B> {
    fn as_ref(&self) -> &[u8] {
        self.body.as_ref()
    }
}

impl<B: Read> Read for HttpResponse<B> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.body.read(buf)
    }
}

impl<B: AsRef<[u8]>> HttpResponse<SliceBody<B>> {
    /// Build a http response from a contiguous slice.
    pub fn from_slice(status: u16, body: B) -> Self {
        Self::new(status, SliceBody(body))
    }
}

impl<B: Read> HttpResponse<ReadBody<B>> {
    /// Build a http response from a byte reader.
    pub fn from_read(status: u16, body: B) -> Self {
        Self::new(status, ReadBody(body))
    }
}


impl<B: ResponseBody + private::ParseResponse> HttpResponse<B> {
    /// Convert this http response into either `Ok(T)` or an `Err(ApiError)`.
    pub fn into_response<T: IsOk + DeserializeOwned>(self) -> Result<T, ResponseError> {
        let maybe = T::is_ok(self.head, self.body)?;

        match maybe.ok {
            true => {
                let ok = maybe.res.parse_ok()?;
                Ok(ok)
            }
            false => {
                let err = maybe.res.parse_err()?;
                Err(ResponseError::Api(err))
            }
        }
    }
}

/// A http response body that can be buffered into a json value.
pub trait ResponseBody: private::ParseResponse + private::Sealed 
    where Self: Sized
{
    /// The type of a buffered response body.
    type Buffered: private::ParseResponse;

    /// Buffer the response body to a json value and return a new buffered representation.
    fn body(self) -> Result<(Value, private::Buffered<Self>), ParseResponseError>;
}

/// A http response body that implements `Read`.
pub struct ReadBody<B>(B);

impl<B> private::Sealed for ReadBody<B> {}

impl<B: Read> ResponseBody for ReadBody<B> {
    type Buffered = ReadBody<Cursor<Vec<u8>>>;

    fn body(mut self) -> Result<(Value, private::Buffered<Self>), ParseResponseError> {
        let mut buf = Vec::new();
        self.0.read_to_end(&mut buf)?;

        let body: Value = serde_json::from_reader(Cursor::new(&buf))?;

        Ok((body, private::Buffered(ReadBody(Cursor::new(buf)))))
    }
}

impl<B: Read> private::ParseResponse for ReadBody<B> {
    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseResponseError> {
        serde_json::from_reader(self.0).map_err(|e| e.into())
    }

    fn parse_err(self) -> Result<ApiError, ParseResponseError> {
        serde_json::from_reader(self.0).map_err(|e| e.into())
    }
}

/// A http response body that implements `AsRef<[u8]>`
pub struct SliceBody<B>(B);

impl<B> private::Sealed for SliceBody<B> {}

impl<B: AsRef<[u8]>> ResponseBody for SliceBody<B> {
    type Buffered = Self;

    fn body(self) -> Result<(Value, private::Buffered<Self>), ParseResponseError> {
        let buf = self.0;

        let body: Value = serde_json::from_slice(buf.as_ref())?;

        Ok((body, private::Buffered(SliceBody(buf))))
    }
}

impl<B: AsRef<[u8]>> private::ParseResponse for SliceBody<B> {
    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseResponseError> {
        serde_json::from_slice(self.0.as_ref()).map_err(|e| e.into())
    }

    fn parse_err(self) -> Result<ApiError, ParseResponseError> {
        serde_json::from_slice(self.0.as_ref()).map_err(|e| e.into())
    }
}

/// Convert a response message into a either a success
/// or failure result.
pub trait IsOk
{
    /// Inspect the http response to determine whether or not it succeeded.
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: B) -> Result<MaybeOkResponse<B>, ParseResponseError>;
}

impl IsOk for Value {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: B) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}

/// A response that might be successful or an `ApiError`.
pub struct MaybeOkResponse<B> 
    where B: ResponseBody + private::ParseResponse
{
    ok: bool,
    res: MaybeBufferedResponse<B>,
}

impl<B> MaybeOkResponse<B> 
    where B: ResponseBody + private::ParseResponse
{
    /// Create a new response that indicates where or not the
    /// body is successful or an `ApiError`.
    pub fn new<I>(ok: bool, res: I) -> Self
        where I: Into<MaybeBufferedResponse<B>>
    {
        MaybeOkResponse {
            ok: ok,
            res: res.into(),
        }
    }

    /// Create a response where the body is successful.
    pub fn ok<I>(res: I) -> Self
        where I: Into<MaybeBufferedResponse<B>>
    {
        Self::new(true, res)
    }

    /// Create a resposne where the body is an error.
    pub fn err<I>(res: I) -> Self
        where I: Into<MaybeBufferedResponse<B>>
    {
        Self::new(false, res)
    }
}

/// A response body that may or may not have been buffered.
///
/// This type makes it possible to inspect the response body for
/// an error type before passing it along to be deserialised properly.
pub enum MaybeBufferedResponse<B>
    where B: ResponseBody + private::ParseResponse
{
    Unbuffered(B),
    Buffered(B::Buffered),
}

impl<B> private::ParseResponse for MaybeBufferedResponse<B>
    where B: ResponseBody + private::ParseResponse
{
    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseResponseError> {
        match self {
            MaybeBufferedResponse::Unbuffered(b) => b.parse_ok(),
            MaybeBufferedResponse::Buffered(b) => b.parse_ok()
        }
    }

    fn parse_err(self) -> Result<ApiError, ParseResponseError> {
        match self {
            MaybeBufferedResponse::Unbuffered(b) => b.parse_err(),
            MaybeBufferedResponse::Buffered(b) => b.parse_err()
        }
    }
}

impl<B> From<B> for MaybeBufferedResponse<B>
    where B: ResponseBody + private::ParseResponse
{
    fn from(value: B) -> Self {
        MaybeBufferedResponse::Unbuffered(value)
    }
}

impl<B> From<private::Buffered<B>> for MaybeBufferedResponse<B>
    where B: ResponseBody + private::ParseResponse
{
    fn from(value: private::Buffered<B>) -> Self {
        MaybeBufferedResponse::Buffered(value.0)
    }
}

mod private {
    use serde::de::DeserializeOwned;
    use error::{ApiError, ParseResponseError};
    use super::*;

    pub trait Sealed {}

    pub trait ParseResponse {
        fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseResponseError>;
        fn parse_err(self) -> Result<ApiError, ParseResponseError>;
    }

    pub struct Buffered<B: ResponseBody + private::ParseResponse>(pub B::Buffered);
}
