//! Load balance client requests by sniffing node addresses from your Elasticsearch cluster.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! You can also use a static set of addresses to load balance on.

extern crate elastic;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;

use std::error::Error;
use futures::Future;
use tokio_core::reactor::Core;
use serde_json::Value;
use elastic::prelude::*;

fn run() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;

    // An async HTTP client that will sniff node addresses from the given base address.
    let client = AsyncClientBuilder::new()
        .sniff_nodes("http://localhost:9200")
        .build(&core.handle())?;

    // Send the request and process the response.
    let ping_future = client
        .request(PingRequest::new())
        .send()
        .and_then(|res| res.into_response::<PingResponse>())
        .and_then(|ping| {
            println!("{:?}", ping);

            Ok(())
        });

    core.run(ping_future)?;

    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    run().unwrap();
}
