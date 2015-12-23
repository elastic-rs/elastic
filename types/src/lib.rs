#![feature(vec_push_all, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate chrono;
#[macro_use]
extern crate chomp;
extern crate serde;

pub mod date;