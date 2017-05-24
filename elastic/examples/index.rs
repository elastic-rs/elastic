//! Index a document.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//! 
//! This sample demonstrates how to create an index, add type mapping, and index a document.
//! Also see the `typed` sample for a more complete implementation.

#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate elastic;

use elastic::prelude::*;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String,
    timestamp: Date<DefaultDateFormat>,
}

fn main() {
    // A HTTP client and request parameters
    let client = Client::new(RequestParams::default()).unwrap();

    // Create a document to index
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now(),
    };

    // Create the index
    client.create_index(sample_index())
          .send()
          .unwrap();

    // Add the document mapping (optional, but makes sure `timestamp` is mapped as a `date`)
    client.put_mapping::<MyType>(sample_index())
          .send()
          .unwrap();

    // Index the document
    client.index_document(sample_index(), id(doc.id), doc)
          .send()
          .unwrap();
}

fn sample_index() -> Index<'static> {
    Index::from("index_sample_index")
}