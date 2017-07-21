//! Elasticsearch Reqwest Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//!
//! This minimal sample executes a simple search request against all indices.
//! To make things readable, the `pretty` url param is provided.

#[macro_use]
extern crate serde_json;
extern crate tokio_core;
extern crate futures;
extern crate elastic_reqwest as cli;

use serde_json::Value;
use tokio_core::reactor::Core;
use futures::Future;
use cli::{AsyncElasticClient, AsyncFromResponse, RequestParams, Error};
use cli::req::SearchRequest;
use cli::res::{parse, SearchResponse};

fn run() -> Result<(), Error> {
    let mut core = Core::new().unwrap();

    // Get a new default client.
    let (client, _) = cli::default_async(&core.handle())?;

    // Create a new set of params with pretty printing.
    let params = RequestParams::default().url_param("pretty", true);

    // Create a query DSL request body.
    let req = {
        let query = "*";

        let body = json!({
            "query": {
                "query_string": {
                    "query": query
                }
            }
        });

        SearchRequest::for_index("_all", body)
    };

    // Send the request and read the response.
    let req_fut = client.elastic_req(&params, req)
        .and_then(|http_res| parse::<SearchResponse<Value>>().from_response(http_res))
        .and_then(|res| {
            println!("Got response: {:?}", res);
            
            Ok(())
        });

    core.run(req_fut).unwrap();

    Ok(())
}

fn main() {
    run().unwrap();
}
