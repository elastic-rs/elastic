//! Elasticsearch Reqwest Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//!
//! This minimal sample executes a simple search request against all indices.
//! To make things readable, the `pretty` url param is provided.

#[macro_use]
extern crate json_str;
extern crate elastic_reqwest as cli;

use cli::{ElasticClient, RequestParams};
use cli::req::SearchRequest;
use std::io::Read;

fn main() {
    // Get a new default client.
    let (client, _) = cli::default().unwrap();

    // Create a new set of params with pretty printing.
    let params = RequestParams::default().url_param("pretty", true);

    // Create a query DSL request body.
    let req = {
        let body = json_str!({
            query: {
                query_string: {
                    query: "*"
                }
            }
        });

        SearchRequest::for_index("_all", body)
    };

    // Send the request and read the response.
    let res = {
        let res = client.elastic_req(&params, req).unwrap();

        cli::res::parse::<SearchResponse>().from_reader(res.status().to_u16(), res).unwrap()
    };

    println!("Got response: {:?}", res);
}
