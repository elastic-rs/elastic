/*!
Response type parsing.
*/

use serde::de::DeserializeOwned;
use serde_json::{
    self,
    Value,
};
use std::{
    io::{
        Cursor,
        Read,
    },
    marker::PhantomData,
};

use super::error::*;

use crate::http::StatusCode;

/** A parser that separates taking a response type from the readable body type. */
pub struct Parse<T> {
    _marker: PhantomData<T>,
}

/**
Try parse a http response into a concrete type.

Parsing is split between two calls:

- `parse` where you can specify the response type
- `from_slice`/`from_reader` wher you can specify the kind of body to read from

The reason for splitting the functions is so we can infer the types of arguments to `from_slice` and `from_reader`,
but provide the concrete response type in cases it can't be inferred.

# Examples

Provide an explicit response type in the `parse` function:

```no_run
# use serde_json::Value;
# use elastic::http::{StatusCode, receiver::{parse, ParseError}};
# use elastic::client::responses::*;
# fn do_request() -> (StatusCode, Vec<u8>) { unimplemented!() }
# let (response_status, response_body) = do_request();
let get_response = parse::<GetResponse<Value>>().from_slice(response_status, response_body);
```

Provide an explicit response type on the result ident:

```no_run
# use serde_json::Value;
# use elastic::http::{StatusCode, receiver::{parse, ResponseError}};
# use elastic::client::responses::*;
# fn do_request() -> (StatusCode, Vec<u8>) { unimplemented!() }
# let (response_status, response_body) = do_request();
let get_response: Result<GetResponse<Value>, ResponseError> = parse().from_slice(response_status, response_body);
```

If Rust can infer the concrete response type then you can avoid specifying it at all:

```no_run
# use serde_json::Value;
# use elastic::http::{StatusCode, receiver::{parse, ResponseError}};
# use elastic::client::responses::*;
# fn do_request() -> (StatusCode, Vec<u8>) { unimplemented!() }
# fn parse_response() -> Result<GetResponse<Value>, ResponseError> {
# let (response_status, response_body) = do_request();
let get_response = parse().from_slice(response_status, response_body);
# get_response
# }
```
*/
pub fn parse<T: IsOk + DeserializeOwned>() -> Parse<T> {
    Parse {
        _marker: PhantomData,
    }
}

#[allow(clippy::wrong_self_convention)]
impl<T: IsOk + DeserializeOwned> Parse<T> {
    /** Try parse a contiguous slice of bytes into a concrete response. */
    pub fn from_slice<B: AsRef<[u8]>, H: Into<HttpResponseHead>>(
        self,
        head: H,
        body: B,
    ) -> Result<T, ResponseError> {
        from_body(head.into(), SliceBody(body))
    }

    /** Try parse an arbitrary reader into a concrete response. */
    pub fn from_reader<B: Read, H: Into<HttpResponseHead>>(
        self,
        head: H,
        body: B,
    ) -> Result<T, ResponseError> {
        from_body(head.into(), ReadBody(body))
    }
}

fn from_body<B: ResponseBody, T: IsOk + DeserializeOwned>(
    head: HttpResponseHead,
    body: B,
) -> Result<T, ResponseError> {
    let maybe = T::is_ok(head, Unbuffered(body))?;

    if maybe.ok {
        let ok = maybe.res.parse_ok()?;
        Ok(ok)
    } else {
        let err = maybe.res.parse_err()?;
        Err(ResponseError::Api(err))
    }
}

/** The non-body component of the HTTP response. */
pub struct HttpResponseHead {
    code: StatusCode,
}

impl HttpResponseHead {
    /** Get the status code. */
    pub fn status(&self) -> StatusCode {
        self.code
    }
}

impl From<StatusCode> for HttpResponseHead {
    fn from(status: StatusCode) -> Self {
        HttpResponseHead { code: status }
    }
}

/** A http response body that can be buffered into a json value. */
pub trait ResponseBody
where
    Self: Sized,
{
    /** The type of a buffered response body. */
    type Buffered: ResponseBody;

    /** Buffer the response body to a json value and return a new buffered representation. */
    fn body(self) -> Result<(Value, Self::Buffered), ParseError>;

    /** Parse the body as a success result. */
    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseError>;

    /** Parse the body as an API error. */
    fn parse_err(self) -> Result<ApiError, ParseError>;
}

struct ReadBody<B>(B);

impl<B: Read> ResponseBody for ReadBody<B> {
    type Buffered = SliceBody<Vec<u8>>;

    fn body(mut self) -> Result<(Value, Self::Buffered), ParseError> {
        let mut buf = Vec::new();
        self.0.read_to_end(&mut buf)?;

        let body: Value = serde_json::from_reader(Cursor::new(&buf))?;

        Ok((body, SliceBody(buf)))
    }

    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseError> {
        serde_json::from_reader(self.0).map_err(|e| e.into())
    }

    fn parse_err(self) -> Result<ApiError, ParseError> {
        match serde_json::from_reader(self.0)? {
            ParsedApiError::Known(err) => Ok(err),
            ParsedApiError::Unknown(err) => Err(ParseError::new(UnknownApiError(err))),
        }
    }
}

struct SliceBody<B>(B);

impl<B: AsRef<[u8]>> ResponseBody for SliceBody<B> {
    type Buffered = Self;

    fn body(self) -> Result<(Value, Self::Buffered), ParseError> {
        let buf = self.0;

        let body: Value = serde_json::from_slice(buf.as_ref())?;

        Ok((body, SliceBody(buf)))
    }

    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseError> {
        serde_json::from_slice(self.0.as_ref()).map_err(|e| e.into())
    }

    fn parse_err(self) -> Result<ApiError, ParseError> {
        match serde_json::from_slice(self.0.as_ref())? {
            ParsedApiError::Known(err) => Ok(err),
            ParsedApiError::Unknown(err) => Err(ParseError::new(UnknownApiError(err))),
        }
    }
}

impl ResponseBody for Value {
    type Buffered = Self;

    fn body(self) -> Result<(Value, Self::Buffered), ParseError> {
        let value = self.clone();

        Ok((self, value))
    }

    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseError> {
        serde_json::from_value(self).map_err(|e| e.into())
    }

    fn parse_err(self) -> Result<ApiError, ParseError> {
        match serde_json::from_value(self)? {
            ParsedApiError::Known(err) => Ok(err),
            ParsedApiError::Unknown(err) => Err(ParseError::new(UnknownApiError(err))),
        }
    }
}

/**
Convert a response message into a either a success
or failure result.

This is the main trait that drives response parsing by inspecting the http status and potentially
buffering the response to determine whether or not it represents a successful operation.
This trait doesn't do the actual deserialisation, it just passes on a `MaybeOkResponse`.

Some endpoints may not map success or error responses directly to a status code.
In this case, the `Unbuffered` body can be buffered into an anonymous json object and inspected
for an error node.
The `Unbuffered` type takes care of response bodies that can only be buffered once.

Any type that implements `IsOk` can be deserialised by `parse`.
Implementations should behave in the following way:

- If the response is successful, this trait should return `Ok(MaybeOkResponse::ok)`.
- If the response is an error, this trait should return `Ok(MaybeOkResponse::err)`.
- If the response isn't recognised or is otherwise invalid, this trait should return `Err`.

# Examples

Implement `IsOk` for a custom response type, where a http `404` might still contain a valid response:

```
# #[macro_use] extern crate serde_derive;
# use elastic::http::{StatusCode, receiver::{parse, IsOk, ResponseBody, HttpResponseHead, Unbuffered, MaybeOkResponse, ParseError}};
# #[derive(Deserialize)]
# struct MyResponse;
impl IsOk for MyResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, unbuffered: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseError> {
        match head.status() {
            status if status.is_success() => Ok(MaybeOkResponse::ok(unbuffered)),
            StatusCode::NOT_FOUND => {
                // If we get a 404, it could be an IndexNotFound error or ok
                // Check if the response contains a root 'error' node
                let (body, buffered) = unbuffered.body()?;

                let is_ok = body.as_object()
                    .and_then(|body| body.get("error"))
                    .is_none();

                Ok(MaybeOkResponse::new(is_ok, buffered))
            }
            _ => Ok(MaybeOkResponse::err(unbuffered)),
        }
    }
}
```
*/
pub trait IsOk {
    /** Inspect the http response to determine whether or not it succeeded. */
    fn is_ok<B: ResponseBody>(
        head: HttpResponseHead,
        unbuffered: Unbuffered<B>,
    ) -> Result<MaybeOkResponse<B>, ParseError>;
}

/**
A convenient trait that automatically derives `IsOk` if the status code is in the `200` range.
*/
pub trait IsOkOnSuccess {}

impl<T: IsOkOnSuccess> IsOk for T {
    fn is_ok<B: ResponseBody>(
        head: HttpResponseHead,
        body: Unbuffered<B>,
    ) -> Result<MaybeOkResponse<B>, ParseError> {
        if head.status().is_success() {
            Ok(MaybeOkResponse::ok(body))
        } else {
            Ok(MaybeOkResponse::err(body))
        }
    }
}

impl IsOkOnSuccess for Value {}

/** A response that might be successful or an `ApiError`. */
pub struct MaybeOkResponse<B>
where
    B: ResponseBody,
{
    ok: bool,
    res: MaybeBufferedResponse<B>,
}

impl<B> MaybeOkResponse<B>
where
    B: ResponseBody,
{
    /**
    Create a new response that indicates where or not the
    body is successful or an `ApiError`.
    */
    pub fn new<I>(ok: bool, res: I) -> Self
    where
        I: Into<MaybeBufferedResponse<B>>,
    {
        MaybeOkResponse {
            ok,
            res: res.into(),
        }
    }

    /** Create a response where the body is successful. */
    pub fn ok<I>(res: I) -> Self
    where
        I: Into<MaybeBufferedResponse<B>>,
    {
        Self::new(true, res)
    }

    /** Create a resposne where the body is an error. */
    pub fn err<I>(res: I) -> Self
    where
        I: Into<MaybeBufferedResponse<B>>,
    {
        Self::new(false, res)
    }
}

/** A response body that hasn't been buffered yet. */
pub struct Unbuffered<B>(B);

impl<B: ResponseBody> Unbuffered<B> {
    /** Buffer the response body to a json value and return a new buffered representation. */
    pub fn body(self) -> Result<(Value, Buffered<B>), ParseError> {
        self.0.body().map(|(value, body)| (value, Buffered(body)))
    }
}

/** A response body that has been buffered. */
pub struct Buffered<B: ResponseBody>(B::Buffered);

/**
A response body that may or may not have been buffered.

This type makes it possible to inspect the response body for
an error type before passing it along to be deserialised properly.
*/
pub enum MaybeBufferedResponse<B>
where
    B: ResponseBody,
{
    /**
    The response body has not been buffered.
    */
    Unbuffered(B),
    /**
    The response body has been buffered.
    */
    Buffered(B::Buffered),
    /**
    The response body has been deserialized.
    */
    Value(Value),
}

impl<B> MaybeBufferedResponse<B>
where
    B: ResponseBody,
{
    fn parse_ok<T: DeserializeOwned>(self) -> Result<T, ParseError> {
        match self {
            MaybeBufferedResponse::Unbuffered(b) => b.parse_ok(),
            MaybeBufferedResponse::Buffered(b) => b.parse_ok(),
            MaybeBufferedResponse::Value(b) => b.parse_ok(),
        }
    }

    fn parse_err(self) -> Result<ApiError, ParseError> {
        match self {
            MaybeBufferedResponse::Unbuffered(b) => b.parse_err(),
            MaybeBufferedResponse::Buffered(b) => b.parse_err(),
            MaybeBufferedResponse::Value(b) => b.parse_err(),
        }
    }
}

impl<B> From<Unbuffered<B>> for MaybeBufferedResponse<B>
where
    B: ResponseBody,
{
    fn from(value: Unbuffered<B>) -> Self {
        MaybeBufferedResponse::Unbuffered(value.0)
    }
}

impl<B> From<Buffered<B>> for MaybeBufferedResponse<B>
where
    B: ResponseBody,
{
    fn from(value: Buffered<B>) -> Self {
        MaybeBufferedResponse::Buffered(value.0)
    }
}

impl<B> From<Value> for MaybeBufferedResponse<B>
where
    B: ResponseBody,
{
    fn from(value: Value) -> Self {
        MaybeBufferedResponse::Value(value)
    }
}
