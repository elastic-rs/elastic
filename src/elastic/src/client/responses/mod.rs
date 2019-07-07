/*!
Elasticsearch Response types.

This module contains implementation details that are useful if you want to customise the request process, but aren't generally important for sending requests.
*/

pub mod bulk;
mod command;
pub mod common;
mod document_delete;
mod document_get;
mod document_index;
mod document_update;
pub mod nodes_info;
mod ping;
pub mod search;
mod sql;

mod index_exists;

#[cfg(test)]
mod tests;

#[doc(inline)]
pub use self::{
    bulk::{
        BulkErrorsResponse,
        BulkResponse,
    },
    command::*,
    document_delete::*,
    document_get::*,
    document_index::*,
    document_update::*,
    nodes_info::NodesInfoResponse,
    ping::*,
    search::SearchResponse,
    sql::*,
};

pub use self::index_exists::*;

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::{
        bulk::Action as BulkAction,
        BulkErrorsResponse,
        BulkResponse,
        CommandResponse,
        DeleteResponse,
        GetResponse,
        IndexResponse,
        IndicesExistsResponse,
        NodesInfoResponse,
        PingResponse,
        SearchResponse,
        SqlQueryResponse,
        UpdateResponse,
    };
}
