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
//! # fn do_request() -> (u16, Vec<u8>) { unimplemented!() }
//! # fn main() {
//! // Send a document get request and read as a response
//! let (response_status, response_body) = do_request();
//! 
//! // Parse body to JSON as an elastic_responses::SearchResponse object
//! // If the response is an API error then it'll be parsed into a friendly Rust error
//! let body_as_json: SearchResponse = parse().from_slice(response_status, response_body).unwrap();
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
//! 
/// Any type that implements `IsOk` can be parsed into a concrete response or an `ApiError`:
/// 
/// ```no_run
/// # extern crate serde_json;
/// # extern crate elastic_responses;
/// # use serde_json::*;
/// # use elastic_responses::*;
/// # use elastic_responses::error::*;
/// # fn do_request() -> (u16, Vec<u8>) { unimplemented!() }
/// # fn main() {
/// // Send a document get request and read as a response
/// let (response_status, response_body) = do_request();
///
/// let get_response = parse::<GetResponseOf<Value>>().from_slice(response_status, response_body);
/// 
/// match get_response {
///     Ok(res) => {
///         // Do something with the GetResponse
///     }
///     Err(ResponseError::Api(ApiError::IndexNotFound { index })) => {
///         // Do something with the missing index error
///     }
///     _ => {
///         // Some other error
///     }
/// }
/// # }
/// ```

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
mod search;
mod bulk;
mod index;

pub use self::common::*;
pub use self::command::*;
pub use self::ping::*;
pub use self::get::*;
pub use self::search::*;
pub use self::bulk::*;
pub use self::index::*;

pub use self::parsing::parse;
