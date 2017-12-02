/*! 
Raw HTTP modules.

These types are re-exported from `reqwest` and used in parts of `elastic`s public API.
They may eventually be wrapped and made implementation details.
*/

pub use reqwest::Url;
pub use reqwest::header;
pub use reqwest::Body as SyncBody;
pub use reqwest::unstable::async::{Body as AsyncBody, Chunk as AsyncChunk};

pub use elastic_requests::Method;

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
