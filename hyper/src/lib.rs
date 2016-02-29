//! Elasticsearch Hyper Client
//!
//! A lightweight implementation of the Elasticsearch API based on Hyper.
//!
//! Each API endpoint is represented as its own function,
//! so each possible http route gets its own function.
//! The functions are also designed to work well with the `elastic_types`
//! and `elastic_macros` crates, but deserialisation is the responsibility of the caller.
//!
//! # Links
//! - [elastic_types](http://kodraus.github.io/rustdoc/elastic_types/index.html)
//! - [elastic_macros](http://kodraus.github.io/rustdoc/elastic_macros/index.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

extern crate hyper;

mod api;
pub use api::*;
