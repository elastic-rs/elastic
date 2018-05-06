//! Ping the cluster.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;
extern crate env_logger;

use std::error::Error;
use elastic::prelude::*;

fn run() -> Result<(), Box<Error>> {
    // A HTTP client and request parameters
    let client = SyncClientBuilder::new().build()?;

    // Ping the cluster
    let ping = client.ping().send()?;

    println!("{:?}", ping);

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
