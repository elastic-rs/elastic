//! Elasticsearch Codegen
//!
//! A library that contains useful structures and functions for generating code from the Elasticsearch API specification.
//! 
//! The library is organised into a few steps:
//! 
//! - Parsing API / Test source to a simple syntax tree from some source
//! - Analysing the API tree to get URLs and their required parameters for API endpoints
//! - Generating source code from the annotated syntax tree
//! - Emitting the results to some destination
//! 
//! A consumer of this library can take advantage of any layer and those below it for their desired level of abstraction.
//! For example, currently only Rust codegen helpers are included through the `libsyntax` crate, but other languages could be added on top of the same API AST.
//! 
//! Where possible, the language-specific requirements for each step are contained in their own modules.
//! Helpers that are relevant to a language in any step live in their own root module.
//! 
//! # Links
//! - [Spec Source](https://github.com/elastic/elasticsearch/tree/master/rest-api-spec)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_codegen/")]
#![deny(missing_docs)]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]

#![feature(associated_type_defaults, rustc_private, core_intrinsics, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate syntax;

pub mod api;
pub mod test;
pub mod gen;
pub mod emit;
mod parse;