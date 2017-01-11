//! A basic typed example.
//! 
//! NOTE: This sample expects you have a node running on `localhost:9200`.

#[macro_use]
extern crate json_str;
#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate elastic;

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
    let client = Client::new().unwrap();
    let params = RequestParams::default();

    // Create a document to index
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    create_index_if_new(&client, &params);

    index_doc(&client, &params, doc);

    let res = search_docs(&client, &params);

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

fn create_index_if_new(client: &Client, params: &RequestParams) {
    let exists = client.request(&params, index_exists_request())
        .and_then(|res| {
            match *res.raw().status() {
                StatusCode::NotFound => Ok(false),
                _ => Ok(true)
            }
        })
        .unwrap();

    if !exists {
        client.request(&params, put_index_request()).unwrap();
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

fn index_doc(client: &Client, params: &RequestParams, doc: MyType) {
    client.request(&params, map_doc_request(&doc)).unwrap();

    // Wait for refresh when indexing so we can search right away
    let params = params.clone().url_param("refresh", true);

    client.request(&params, put_doc_request(&doc)).unwrap();
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

fn search_docs(client: &Client, params: &RequestParams) -> serde_json::Value {
    client.request(&params, search()).and_then(|res| res.json()).unwrap()
}