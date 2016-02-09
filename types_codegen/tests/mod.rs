#![feature(plugin)]
#![plugin(elastic_types_codegen)]
#![plugin(serde_macros)]

extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate elastic_types_codegen;

pub mod date;
pub mod json;