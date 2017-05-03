use serde::de::DeserializeOwned;
use serde_json::{self, Value};

use std::io::{Cursor, Read};

use super::{HttpResponse, HttpResponseHead, SliceBody, ReadBody};
use error::*;

impl<B: ResponseBody> HttpResponse<B> {
    /// Convert this http response into either `Ok(T)` or an `Err(ApiError)`.
    pub fn into_response<T: IsOk + DeserializeOwned>(self) -> Result<T, ResponseError> {
        let maybe = T::is_ok(self.head, Unbuffered(self.body))?;

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
pub trait ResponseBody where Self: Sized
{
    /// The type of a buffered response body.
    type Buffered: ResponseBody;

    /// Buffer the response body to a json value and return a new buffered representation.
    fn body(self) -> Result<(Value, Self::Buffered), ParseResponseError>;

    /// Parse the body as a success result.
    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseResponseError>;

    /// Parse the body as an API error.
    fn parse_err(self) -> Result<ApiError, ParseResponseError>;
}

impl<B: Read> ResponseBody for ReadBody<B> {
    type Buffered = ReadBody<Cursor<Vec<u8>>>;

    fn body(mut self) -> Result<(Value, Self::Buffered), ParseResponseError> {
        let mut buf = Vec::new();
        self.0.read_to_end(&mut buf)?;

        let body: Value = serde_json::from_reader(Cursor::new(&buf))?;

        Ok((body, ReadBody(Cursor::new(buf))))
    }

    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseResponseError> {
        serde_json::from_reader(self.0).map_err(|e| e.into())
    }

    fn parse_err(self) -> Result<ApiError, ParseResponseError> {
        serde_json::from_reader(self.0).map_err(|e| e.into())
    }
}

impl<B: AsRef<[u8]>> ResponseBody for SliceBody<B> {
    type Buffered = Self;

    fn body(self) -> Result<(Value, Self::Buffered), ParseResponseError> {
        let buf = self.0;

        let body: Value = serde_json::from_slice(buf.as_ref())?;

        Ok((body, SliceBody(buf)))
    }

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
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError>;
}

impl IsOk for Value {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}

/// A response that might be successful or an `ApiError`.
pub struct MaybeOkResponse<B> 
    where B: ResponseBody
{
    ok: bool,
    res: MaybeBufferedResponse<B>,
}

impl<B> MaybeOkResponse<B> where B: ResponseBody
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

pub struct Unbuffered<B>(B);

impl<B: ResponseBody> Unbuffered<B> {
    /// Buffer the response body to a json value and return a new buffered representation.
    pub fn body(self) -> Result<(Value, Buffered<B>), ParseResponseError> {
        self.0.body().map(|(value, body)| (value, Buffered(body)))
    }
}

pub struct Buffered<B: ResponseBody>(B::Buffered);

/// A response body that may or may not have been buffered.
///
/// This type makes it possible to inspect the response body for
/// an error type before passing it along to be deserialised properly.
pub enum MaybeBufferedResponse<B>
    where B: ResponseBody
{
    Unbuffered(B),
    Buffered(B::Buffered),
}

impl<B> MaybeBufferedResponse<B>
    where B: ResponseBody
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

impl<B> From<Unbuffered<B>> for MaybeBufferedResponse<B>
    where B: ResponseBody
{
    fn from(value: Unbuffered<B>) -> Self {
        MaybeBufferedResponse::Unbuffered(value.0)
    }
}

impl<B> From<Buffered<B>> for MaybeBufferedResponse<B>
    where B: ResponseBody
{
    fn from(value: Buffered<B>) -> Self {
        MaybeBufferedResponse::Buffered(value.0)
    }
}
