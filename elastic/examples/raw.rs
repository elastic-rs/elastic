//! A basic raw usage example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

extern crate elastic;

use std::io::Read;
use elastic::prelude::*;

fn main() {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let params = RequestParams::default().url_param("pretty", true);
    let client = Client::new(params).unwrap();

    // A search request from the body.
    let req = SearchRequest::for_index("_all", r#"{ "query": { "match_all": {} } }"#);

    // Send the request and process the response.
    let mut res = client.request(req)
                        .send()
                        .map(|res| res.raw())
                        .unwrap();

    // Check if the response is in the 200 range
    match res.status() {
        200...299 => (),
        status => panic!("error: {:?}", status)
    }

    // Read the response body to a string
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("{}", body);
}