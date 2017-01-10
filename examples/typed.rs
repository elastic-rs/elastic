//! A basic typed example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

#[macro_use]
extern crate json_str;
#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate elastic;

use elastic::types::prelude::FieldType;
use elastic::client::{self, ElasticClient, FromDoc};

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String
}

fn main() {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let (client, params) = client::default().unwrap();

    let doc = MyType {
        id: 1,
        title: String::from("A title")
    };

    let index = client::Index::from("typed_sample_index");

    // An index request
    let req = client::IndexRequest::from_doc((index.clone(), &doc));

    // Response from the index
    client.elastic_req(&params, req).unwrap();

    // A freeform JSON request body.
    let body = json_str!({
        query: {
            query_string: {
                query: "*"
            }
        }
    });

    // A search request from the body.
    let req = client::SearchRequest::for_index(index, body);

    // Send the request and process the response.
    let res: client::ResponseOf<MyType> = client
        .elastic_req(&params, req).unwrap()
        .json().unwrap();

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }
}