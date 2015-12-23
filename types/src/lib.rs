#![feature(vec_push_all, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate chomp;
extern crate chrono;
extern crate serde;

pub mod date;