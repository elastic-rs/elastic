//! Elasticsearch API Client
//!
//! This crate is a meta-package that makes it easy to work
//! with the Elasticsearch REST API.

#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate elastic_reqwest;
extern crate elastic_requests;
extern crate elastic_types;
extern crate elastic_responses;

mod http_client;
mod impls;

pub mod errors;

pub mod client {
    //! HTTP client, requests and responses.
    //!
    //! This module contains the HTTP client, as well
    //! as request and response types.

    pub use super::http_client::*;
    pub use super::impls::*;
}

pub mod types {
    //! Indexable documents and type mapping.
    //!
    //! This module contains tools for defining Elasticsearch-compatible
    //! document types.

    pub use elastic_types::*;
}

pub mod prelude {
    pub use super::client::*;
    pub use super::types::prelude::*;
}