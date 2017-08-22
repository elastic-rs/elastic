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
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate elastic;

use elastic::error::*;
use elastic::prelude::*;

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String,
    timestamp: Date<DefaultDateFormat>,
}

fn main() {
    let mut core = Core::new()?;

    // A HTTP client and request parameters
    let client = AsyncClientBuilder::new().build(&core.handle())?;

    // Create a document to index
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now(),
    };

    // Check if the doc exists and index if it doesn't
    let index_future = ensure_indexed(&client, doc);

    // Do a search request
    let search_future = index_future.and_then(|| search(&client, "title"));

    let res_future = search_future.and_then(|res| {
        println!("{:?}", res);

        Ok(())
    });

    core.run(res_future)?;
}

fn sample_index() -> Index<'static> {
    Index::from("typed_sample_index")
}

fn ensure_indexed(client: &Client, doc: MyType) {
    let get_res = client
        .get_document::<MyType>(sample_index(), id(doc.id))
        .send();

    match get_res.map(|res| res.into_document()) {
        // The doc was found: no need to index
        Ok(Some(doc)) => {
            println!("document already indexed: {:?}", doc);
        }
        // The index exists, but the doc wasn't found: map and index
        Ok(None) => {
            println!("indexing doc");

            put_doc(client, doc);
        }
        // No index: create it, then map and index
        Err(Error::Api(ApiError::IndexNotFound { .. })) => {
            println!("creating index and doc");

            put_index(client);
            put_doc(client, doc);
        }
        // Something went wrong: panic
        Err(e) => panic!("{:?}", e),
    }
}

fn put_index(client: &Client) {
    client.create_index(sample_index()).send().unwrap();
    client.put_mapping::<MyType>(sample_index()).send().unwrap();
}

fn put_doc(client: &Client, doc: MyType) {
    client
        .index_document(sample_index(), id(doc.id), doc)
        .params(|p| p.url_param("refresh", true))
        .send()?;
}

fn search(client: &Client, query: &'static str) -> SearchResponse<MyType> {
    client
        .search()
        .index(sample_index())
        .body(json!({
                "query": {
                    "query_string": {
                        "query": query
                    }
                }
          }))
        .send()
}
