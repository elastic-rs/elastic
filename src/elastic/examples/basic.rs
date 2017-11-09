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

use std::error::Error;
use serde_json::Value;
use elastic::prelude::*;

fn run() -> Result<(), Box<Error>> {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = SyncClientBuilder::new().build()?;

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
    env_logger::init().unwrap();
    run().unwrap();
}
