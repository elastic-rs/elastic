//! Elasticsearch API Client
//! 
//! This crate is a meta-package that makes it easy to work
//! with the Elasticsearch REST API.

extern crate elastic_reqwest;
extern crate elastic_requests;
extern crate elastic_types;

pub mod client {
	pub use elastic_reqwest::*;
	pub use elastic_requests::*;
}

pub mod types {
	pub use elastic_types::*;
}
