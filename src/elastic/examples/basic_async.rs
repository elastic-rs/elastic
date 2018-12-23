//! A simple search request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample executes a search request and iterates through the returned hits
//! as anonymous json objects.

extern crate elastic;
extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
#[macro_use]
extern crate serde_json;
extern crate tokio;

use std::error::Error;
use futures::Future;
use futures_cpupool::CpuPool;
use serde_json::Value;
use elastic::prelude::*;
use tokio::runtime::current_thread::block_on_all;

fn run() -> Result<(), Box<Error>> {
    let pool = CpuPool::new(4);

    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    // We also specify a cpu pool for serialising and deserialising data on.
    let client = AsyncClientBuilder::new()
        .serde_pool(pool)
        .build()?;

    // Send the request and process the response.
    let res_future = client
        .search::<Value>()
        .index("_all")
        .body(json!({
            "query": {
                "query_string": {
                    "query": "*"
                }
            }
        }))
        .send();

    let search_future = res_future.and_then(|res| {
        // Iterate through the hits in the response.
        for hit in res.hits() {
            println!("{:?}", hit);
        }

        Ok(())
    });

    block_on_all(search_future)?;

    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    run().unwrap();
}
