/*!
Raw HTTP modules.

These types are re-exported from `reqwest` and used in parts of `elastic`s public API.
They may eventually be wrapped and made implementation details.
*/

mod async;
mod sync;

pub use self::async::*;
pub use self::sync::*;

pub use reqwest::header;
pub use reqwest::Url;

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
