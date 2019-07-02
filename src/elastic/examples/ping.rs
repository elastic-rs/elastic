//! Ping the cluster.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;
extern crate env_logger;

use elastic::prelude::*;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    // A HTTP client and request parameters
    let client = SyncClient::builder().build()?;

    // Ping the cluster
    let ping = client.ping().send()?;

    println!("{:?}", ping);

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
