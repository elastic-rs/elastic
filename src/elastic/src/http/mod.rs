/*!
Raw HTTP modules.

These types are re-exported from `reqwest` and used in parts of `elastic`s public API.
They may eventually be wrapped and made implementation details.
*/

mod async;
mod sync;

pub use self::{
    async::*,
    sync::*,
};

pub use reqwest::{
    header,
    Url,
};

pub use elastic_requests::Method;
pub use elastic_responses::StatusCode;

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
    pub fn url_mut(&mut self) -> &mut Url {
        &mut self.url
    }

    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        Arc::make_mut(&mut self.headers)
    }

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
