#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate chrono;
#[macro_use(date_fmt)]
extern crate elastic_types;

pub mod date;