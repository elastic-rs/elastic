#![feature(test, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_types_codegen)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate elastic_types;

pub mod date;