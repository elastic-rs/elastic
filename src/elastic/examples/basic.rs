//! A simple search request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample executes a search request and iterates through the returned hits
//! as anonymous json objects.

extern crate elastic;
extern crate env_logger;
#[macro_use]
extern crate serde_json;

use elastic::prelude::*;
use serde_json::Value;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    // A reqwest HTTP client and default parameters.
    let client = SyncClient::builder()
        .static_node("http://localhost:9200")
        .build()?;

    // Send the request and process the response.
    let res = client
        .search::<Value>()
        .index("_all")
        .body(json!({
            "query": {
                "query_string": {
                    "query": "*"
                }
            }
        }))
        .send()?;

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }

    println!("{:?}", res);

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap();
}
