#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, elastic_types_macros)]

extern crate serde;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::prelude::*;

#[derive(Serialize, Deserialize, ElasticType)]
pub struct MyType {
    pub my_date2: ElasticDate<DefaultFormat>,
    pub my_string1: String,
    pub my_num1: i32,
    pub my_bool1: bool
}

fn main() {
    println!("Hello, world!");
}
