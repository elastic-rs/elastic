/*!
Response types for the Elasticsearch REST API.

All [high-level builders][request-builders] return one of these response types for you.
This module also contains helpers that can be used to parse responses from a raw [`HttpResponse`][HttpResponse].

[HttpResponse]: struct.HttpResponse.html
[request-builders]: ../index.html#request-builders
*/

mod sync;
mod async;

pub mod parse;

pub use self::sync::*;
pub use self::async::*;

pub use elastic_reqwest::res::{SearchResponse, GetResponse, Shards, CommandResponse, IndexResponse, PingResponse,
                               BulkResponse, BulkErrorsResponse};

pub use elastic_reqwest::res::search;
pub use elastic_reqwest::res::bulk;

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::{SearchResponse, GetResponse, Shards, CommandResponse, IndexResponse, PingResponse,
                               BulkResponse, BulkErrorsResponse};

    pub use super::async::AsyncResponseBuilder;
    pub use super::sync::SyncResponseBuilder;
}