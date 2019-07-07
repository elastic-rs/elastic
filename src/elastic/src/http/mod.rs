/*!
Raw HTTP modules.

These types are lower-level details for sending requests and receiving
responses.
*/

mod asynchronous;
mod synchronous;

pub use self::{
    asynchronous::*,
    synchronous::*,
};

pub mod receiver;
pub mod sender;

#[doc(inline)]
pub use reqwest::{
    header,
    Url,
};

#[doc(inline)]
pub use http::{
    Method,
    StatusCode,
};

#[doc(inline)]
pub use crate::genned::http::{
    empty_body,
    DefaultBody,
    UrlPath,
};

use std::{
    fmt,
    sync::Arc,
};

use self::header::HeaderMap;

/**
A request just before being sent.
*/
#[derive(Clone)]
pub struct HttpRequest<TBody> {
    pub(crate) url: Url,
    pub(crate) method: Method,
    pub(crate) headers: Arc<HeaderMap>,
    pub(crate) body: Option<TBody>,
}

impl<TBody> HttpRequest<TBody> {
    /**
    Get a mutable reference to the request url.
    */
    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    /**
    Get a mutable reference to the request method.
    */
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    /**
    Get a mutable reference to the request headers.
    */
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        Arc::make_mut(&mut self.headers)
    }

    /**
    Get a mutable reference to the request body.
    */
    pub fn body_mut(&mut self) -> Option<&mut TBody> {
        self.body.as_mut()
    }
}

impl<TBody> fmt::Debug for HttpRequest<TBody> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HttpRequest")
            .field("url", &self.url)
            .field("method", &self.method)
            .field("headers", &self.headers)
            .field("body", &self.body.as_ref().map(|_| ()))
            .finish()
    }
}
