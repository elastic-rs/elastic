//! Send a bulk request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample demonstrates a bulk request.
//! There are a few extra dials you can tweak on bulk requests to get more
//! performance out of them.
//! See the docs for `BulkResponse` for more details.

extern crate elastic;

use elastic::prelude::*;

fn main() {
    // A HTTP client and request parameters
    let client = Client::new(RequestParams::default()).unwrap();

    // Execute a bulk request
    let bulk: BulkResponse = client
        .request(BulkRequest::new(bulk_body()))
        .send()
        .and_then(into_response)
        .unwrap();

    println!("Successful operations");
    for op in bulk.items.ok {
        println!("{:?}", op);
    }

    println!("Failed operations");
    for op in bulk.items.err {
        println!("{:?}", op);
    }
}

fn bulk_body() -> String {
    let mut bulk = String::new();
    for i in 1..1000 {
        let header = format!("{{ \"index\" : {{ \"_index\" : \"test\", \"_type\" : \"ty\", \"_id\" : \"{}\" }} }}",
                             i);
        let body = format!("{{ \"title\" : \"string value {}\" }}", i);

        bulk.push_str(&header);
        bulk.push('\n');
        bulk.push_str(&body);
        bulk.push('\n');
    }

    bulk
}
