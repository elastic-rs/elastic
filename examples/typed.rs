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
extern crate reqwest;

extern crate elastic;

use elastic::types::prelude::*;
use elastic::client::*;

const INDEX: &'static str = "typed_sample_index";

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String
}

#[derive(Default, Serialize)]
struct MyIndex {
    mappings: Mappings
}

#[derive(Default, Serialize)]
struct Mappings {
    mytype: Document<MyTypeMapping>
}

fn index_exists() -> IndicesExistsRequest<'static> {
    let index = Index::from(INDEX);

    IndicesExistsRequest::for_index(index)
}

fn put_index() -> IndicesCreateRequest<'static> {
    let index = Index::from(INDEX);

    IndicesCreateRequest::try_for_doc((index, MyIndex::default())).unwrap()
}

fn put_doc(doc: MyType) -> IndexRequest<'static> {
    let index = Index::from(INDEX);
    let id = Id::from(doc.id.to_string());

    IndexRequest::try_for_doc((index, id, &doc)).unwrap()
}

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

fn main() {
    // A reqwest HTTP client.
    let client = Client::new().unwrap();

    // The `params` includes the base node url (http://localhost:9200).
    let params = RequestParams::default();

    // Wait for refresh when indexing data.
    // Normally this isn't a good idea, but is ok for this example.
    let index_params = RequestParams::default()
        .url_param("refresh", true);

    let doc = MyType {
        id: 1,
        title: String::from("A title")
    };

    match client.elastic_req(&params, index_exists()).unwrap().status() {
        &reqwest::StatusCode::NotFound => {
            client.elastic_req(&params, put_index()).unwrap();
        },
        _ => ()
    }

    client.elastic_req(&index_params, put_doc(doc)).unwrap();

    let res: serde_json::Value = client
        .elastic_req(&params, search()).unwrap()
        .json().unwrap();

    println!("{:?}", res);
}