//! Update a document.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample demonstrates how to create an index, add type mapping, and index a document.
//! Also see the `typed` sample for a more complete implementation.

#[macro_use]
extern crate elastic_derive;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate elastic;

use std::error::Error;
use elastic::prelude::*;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[elastic(index = "index_sample_index")]
struct MyType {
    #[elastic(id)]
    id: String,
    title: String,
    timestamp: Date<DefaultDateMapping>,
}

fn run() -> Result<(), Box<Error>> {
    // A HTTP client and request parameters
    let client = SyncClientBuilder::new().build()?;

    // Create a document to index
    let doc = MyType {
        id: "1".to_owned(),
        title: String::from("A title"),
        timestamp: Date::now(),
    };

    let doc_id = doc.id;

    // Create the index
    client.index(MyType::static_index()).create().send()?;

    // Add the document mapping (optional, but makes sure `timestamp` is mapped as a `date`)
    client
        .document::<MyType>()
        .put_mapping()
        .send()?;

    // Index the document
    client
        .document()
        .index(doc)
        .params_fluent(|p| p.url_param("refresh", true))
        .send()?;

    // Update the document using a script
    let update = client
        .document::<MyType>()
        .update("1")
        .script(r#"ctx._source.title = "A new title""#)
        .send()?;

    assert!(update.updated());

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap();
}
