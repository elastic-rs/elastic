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
struct MyType {
    id: i32,
    title: String,
    timestamp: Date<DefaultDateMapping>,
}

fn run() -> Result<(), Box<Error>> {
    // A HTTP client and request parameters
    let client = SyncClientBuilder::new().build()?;

    // Create a document to index
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now(),
    };

    let doc_id = doc.id;

    // Create the index
    client.index_create(sample_index()).send()?;

    // Add the document mapping (optional, but makes sure `timestamp` is mapped as a `date`)
    client
        .document_put_mapping::<MyType>(sample_index())
        .send()?;

    // Index the document
    client
        .document_index(sample_index(), doc)
        .id(doc_id)
        .params_fluent(|p| p.url_param("refresh", true))
        .send()?;

    // Update the document using a script
    let update = client
        .document_update::<MyType>(sample_index(), id(doc_id))
        .script(r#"ctx._source.title = "A new title""#)
        .send()?;

    assert!(update.updated());

    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    run().unwrap();
}

fn sample_index() -> Index<'static> {
    Index::from("index_sample_index")
}
