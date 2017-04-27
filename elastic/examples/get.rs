//! Also see the `typed` sample for a more complete implementation.

extern crate serde_json;
extern crate elastic;

use serde_json::Value;
use elastic::error::*;
use elastic::prelude::*;

fn main() {
    // A reqwest HTTP client and default parameters.
    // The `params` includes the base node url (http://localhost:9200).
    let client = Client::new(RequestParams::default()).unwrap();

    let req = GetRequest::for_index_ty_id("typed_sample_index", "mytype", "1");

    let res = client.request(req)
                    .send()
                    .and_then(into_response::<GetResponse<Value>>);

    match res {
        // The doc was found: no need to index
        Ok(GetResponse { source: Some(doc), .. }) => {
            println!("document found: {:?}", doc);
        },
        // The index exists, but the doc wasn't found: map and index
        Ok(_) => {
            println!("document not found");
        },
        // No index: create it, then map and index
        Err(Error(ErrorKind::Api(ApiError::IndexNotFound { .. }), _)) => {
            println!("index not found");
        },
        // Something went wrong: panic
        Err(e) => panic!(e)
    }
}