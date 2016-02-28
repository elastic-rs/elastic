#![feature(plugin)]
#![plugin(elastic_macros)]
#![plugin(serde_macros)]

extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate elastic_macros;

pub mod date;
pub mod json;