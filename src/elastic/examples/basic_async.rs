//! A simple search request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample executes a search request and iterates through the returned hits
//! as anonymous json objects.

extern crate elastic;
extern crate env_logger;
extern crate futures;
extern crate tokio_threadpool;
#[macro_use]
extern crate serde_json;
extern crate tokio;

use elastic::prelude::*;
use futures::Future;
use serde_json::Value;
use std::{
    error::Error,
    sync::Arc,
};
use tokio_threadpool::ThreadPool;

fn run() -> Result<(), Box<dyn Error>> {
    // A reqwest HTTP client and default parameters.
    // We also specify a cpu pool for serialising and deserialising data on.
    // The cpu pool is optional.
    let client = AsyncClient::builder()
        .static_node("http://localhost:9200")
        .serde_pool(Arc::new(ThreadPool::new()))
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

    tokio::executor::current_thread::block_on_all(search_future)?;

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap();
}
