use serde::Deserialize;
use serde_json::{self, Value, Error as JsonError};

use std::io::{Cursor, Read};

use error::*;
use super::{HttpResponse, ApiResult};

macro_rules! read_ok {
    ($buf:expr) => (serde_json::from_reader($buf).map_err(|e| e.into()))
}

macro_rules! read_err {
    ($buf:expr) => ({
        let err: ApiError = serde_json::from_reader($buf)?;
        Err(err.into())
    })
}

impl<R: Read> HttpResponse<R> {
    /// Get the response body from JSON.
    ///
    /// This method takes a closure that determines
    /// whether the result is successful.
    /// If the `MaybeOkResponse` is ok, then the body will be returned as `Ok(T)`.
    /// Otherwise the body will be returned as `Err(ApiError)`.
    pub fn response<T, F>(self, is_ok: F) -> ApiResult<T>
        where T: Deserialize,
              F: Fn(UnbufferedResponse<R>) -> Result<MaybeOkResponse<R>, JsonError>
    {
        let maybe = is_ok(UnbufferedResponse(self))?;

        match maybe.ok {
            true => {
                match maybe.res {
                    MaybeBufferedResponse::Buffered(b) => read_ok!(b.0.body),
                    MaybeBufferedResponse::Unbuffered(b) => read_ok!(b.0.body),
                }
            }
            false => {
                match maybe.res {
                    MaybeBufferedResponse::Buffered(b) => read_err!(b.0.body),
                    MaybeBufferedResponse::Unbuffered(b) => read_err!(b.0.body),
                }
            }
        }
    }
}

/// A response that might be successful or an `ApiError`.
pub struct MaybeOkResponse<R> {
    ok: bool,
    res: MaybeBufferedResponse<R>,
}

impl<R> MaybeOkResponse<R> {
    /// Create a new response that indicates where or not the
    /// body is successful or an `ApiError`.
    pub fn new<I>(ok: bool, res: I) -> Self
        where I: Into<MaybeBufferedResponse<R>>
    {
        MaybeOkResponse {
            ok: ok,
            res: res.into(),
        }
    }

    /// Create a response where the body is successful.
    pub fn ok<I>(res: I) -> Self
        where I: Into<MaybeBufferedResponse<R>>
    {
        Self::new(true, res)
    }

    /// Create a resposne where the body is an error.
    pub fn err<I>(res: I) -> Self
        where I: Into<MaybeBufferedResponse<R>>
    {
        Self::new(false, res)
    }
}

/// A response body that may or may not have been buffered.
///
/// This type makes it possible to inspect the response body for
/// an error type before passing it along to be deserialised properly.
pub enum MaybeBufferedResponse<R> {
    Unbuffered(UnbufferedResponse<R>),
    Buffered(BufferedResponse),
}

impl<R> From<UnbufferedResponse<R>> for MaybeBufferedResponse<R> {
    fn from(value: UnbufferedResponse<R>) -> Self {
        MaybeBufferedResponse::Unbuffered(value)
    }
}

impl<R> From<BufferedResponse> for MaybeBufferedResponse<R> {
    fn from(value: BufferedResponse) -> Self {
        MaybeBufferedResponse::Buffered(value)
    }
}

/// An untouched response body.
pub struct UnbufferedResponse<R>(HttpResponse<R>);

impl<R: Read> UnbufferedResponse<R> {
    /// Get the HTTP status code for the response.
    pub fn status(&self) -> u16 {
        self.0.status()
    }

    /// Buffer the response body into a `serde_json::Value` and return
    /// a `BufferedResponse`.
    ///
    /// This is _expensive_ so you should avoid using it if it's not
    /// necessary.
    pub fn body(mut self) -> Result<(Value, BufferedResponse), JsonError> {
        let status = self.status();

        let mut buf = Vec::new();
        self.0.body.read_to_end(&mut buf)?;

        let body: Value = serde_json::from_reader(Cursor::new(&buf))?;

        Ok((body, BufferedResponse(HttpResponse::new(status, Cursor::new(buf)))))
    }
}

/// A previously buffered response body.
pub struct BufferedResponse(HttpResponse<Cursor<Vec<u8>>>);

impl BufferedResponse {
    /// Get the HTTP status code for the response.
    pub fn status(&self) -> u16 {
        self.0.status()
    }
}
