//! A raw search request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample demonstrates a raw search request where the body is read into a `String` rather
//! than being deserialised.

extern crate elastic;
extern crate env_logger;

use elastic::prelude::*;
use std::{
    error::Error,
    io::Read,
};

fn run() -> Result<(), Box<dyn Error>> {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = SyncClient::builder()
        .params_fluent(|p| p.url_param("pretty", true))
        .build()?;

    // A search request from the body.
    let req = SearchRequest::for_index("_all", r#"{ "query": { "match_all": {} } }"#);

    // Send the request and process the response.
    let mut res = client.request(req).send()?.into_raw();

    // Check if the response is in the 200 range
    match res.status() {
        status if status.is_success() => (),
        status => panic!("error: {:?}", status),
    }

    // Read the response body to a string
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("{}", body);

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
