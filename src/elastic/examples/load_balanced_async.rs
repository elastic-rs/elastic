//! Load balance client requests by sniffing node addresses from your Elasticsearch cluster.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! You can also use a static set of addresses to load balance on.

extern crate elastic;
extern crate env_logger;
extern crate futures;
extern crate tokio;

use elastic::prelude::*;
use futures::Future;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    // An async HTTP client that will sniff node addresses from the given base address.
    let client = AsyncClient::builder()
        .sniff_nodes("http://localhost:9200")
        .build()?;

    // Send the request and process the response.
    let ping_future = client
        .request(PingRequest::new())
        .send()
        .and_then(|res| res.into_response::<PingResponse>())
        .and_then(|ping| {
            println!("{:?}", ping);

            Ok(())
        });

    tokio::executor::current_thread::block_on_all(ping_future)?;

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap();
}
