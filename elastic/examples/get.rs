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

    let res = client.get::<Value>(index("typed_sample_index"), id("1")).ty("mytype").send();

    // Responses can be unpacked in various ways. In this case we check if:
    // - The call succeeded and the document was found
    // - The call succeeded but the document wasn't found
    // - The call failed because the index doesn't exist
    // - The call failed for some other reason
    match res {
        // The doc was found
        Ok(GetResponse { source: Some(doc), .. }) => {
            println!("document found: {:?}", doc);
        },
        // The index exists, but the doc wasn't found
        Ok(_) => {
            println!("document not found, but index exists");
        },
        // An error was returned
        Err(e) => {
            match *e.kind() {
                // No index
                ErrorKind::Api(ApiError::IndexNotFound { .. }) => {
                    println!("index not found");
                },
                // Something went wrong, panic
                _ => panic!(e)
            }
        }
    }
}