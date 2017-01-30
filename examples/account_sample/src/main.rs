//! This sample is a more fleshed out application using `elastic`.
//! 
//! It expects you have an Elasticsearch node running on `localhost:9200`.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate json_str;

extern crate serde;
extern crate serde_json;
extern crate elastic;

pub mod model;

fn main() {
    println!("Hello, world!");
}
