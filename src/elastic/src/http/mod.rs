/*! 
Raw HTTP modules.

These types are re-exported from `reqwest` and used in parts of `elastic`s public API.
They may eventually be wrapped and made implementation details.
*/

mod sync;
mod async;

pub use self::sync::*;
pub use self::async::*;

pub use reqwest::Url;
pub use reqwest::header;

pub use elastic_requests::Method;
pub use elastic_responses::StatusCode;

use self::header::Headers;

/**
A request just before being sent.
*/
pub struct HttpRequest<TBody> {
    pub url: Url,
    pub method: Method,
    pub headers: Headers,
    pub body: Option<TBody>,
    pub(crate) _private: (),
}
