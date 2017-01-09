//! A basic usage example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

#[macro_use]
extern crate json_str;
extern crate elastic;

use elastic::client::{self, ElasticClient};

fn main() {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let (client, params) = client::default().unwrap();

    // A freeform JSON request body.
    let body = json_str!({
        query: {
            query_string: {
                query: "*"
            }
        }
    });

    // A search request from the body.
    let req = client::SearchRequest::for_index("_all", body);

    // Send the request and process the response.
    let res: client::Response = client
        .elastic_req(&params, req).unwrap()
        .json().unwrap();

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }
}