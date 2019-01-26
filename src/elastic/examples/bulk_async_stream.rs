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
#[macro_use]
extern crate serde_json;

use elastic::prelude::*;
use futures::{
    stream,
    Future,
    Sink,
    Stream,
};
use std::error::Error;
use std::time::Duration;
use tokio_core::reactor::Core;

fn run() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;

    // A HTTP client and request parameters
    let client = AsyncClientBuilder::new().build(&core.handle())?;

    // Get a stream for bulk operations
    // Individual operations can be sent to the stream and will be buffered to Elasticsearch
    let (bulk_stream, bulk_responses) = client
        .bulk_stream()
        .index("bulk_idx")
        .ty("bulk_ty")
        .timeout(Duration::from_secs(5))
        .body_size_bytes(1024)
        .build();

    let ops = (0..1000).into_iter().map(|i| {
        bulk_raw()
            .index(json!({
                "id": i,
                "title": "some string value"
            }))
            .id(i)
    });

    let req_future = bulk_stream.send_all(stream::iter_ok(ops));

    let res_future = bulk_responses.for_each(|bulk| {
        println!("response:");
        for op in bulk {
            match op {
                Ok(op) => println!("  ok: {:?}", op),
                Err(op) => println!("  err: {:?}", op),
            }
        }

        Ok(())
    });

    core.run(req_future.join(res_future))?;

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
