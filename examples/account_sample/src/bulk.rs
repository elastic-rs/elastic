//! Commands for a bulk account index.
//! 
//! The bulk request body is read from a file and sent straight up to
//! Elasticsearch without any additional processing.
//! 
//! In the future it would be nice to avoid buffering the file into memory
//! at all.

use elastic::client::requests::{Body, BulkRequest};

use model::account;
use model::index;

/// Get a request to create the bank index.
pub fn put() -> BulkRequest<'static> {
    BulkRequest::for_index_ty(index::name(), account::name(), Body::none())
}