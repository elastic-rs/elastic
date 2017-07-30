//! Ping the cluster.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate env_logger;
extern crate elastic;

use elastic::prelude::*;

fn main() {
    env_logger::init().unwrap();

    // A HTTP client and request parameters
    let client = ClientBuilder::new().build().unwrap();

    // Ping the cluster
    let ping: PingResponse = client
        .request(PingRequest::new())
        .send()
        .and_then(into_response)
        .unwrap();

    println!("{:?}", ping);
}
