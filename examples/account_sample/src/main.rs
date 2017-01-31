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
pub mod bulk;

use std::io::Read;
use elastic::client::{Client, RequestParams};
use elastic::http::StatusCode;

fn main() {
    let client = Client::new(RequestParams::default()).unwrap();

    println!("checking index");

    let exists = client.request(model::index::exists()).send().unwrap();
    match exists.status() {
        StatusCode::NotFound => {
            println!("creating the index");

            let res = client.request(model::index::put()).send().unwrap();
            let success = res.status().is_success();

            if !success {
                let mut body = String::new();
                res.raw().read_to_string(&mut body).unwrap();

                panic!("{}", body);
            }
        }
        x if x.is_success() => println!("index already exists"),
        _ => panic!("failed to check whether index exists"),
    }

    println!("updating docs");

    let res = client.request(bulk::put("data/accounts.json").unwrap()).send().unwrap();

    let success = res.status().is_success();
    if !success {
        let mut body = String::new();
        res.raw().read_to_string(&mut body).unwrap();

        panic!("{}", body);
    }

    println!("done");
}
