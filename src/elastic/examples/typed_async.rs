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

extern crate env_logger;
#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate tokio_core;
extern crate futures;

extern crate elastic;

use std::error::Error as StdError;
use futures::{Future, IntoFuture};
use elastic::prelude::*;
use elastic::error::{Error, ApiError};

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String,
    timestamp: Date<DefaultDateMapping>,
}

fn run() -> Result<(), Box<StdError>> {
    let mut core = tokio_core::reactor::Core::new()?;

    // A HTTP client and request parameters
    let client = AsyncClientBuilder::new().build(&core.handle())?;

    // Create a document to index
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now(),
    };

    // Check if the doc exists and index if it doesn't
    let index_future = ensure_indexed(client.clone(), doc);

    // Do a search request
    let search_future = search(client.clone(), "title");

    let res_future = index_future
        .and_then(|_| search_future)
        .and_then(|res| {
            println!("{:?}", res);

            Ok(())
        });

    core.run(res_future)?;

    Ok(())
}

fn sample_index() -> Index<'static> {
    Index::from("typed_sample_index")
}

fn ensure_indexed(client: AsyncClient, doc: MyType) -> Box<Future<Item = (), Error = Error>> {
    let get_res = client
        .document_get::<MyType>(sample_index(), id(doc.id))
        .send()
        .map(|res| res.into_document());

    let put_doc = get_res.then(move |res| {
        match res {
            // The doc was found: no need to index
            Ok(Some(doc)) => {
                println!("document already indexed: {:?}", doc);

                Box::new(Ok(()).into_future())
            }
            // The index exists, but the doc wasn't found: map and index
            Ok(None) => {
                println!("indexing doc");

                put_doc(client, doc)
            }
            // No index: create it, then map and index
            Err(Error::Api(ApiError::IndexNotFound { .. })) => {
                println!("creating index and doc");

                let put_doc = put_index(client.clone()).and_then(|_| put_doc(client, doc));

                Box::new(put_doc)
            }
            // Something went wrong
            Err(e) => Box::new(Err(e).into_future()),
        }
    });

    Box::new(put_doc)
}

fn put_index(client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
    let create_index = client
        .index_create(sample_index())
        .send();

    let put_mapping = client
        .document_put_mapping::<MyType>(sample_index())
        .send()
        .map(|_| ());

    Box::new(create_index.and_then(|_| put_mapping))
}

fn put_doc(client: AsyncClient, doc: MyType) -> Box<Future<Item = (), Error = Error>> {
    let index_doc = client
        .document_index(sample_index(), id(doc.id), doc)
        .params(|p| p.url_param("refresh", true))
        .send()
        .map(|_| ());
    
    Box::new(index_doc)
}

fn search(client: AsyncClient, query: &'static str) -> Box<Future<Item = SearchResponse<MyType>, Error = Error>> {
    let search = client
        .search()
        .index(sample_index())
        .body(json!({
                "query": {
                    "query_string": {
                        "query": query
                    }
                }
          }))
        .send();
    
    Box::new(search)
}

fn main() {
    env_logger::init().unwrap();
    run().unwrap()
}
