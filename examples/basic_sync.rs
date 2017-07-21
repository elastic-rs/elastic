//! Elasticsearch Reqwest Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//!
//! This minimal sample executes a simple search request against all indices.
//! To make things readable, the `pretty` url param is provided.

#[macro_use]
extern crate serde_json;
extern crate elastic_reqwest as cli;

use serde_json::Value;
use cli::{SyncElasticClient, SyncFromResponse, RequestParams, Error};
use cli::req::SearchRequest;
use cli::res::{parse, SearchResponse};

fn run() -> Result<(), Error> {
    // Get a new default client.
    let (client, _) = cli::default_sync()?;

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
    let http_res = client.elastic_req(&params, req)?;

    let res = parse::<SearchResponse<Value>>().from_response(http_res)?;

    println!("Got response: {:?}", res);

    Ok(())
}

fn main() {
    run().unwrap();
}
