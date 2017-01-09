//! Elasticsearch API Client
//! 
//! This crate is a meta-package that makes it easy to work
//! with the Elasticsearch REST API.

extern crate reqwest;
extern crate elastic_reqwest;
extern crate elastic_requests;
extern crate elastic_types;
extern crate elastic_responses;

pub mod client {
	//! HTTP client, requests and responses.
	//! 
	//! This module contains the core `ElasticClient` trait, as well
	//! as request and response types.

	pub use reqwest::Client;

	/// A client wrapper over [`reqwest`](https://github.com/seanmonstar/reqwest).
	pub use elastic_reqwest::*;

	/// Request types the Elasticsearch REST API.
	pub use elastic_requests::*;

	/// Response types for the Elasticsearch REST API.
	pub use elastic_responses::*;
}

pub mod types {
	//! Indexable documents and type mapping.
	//! 
	//! This module contains tools for defining Elasticsearch-compatible
	//! document types.

	pub use elastic_types::*;
}