//! An example that uses `elastic_types_macros` to generate mapping boilerplate.
//! 
//! Build with `--pretty=expanded` to see the results of the compiler plugin.

#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, elastic_types_macros)]

extern crate serde;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::prelude::*;

#[derive(Serialize, ElasticType)]
pub struct MyType {
    pub my_date: ElasticDate<DefaultDateFormat>,
    pub my_string: String,
    pub my_num: i32,
    pub my_bool: bool
}

fn main() {}
