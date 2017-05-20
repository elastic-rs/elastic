//! A basic usage example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

#[macro_use]
extern crate serde_json;
extern crate elastic;

use serde_json::Value;
use elastic::prelude::*;

fn main() {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = Client::new(RequestParams::default()).unwrap();

    let query = json!({
        "query": {
            "query_string": {
                "query": "*"
            }
        }
    });

    // Send the request and process the response.
    let res = client.search::<Value>()
                    .index("_all")
                    .body(query.to_string())
                    .send()
                    .unwrap();

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }

    println!("{:?}", res);
}