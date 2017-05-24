//! Ping the cluster.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;

use elastic::prelude::*;

fn main() {
    // A HTTP client and request parameters
    let client = Client::new(RequestParams::default()).unwrap();

    // Ping the cluster
    let ping: PingResponse = client.request(PingRequest::new())
                                   .send()
                                   .and_then(into_response)
                                   .unwrap();

    println!("{:?}", ping);
}