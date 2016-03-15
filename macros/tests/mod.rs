#![allow(unused_attributes)]
#![allow(plugin_as_library)]

#![feature(plugin, custom_derive)]
#![plugin(elastic_macros)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_macros;

pub mod json;

#[cfg(feature = "types")]
pub mod types;