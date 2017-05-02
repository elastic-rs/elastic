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
