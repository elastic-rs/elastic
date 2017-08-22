//! Ping the cluster.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;

use elastic::prelude::*;

fn main() {
    // A HTTP client and request parameters
    let client = SyncClientBuilder::new().build()?;

    // Ping the cluster
    let ping = client
        .request(PingRequest::new())
        .send()?
        .into_response::<PingResponse>()?;

    println!("{:?}", ping);
}
