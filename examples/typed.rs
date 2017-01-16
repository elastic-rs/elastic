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
extern crate json_str;
#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate elastic;

use elastic::http;
use elastic::prelude::*;

const INDEX: &'static str = "typed_sample_index";

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String,
    timestamp: Date<DefaultDateFormat>
}

fn main() {
    // A HTTP client and request parameters
    let client = Client::new(RequestParams::default()).unwrap();

    // Create a document to index
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    create_index_if_new(&client);

    index_doc(&client, doc);

    let res = search_docs(&client);

    println!("{:?}", res);
}

// Create the index if it doesn't already exist
fn index_exists_request() -> IndicesExistsRequest<'static> {
    let index = Index::from(INDEX);

    IndicesExistsRequest::for_index(index)
}

fn put_index_request() -> IndicesCreateRequest<'static> {
    let index = Index::from(INDEX);

    IndicesCreateRequest::for_index(index, Body::none())
}

fn create_index_if_new(client: &Client) {
    let exists = client.request(index_exists_request())
        .send()
        .and_then(|res| {
            match *res.raw().status() {
                http::StatusCode::NotFound => Ok(false),
                _ => Ok(true)
            }
        })
        .unwrap();

    if !exists {
        client.request(put_index_request()).send().unwrap();
    }
}

// Update the document mapping and index our document
fn map_doc_request(doc: &MyType) -> IndicesPutMappingRequest<'static> {
    let index = Index::from(INDEX);

    IndicesPutMappingRequest::try_for_doc((index, doc)).unwrap()
}

fn put_doc_request(doc: &MyType) -> IndexRequest<'static> {
    let index = Index::from(INDEX);
    let id = Id::from(doc.id.to_string());

    IndexRequest::try_for_doc((index, id, doc)).unwrap()
}

fn index_doc(client: &Client, doc: MyType) {
    client.request(map_doc_request(&doc)).send().unwrap();

    // Wait for refresh when indexing so we can search right away
    client.request(put_doc_request(&doc))
        .params(|params| params.url_param("refresh", true))
        .send()
        .unwrap();
}

// Search for documents in the index
fn search() -> SearchRequest<'static> {
    let index = Index::from(INDEX);

    let body = json_str!({
        query: {
            query_string: {
                query: "title"
            }
        }
    });

    SearchRequest::for_index(index, body)
}

fn search_docs(client: &Client) -> serde_json::Value {
    client.request(search()).send().and_then(|res| res.json()).unwrap()
}