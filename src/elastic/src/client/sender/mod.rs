pub mod static_nodes;
pub mod sniffed_nodes;

mod sync;
mod async;
mod params;
pub use self::sync::*;
pub use self::async::*;
pub use self::params::*;

use std::marker::PhantomData;

use client::requests::HttpRequest;
use private;

/** A sendable request. */
pub struct SendableRequest<TRequest, TBody> {
    inner: TRequest,
    params_builder: Option<Box<Fn(RequestParams) -> RequestParams>>,
    _marker: PhantomData<TBody>,
}

impl<TRequest, TBody> SendableRequest<TRequest, TBody> {
    fn new(inner: TRequest, params_builder: Option<Box<Fn(RequestParams) -> RequestParams>>) -> Self {
        SendableRequest {
            inner: inner,
            params_builder: params_builder,
            _marker: PhantomData,
        }
    }
}

/**
Represents a type that can send a request.

You probably don't need to touch this trait directly.
See the [`Client`][Client] type for making requests.

[Client]: struct.Client.html
*/
pub trait Sender: private::Sealed + Clone {
    /// The kind of request body this sender accepts.
    type Body;
    /// The kind of response this sender produces.
    type Response;

    /// Send a request.
    fn send<TRequest, TBody>(&self, request: SendableRequest<TRequest, TBody>) -> Self::Response
    where
        TRequest: Into<HttpRequest<'static, TBody>>,
        TBody: Into<Self::Body> + 'static;
}
