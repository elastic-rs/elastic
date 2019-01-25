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

use std::error::Error;
use futures::Future;
use tokio_core::reactor::Core;
use elastic::prelude::*;

fn run() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;

    // A HTTP client and request parameters
    let client = AsyncClientBuilder::new().build(&core.handle())?;

    let ops = (0..1000)
        .into_iter()
        .map(|i| bulk_raw().index(json!({
                "id": i,
                "title": "some string value"
            }))
            .id(i));

    // Execute a bulk request
    let res_future = client.bulk()
        .index("bulk_idx")
        .ty("bulk_ty")
        .extend(ops)
        .send();

    let res_future = res_future.and_then(|bulk| {
        for op in bulk {
            match op {
                Ok(op) => println!("ok: {:?}", op),
                Err(op) => println!("err: {:?}", op),
            }
        }

        Ok(())
    });

    core.run(res_future)?;

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
