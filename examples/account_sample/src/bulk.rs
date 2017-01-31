//! Commands for a bulk account index.
//!
//! The bulk request body is read from a file and sent straight up to
//! Elasticsearch without any additional processing.
//!
//! In the future it would be nice to avoid buffering the file into memory
//! at all.

use std::io::{Read, Result as IoResult};
use std::fs::File;
use std::path::Path;
use elastic::client::requests::BulkRequest;

use model::account;
use model::index;

/// Get a request to create the bank index.
pub fn put<P>(path: P) -> IoResult<BulkRequest<'static>>
    where P: AsRef<Path>
{
    let body = bulk_body(path)?;
    Ok(BulkRequest::for_index_ty(index::name(), account::name(), body))
}

fn bulk_body<P>(path: P) -> IoResult<Vec<u8>>
    where P: AsRef<Path>
{
    let mut body = File::open(path)?;

    let mut buf = Vec::new();
    body.read_to_end(&mut buf)?;

    Ok(buf)
}
