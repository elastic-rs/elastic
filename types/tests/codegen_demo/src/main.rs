//! An example that uses `elastic_types_derive` to generate mapping boilerplate.
//! 
//! Build with `--pretty=expanded` to see the results of the compiler plugin.

#![feature(proc_macro)]
#![deny(warnings)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_types_derive;

extern crate serde;
extern crate elastic_types;

use elastic_types::prelude::*;

#[derive(Serialize, ElasticType)]
pub struct MyType {
    pub my_date: Date<DefaultDateFormat>,
    pub my_string: String,
    pub my_num: i32,
    pub my_bool: bool
}

fn main() {}
