//! Elasticsearch Response Iterators
//!
//! A crate to handle parsing and handling Elasticsearch search results which provides
//! convenient iterators to step through the results returned. It is designed to work
//! with [`elastic-reqwest`](https://github.com/elastic-rs/elastic-reqwest/).
//! 
//! This crate provides a generic `HttpResponse` that can be parsed into a concrete response
//! type, or an `ApiError`.
//!
//! ## Usage
//!
//! Query your Elasticsearch Cluster, then iterate through the results
//!
//! ```no_run
//! # extern crate elastic_responses;
//! # use elastic_responses::*;
//! # fn do_request() -> HttpResponseSlice<Vec<u8>> { unimplemented!() }
//! # fn main() {
//! // Send a request (omitted, see `samples/search`)
//! let http_response = do_request();
//! 
//! // Parse body to JSON as an elastic_responses::SearchResponse object
//! // If the response is an API error then it'll be parsed into a friendly Rust error
//! let body_as_json: SearchResponse = http_response.into_response().unwrap();
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

pub mod error;
pub mod parse;

mod common;
mod ping;
mod get;
mod search;
mod bulk;
mod index;

pub use self::common::*;
pub use self::ping::*;
pub use self::get::*;
pub use self::search::*;
pub use self::bulk::*;
pub use self::index::*;

use std::io::{Read, Result as IoResult};

/// The non-body component of the HTTP response.
pub struct HttpResponseHead {
    code: u16,
}

impl HttpResponseHead {
    /// Get the status code.
    pub fn status(&self) -> u16 {
        self.code
    }
}

/// A HTTP response body that implements `Read`.
pub struct ReadBody<B>(B);

/// A HTTP response body that implements `AsRef<[u8]>`
pub struct SliceBody<B>(B);

/// A raw HTTP response with enough information to parse
/// a concrete type from it.
/// 
/// `HttpResponse`s are generic over the body kind, which can be either
/// an IO buffer or contiguous slice of bytes.
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

impl<B: AsRef<[u8]>> AsRef<[u8]> for HttpResponse<SliceBody<B>> {
    fn as_ref(&self) -> &[u8] {
        self.body.0.as_ref()
    }
}

impl<B: Read> Read for HttpResponse<ReadBody<B>> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.body.0.read(buf)
    }
}

impl<B: AsRef<[u8]>> HttpResponse<SliceBody<B>> {
    /// Build a HTTP response from a contiguous slice.
    pub fn from_slice(status: u16, body: B) -> Self {
        Self::new(status, SliceBody(body))
    }
}

impl<B: Read> HttpResponse<ReadBody<B>> {
    /// Build a HTTP response from a byte reader.
    /// 
    /// # Examples
    /// 
    pub fn from_read(status: u16, body: B) -> Self {
        Self::new(status, ReadBody(body))
    }
}

pub type HttpResponseRead<T> = HttpResponse<ReadBody<T>>;
pub type HttpResponseSlice<T> = HttpResponse<SliceBody<T>>;