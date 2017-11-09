//! Send a bulk request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample demonstrates a bulk request.
//! There are a few extra dials you can tweak on bulk requests to get more
//! performance out of them.
//! See the docs for `BulkResponse` for more details.

extern crate elastic;
extern crate env_logger;
extern crate futures;
extern crate tokio_core;

use std::error::Error;
use futures::Future;
use tokio_core::reactor::Core;
use elastic::prelude::*;

fn run() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;

    // A HTTP client and request parameters
    let client = AsyncClientBuilder::new().build(&core.handle())?;

    // Execute a bulk request
    let res_future = client
        .request(BulkRequest::new(bulk_body()))
        .send()
        .and_then(|res| res.into_response::<BulkResponse>());

    let bulk_future = res_future.and_then(|bulk| {
        for op in bulk {
            match op {
                Ok(op) => println!("ok: {:?}", op),
                Err(op) => println!("err: {:?}", op),
            }
        }

        Ok(())
    });

    core.run(bulk_future)?;

    Ok(())
}

fn bulk_body() -> String {
    let mut bulk = String::new();
    for i in 1..1000 {
        let header = format!(
            "{{ \"index\" : {{ \"_index\" : \"test\", \"_type\" : \"ty\", \"_id\" : \"{}\" }} }}",
            i
        );
        let body = format!("{{ \"title\" : \"string value {}\" }}", i);

        bulk.push_str(&header);
        bulk.push('\n');
        bulk.push_str(&body);
        bulk.push('\n');
    }

    bulk
}

fn main() {
    env_logger::init().unwrap();
    run().unwrap()
}
