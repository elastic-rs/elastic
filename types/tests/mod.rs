#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate elastic_types;

pub mod mapping;

pub mod date;
pub mod date_format;
pub mod date_mapping;

pub mod string;
pub mod string_mapping;