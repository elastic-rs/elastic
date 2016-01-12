//! Elasticsearch Codegen
//!
//! A library that contains useful structures and functions for generating code from the Elasticsearch API specification.
//! 
//! The library is organised into a few layers:
//! 
//! - Parsing API / Test source to an AST
//! - Getting URLs and their required parameters for API endpoints
//! - Helpers for Rust codegen from an API AST
//! 
//! A consumer of this library can take advantage of any layer and those below it for their desired level of abstraction.
//! For example, currently only Rust codegen helpers are included through the `libsyntax` crate, but other languages could be added on top of the same API AST.
//! 
//! # Links
//! - [Spec Source](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

//TODO: Uncomment attr
//#![deny(missing_docs)]

#![feature(rustc_private, core_intrinsics, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate syntax;
#[macro_use]
extern crate chomp;

pub mod api;
pub mod test;
pub mod gen;
