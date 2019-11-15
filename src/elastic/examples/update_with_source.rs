//! Update a document and return the updated document in a single request.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample demonstrates how to index & update a document and return
//! the newly updated document.
//! Also see the `typed` sample for a more complete implementation.

use std::error::Error;

use elastic::prelude::*;
use elastic_derive::ElasticType;
use env_logger;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[elastic(index = "update_with_source_sample_index")]
struct NewsArticle {
    #[elastic(id)]
    id: String,
    title: String,
    content: String,
    likes: i64,
}

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[elastic(index = "update_with_source_sample_index")]
struct UpdatedNewsArticle {
    #[elastic(id)]
    id: String,
    title: String,
    content: String,
    likes: i64,
}

fn run() -> Result<(), Box<dyn Error>> {
    // A HTTP client and request parameters
    let client = SyncClient::builder().build()?;

    // Create a document to index
    let doc = NewsArticle {
        id: "1".to_string(),
        title: "A title".to_string(),
        content: "Some content.".to_string(),
        likes: 0,
    };

    // Index the document
    client.document().index(doc).send()?;

    // Update the document using a script
    let update = client
        .document::<NewsArticle>()
        .update("1")
        .script("ctx._source.likes++")
        // Request that the updated document be returned with the response
        .source()
        .send()?;

    assert!(update.updated());

    // Deserialize the updated document,
    // will return `None` if `source()` was not called on the request
    let updated_doc = update.into_document::<UpdatedNewsArticle>().unwrap();

    assert!(updated_doc.likes > 0);

    println!("{:#?}", &updated_doc);

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
