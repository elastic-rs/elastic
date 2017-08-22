//! A simple search request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample executes a search request and iterates through the returned hits
//! as anonymous json objects.

#[macro_use]
extern crate serde_json;
extern crate elastic;

use serde_json::Value;
use elastic::prelude::*;

fn main() {
    let mut core = Core::new()?;
    let pool = CpuPool::new(4)?;

    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    // We also specify a cpu pool for deserialising responses on.
    let client = AsyncClientBuilder::new().de_pool(pool).build(&core.handle())?;

    // Send the request and process the response.
    let res_future = client
        .search::<Value>()
        .index("_all")
        .body(json!({
            "query": {
                "query_string": {
                    "query": "*"
                }
            }
        }))
        .send()
        .and_then(|res| res.into_response::<BulkResponse>());

    let search_future = res_future.and_then(|res| {
        // Iterate through the hits in the response.
        for hit in res.hits() {
            println!("{:?}", hit);
        }

        Ok(())
    });

    core.run(search_future)?;
}
