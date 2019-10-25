//! A basic typed example.
//!
//! NOTE: This sample expects you have a node running on `localhost:9200`.
//!
//! This sample does the following:
//!
//! - Check if a particular index exists
//! - Create the index if it doesn't
//! - Put the mapping for a document type
//! - Index a document
//! - Search the index and iterate over hits

#[macro_use]
extern crate elastic_derive;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate elastic;

use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use std::error::Error as StdError;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
#[elastic(index = "typed_sample_index")]
struct MyType {
    #[elastic(id)]
    id: String,
    title: String,
    timestamp: Date<DefaultDateMapping>,
}

fn run() -> Result<(), Box<dyn StdError>> {
    // A HTTP client and request parameters
    let client = SyncClient::builder().build()?;

    // Create a document to index
    let doc = MyType {
        id: "1".to_owned(),
        title: String::from("A title"),
        timestamp: Date::now(),
    };

    // Check if the doc exists and index if it doesn't
    ensure_indexed(&client, doc)?;

    // Do a search request
    let res = search(&client, "title")?;

    println!("{:?}", res);

    Ok(())
}

fn ensure_indexed(client: &SyncClient, doc: MyType) -> Result<(), Error> {
    let get_res = client.document::<MyType>().get(doc.id.clone()).send();

    match get_res.map(|res| res.into_document()) {
        // The doc was found: no need to index
        Ok(Some(doc)) => {
            println!("document already indexed: {:?}", doc);
        }
        // The index exists, but the doc wasn't found: map and index
        Ok(None) => {
            println!("indexing doc");

            put_doc(client, doc)?;
        }
        // No index: create it, then map and index
        Err(Error::Api(ApiError::IndexNotFound { .. })) => {
            println!("creating index and doc");

            put_index(client)?;
            put_doc(client, doc)?;
        }
        // Something went wrong: panic
        Err(e) => return Err(e),
    }

    Ok(())
}

fn put_index(client: &SyncClient) -> Result<(), Error> {
    client.index(MyType::static_index()).create().send()?;
    client.document::<MyType>().put_mapping().send()?;

    Ok(())
}

fn put_doc(client: &SyncClient, doc: MyType) -> Result<(), Error> {
    client
        .document()
        .index(doc)
        .params_fluent(|p| p.url_param("refresh", true))
        .send()?;

    Ok(())
}

fn search(client: &SyncClient, query: &'static str) -> Result<SearchResponse<MyType>, Error> {
    client
        .search()
        .index(MyType::static_index())
        .body(json!({
              "query": {
                  "query_string": {
                      "query": query
                  }
              }
        }))
        .send()
}

fn main() {
    env_logger::init();
    run().unwrap()
}
