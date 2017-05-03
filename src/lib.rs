//! Elasticsearch Response Iterators
//!
//! A crate to handle parsing and handling Elasticsearch search results which provides
//! convenient iterators to step through the results returned. It is designed to work
//! with [`elastic-reqwest`](https://github.com/elastic-rs/elastic-reqwest/).
//!
//! ## Usage
//!
//! Query your Elasticsearch Cluster, then iterate through the results
//!
//! ```no_run
//! # extern crate elastic_responses;
//! # use elastic_responses::SearchResponse;
//! # fn do_request() -> SearchResponse { unimplemented!() }
//! # fn main() {
//! // Send a request (omitted, see `samples/search`), and read the response.
//! // Parse body to JSON as an elastic_responses::SearchResponse object
//! let body_as_json: SearchResponse = do_request();
//!
//! // Use hits() or aggs() iterators
//! // Hits
//! for i in body_as_json.hits() {
//!   println!("{:?}",i);
//! }
//!
//! // Agregations
//! for i in body_as_json.aggs() {
//!   println!("{:?}",i);
//! }
//! # }
//! ```

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate quick_error;

extern crate serde;
extern crate serde_json;

extern crate slog_stdlog;
extern crate slog_envlogger;

/// Error types from Elasticsearch
pub mod error;

/// Response type parsing.
pub mod parse;

mod common;
mod ping;
mod get;
mod search;
mod bulk;

pub use self::common::*;
pub use self::ping::*;
pub use self::get::*;
pub use self::search::*;
pub use self::bulk::*;

use std::io::{Read, Result as IoResult};
use error::ResponseError;

/// The non-body component of the http response.
pub struct HttpResponseHead {
    code: u16,
}

impl HttpResponseHead {
    pub fn status(&self) -> u16 {
        self.code
    }
}

/// A http response body that implements `Read`.
struct ReadBody<B>(B);

/// A http response body that implements `AsRef<[u8]>`
struct SliceBody<B>(B);

/// A raw HTTP response with enough information to parse
/// a concrete type from it.
pub struct HttpResponse<B> {
    head: HttpResponseHead,
    body: B,
}

impl<B> HttpResponse<B> {
    /// Create a new HTTP response from the given status code
    /// and body.
    fn new(status: u16, body: B) -> Self {
        HttpResponse {
            head: HttpResponseHead {
                code: status,
            },
            body: body,
        }
    }

    /// Get the status code.
    pub fn status(&self) -> u16 {
        self.head.code
    }
}

impl<B: AsRef<[u8]>> AsRef<[u8]> for HttpResponse<B> {
    fn as_ref(&self) -> &[u8] {
        self.body.as_ref()
    }
}

impl<B: Read> Read for HttpResponse<B> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.body.read(buf)
    }
}

impl<B: AsRef<[u8]>> HttpResponse<SliceBody<B>> {
    /// Build a http response from a contiguous slice.
    pub fn from_slice(status: u16, body: B) -> Self {
        Self::new(status, SliceBody(body))
    }
}

impl<B: Read> HttpResponse<ReadBody<B>> {
    /// Build a http response from a byte reader.
    pub fn from_read(status: u16, body: B) -> Self {
        Self::new(status, ReadBody(body))
    }
}