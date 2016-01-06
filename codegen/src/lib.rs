//TODO: Uncomment attr
//#![deny(missing_docs)]

#![feature(rustc_private, core_intrinsics, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate syntax;

pub mod api;
pub mod test;