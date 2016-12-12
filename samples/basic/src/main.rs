//! Elasticsearch Hyper Client Samples
//!
//! This sample assumes you have a node running on `localhost`.
//!
//! This minimal sample executes a simple search request against all indices.
//! To make things readable, the `pretty` url param is provided.

#[macro_use]
extern crate json_str;
extern crate hyper;
extern crate elastic_hyper;
extern crate elastic_requests as requests;

use elastic_hyper::{ElasticClient, RequestParams};
use hyper::client::Client;
use std::io::Read;

fn main() {
    let cli = Client::new();
    let params = RequestParams::default().url_params(vec![("pretty", "true".to_owned())]);

    let body = json_str!({
        query: {
            query_string: {
                query: "*"
            }
        }
    });

    let mut res = cli.elastic_req(&params, requests::SearchRequest::index("_all", body)).unwrap();

    let mut message = String::new();
    res.read_to_string(&mut message).unwrap();

    println!("Got response: {}", message);
}
