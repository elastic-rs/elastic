//! Elasticsearch Core Types
//!
//! A high-level implementation of the core types in Elasticsearch documents.
//! 
//! Types within this crate are self-contained and handle their own serialisation/deserialisation requirements.
//! Each type also supplies a `struct` for its [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html) properties.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types/")]
#![deny(missing_docs)]

#![feature(custom_derive, custom_attribute, plugin, optin_builtin_traits, associated_type_defaults)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]
#![plugin(clippy)]

extern crate chrono;
extern crate serde;

pub mod mapping;
pub mod date;
pub mod string;