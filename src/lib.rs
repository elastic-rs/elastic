//! Elasticsearch Response Iterators
//!
//! A crate to handle parsing and handling Elasticsearch search results which provides
//! convenient iterators to step through the results returned. It is designed to work
//! with [`elastic-reqwest`][elastic-reqwest].
//! It also re-exports `serde_json::Value` for convenient anonymous json objects.
//! 
//! This crate provides parsers that can be used to convert a http response into a concrete
//! type or an API error.
//!
//! ## Usage
//! 
//! This crate is on [crates.io][crates-io].
//! Add `elastic_responses` to your `Cargo.toml`:
//! 
//! ```text
//! [dependencies]
//! elastic_responses = "*"
//! ```
//! 
//! Use the [`parse`][parse] function to deserialise a http response to a `Result<T, ApiError>` for some
//! concrete response type `T`.
//! 
//! # Examples
//!
//! Run a [Query DSL][query-dsl] query, then iterate through the results:
//!
//! ```no_run
//! # extern crate elastic_responses;
//! # use elastic_responses::*;
//! # fn do_request() -> (u16, Vec<u8>) { unimplemented!() }
//! # fn main() {
//! // Send a search request and read as a response
//! let (response_status, response_body) = do_request();
//! 
//! // Parse body to JSON as an elastic_responses::SearchResponse object
//! // If the response is an API error then it'll be parsed into a friendly Rust error
//! let response = parse::<SearchResponse<Value>>().from_slice(response_status, response_body).unwrap();
//!
//! // Use hits() or aggs() iterators
//! // Hits
//! for i in response.hits() {
//!   println!("{:?}", i);
//! }
//!
//! // Agregations
//! for i in response.aggs() {
//!   println!("{:?}", i);
//! }
//! # }
//! ```
//! 
//! Run a [Get Document][get-document] request, and handle cases where the document wasn't found or the index doesn't exist:
//! 
//! ```no_run
//! # extern crate serde_json;
//! # extern crate elastic_responses;
//! # use serde_json::Value;
//! # use elastic_responses::*;
//! # use elastic_responses::error::*;
//! # fn do_request() -> (u16, Vec<u8>) { unimplemented!() }
//! # fn main() {
//! // Send a document get request and read as a response
//! let (response_status, response_body) = do_request();
//!
//! let response = parse::<GetResponse<Value>>().from_slice(response_status, response_body);
//! 
//! match response.map(|res| res.into_document()) {
//!     Ok(Some(doc)) => {
//!         // The document was found
//!     }
//!     Ok(None) => {
//!         // The document was not found
//!     }
//!     Err(ResponseError::Api(ApiError::IndexNotFound { index })) => {
//!         // The index doesn't exist
//!     }
//!     _ => {
//!         // Some other error
//!     }
//! }
//! # }
//! ```
//! [elastic-reqwest]: https://github.com/elastic-rs/elastic-reqwest/
//! [crates-io]: https://crates.io/crates/elastic_responses
//! [query-dsl]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search.html
//! [get-document]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
//! [parse]: parsing/fn.parse.html

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
pub mod parsing;

mod common;
mod command;
mod ping;
mod get;
pub mod search;
pub mod bulk;
mod index;

pub use self::common::*;
pub use self::command::*;
pub use self::ping::*;
pub use self::get::*;
pub use self::search::SearchResponse;
pub use self::bulk::{BulkResponse, BulkErrorsResponse};
pub use self::index::*;

pub use self::parsing::parse;

/// Re-export of `serde_json::Value` for convenience.
pub use serde_json::Value;