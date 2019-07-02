//! A custom search response type.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample demonstrates creating a custom `SearchResponse` type that can be used with
//! the `filter_path` query parameter to only return the matched hits.

extern crate elastic;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use elastic::{
    http::receiver::IsOkOnSuccess,
    prelude::*,
};
use serde_json::Value;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct SearchResponse {
    hits: Hits,
}

#[derive(Deserialize, Debug)]
struct Hits {
    hits: Vec<Hit>,
}

#[derive(Deserialize, Debug)]
struct Hit {
    #[serde(rename = "_source")]
    pub source: Value,
}

// Implement `IsOkOnSuccess` for our custom `SearchResponse` so it can be used in the call to `into_response`.
// `IsOkOnSuccess` will return `Ok` if the response is in the `200` range.
impl IsOkOnSuccess for SearchResponse {}

fn run() -> Result<(), Box<dyn Error>> {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = SyncClient::builder().build()?;

    let query = json!({
        "query": {
            "query_string": {
                "query": "*"
            }
        }
    });

    // Send the request and process the response.
    let res = client
        .request(SearchRequest::new(query.to_string()))
        .params_fluent(|q| q.url_param("filter_path", "hits.hits._source"))
        .send()?
        .into_response::<SearchResponse>()?;

    // Iterate through the hits in the response.
    for hit in &res.hits.hits {
        println!("{:?}", hit);
    }

    println!("{:?}", res);

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
