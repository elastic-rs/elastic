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
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tokio;

extern crate elastic;

use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use futures::{
    Future,
    IntoFuture,
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
    let client = AsyncClient::builder().build()?;

    // Create a document to index
    let doc = MyType {
        id: "1".to_owned(),
        title: String::from("A title"),
        timestamp: Date::now(),
    };

    // Check if the doc exists and index if it doesn't
    let index_future = ensure_indexed(client.clone(), doc);

    // Do a search request
    let search_future = search(client, "title");

    let res_future = index_future.and_then(|_| search_future).and_then(|res| {
        println!("{:?}", res);

        Ok(())
    });

    tokio::executor::current_thread::block_on_all(res_future)?;

    Ok(())
}

fn ensure_indexed(client: AsyncClient, doc: MyType) -> Box<dyn Future<Item = (), Error = Error>> {
    let get_res = client
        .document::<MyType>()
        .get(doc.id.clone())
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

fn put_index(client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
    let create_index = client.index(MyType::static_index()).create().send();

    let put_mapping = client.document::<MyType>().put_mapping().send().map(|_| ());

    Box::new(create_index.and_then(|_| put_mapping))
}

fn put_doc(client: AsyncClient, doc: MyType) -> Box<dyn Future<Item = (), Error = Error>> {
    let index_doc = client
        .document()
        .index(doc)
        .params_fluent(|p| p.url_param("refresh", true))
        .send()
        .map(|_| ());

    Box::new(index_doc)
}

fn search(
    client: AsyncClient,
    query: &'static str,
) -> Box<dyn Future<Item = SearchResponse<MyType>, Error = Error>> {
    let search = client
        .search()
        .index(MyType::static_index())
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
    env_logger::init();
    run().unwrap()
}
