//! Elasticsearch Reqwest Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//!
//! This minimal sample executes a simple search request against all indices.
//! To make things readable, the `pretty` url param is provided.

#[macro_use]
extern crate serde_json;
extern crate elastic_reqwest;

use serde_json::Value;
use elastic_reqwest::{SyncElasticClient, SyncFromResponse, RequestParams, Error};
use elastic_reqwest::req::SearchRequest;
use elastic_reqwest::res::{parse, SearchResponse};

fn run() -> Result<(), Error> {
    // Get a new default client.
    let (client, _) = elastic_reqwest::sync::default()?;

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
