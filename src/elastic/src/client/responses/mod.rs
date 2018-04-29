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

pub use elastic_responses::{BulkErrorsResponse, BulkResponse, CommandResponse, DeleteResponse, GetResponse, IndexResponse, IndicesExistsResponse, PingResponse, SearchResponse, Shards, UpdateResponse};

pub use elastic_responses::search;
pub use elastic_responses::bulk;

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::{BulkErrorsResponse, BulkResponse, CommandResponse, DeleteResponse, GetResponse, IndicesExistsResponse, IndexResponse, PingResponse, SearchResponse, Shards, UpdateResponse};

    pub use super::async::AsyncResponseBuilder;
    pub use super::sync::SyncResponseBuilder;
}
