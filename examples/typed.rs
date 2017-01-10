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

extern crate elastic;

use elastic::client::{
    self, 
    ElasticClient, 
    ResponseOf, 
    IndexRequest, 
    SearchRequest, 
    Index, 
    Id, 
    TryForDoc
};

const INDEX: &'static str = "typed_sample_index";

#[derive(Debug, Serialize, Deserialize, ElasticType)]
struct MyType {
    id: i32,
    title: String
}

fn index_req(doc: MyType) -> IndexRequest<'static> {
    let index = Index::from(INDEX);
    let id = Id::from(doc.id.to_string());

    IndexRequest::try_for_doc((index, id, &doc)).unwrap()
}

fn search_req() -> SearchRequest<'static> {
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
    let client = client::Client::new().unwrap();

    // The `params` includes the base node url (http://localhost:9200).
    // Wait for refresh when indexing data.
    // Normally this isn't a good idea, but is ok for this example.
    let params = client::RequestParams::default()
        .url_param("refresh", true);

    let doc = MyType {
        id: 1,
        title: String::from("A title")
    };

    client.elastic_req(&params, index_req(doc)).unwrap();

    let search_response: ResponseOf<MyType> = client
        .elastic_req(&params, search_req()).unwrap()
        .json().unwrap();

    for hit in search_response.hits() {
        println!("{:?}", hit);
    }
}