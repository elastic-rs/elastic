#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate chrono;
extern crate elastic_types;

pub mod date;