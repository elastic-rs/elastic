#![feature(rustc_private)]

extern crate syntax;
extern crate serde_json;
extern crate elastic_codegen;
extern crate elastic_types;

pub mod api_parse;
pub mod gen;
pub mod rust_gen;