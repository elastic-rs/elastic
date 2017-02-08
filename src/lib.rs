//! Elasticsearch Response Iterators
//!
//! A crate to handle parsing and handling Elasticsearch search results which provides
//! convenient iterators to step through the results returned. It is designed to work
//! with [`elastic-reqwest`](https://github.com/elastic-rs/elastic-hyper/).
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
//! // Send a request (omitted, see `samples/basic`), and read the response.
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

mod get;
mod search;

pub use self::get::*;
pub use self::search::*;

use std::io::Read;

use error::*;

/// A raw HTTP response with enough information to parse
/// a concrete type from it.
pub struct HttpResponse<R> {
    code: u16,
    body: R,
}

impl<R> HttpResponse<R> {
    /// Create a new HTTP response from the given status code
    /// and body.
    pub fn new(status: u16, body: R) -> Self {
        HttpResponse {
            code: status,
            body: body,
        }
    }

    /// Get the status code.
    pub fn status(&self) -> u16 {
        self.code
    }
}

type ApiResult<T> = Result<T, ResponseError>;

/// Convert a response message into a either a success
/// or failure result.
pub trait FromResponse
    where Self: Sized
{
    fn from_response<I: Into<HttpResponse<R>>, R: Read>(res: I) -> ApiResult<Self>;
}
