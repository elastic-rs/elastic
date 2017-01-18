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
use elastic::error::*;
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

    ensure_indexed(&client, doc);

    let res = search(&client);

    println!("{:?}", res);
}

fn ensure_indexed(client: &Client, doc: MyType) {
    let req = GetRequest::for_index_ty_id(INDEX, MyType::name(), doc.id.to_string());

    let get_res = client.request(req)
                        .send()
                        .and_then(|res| res.doc_response::<MyType>());

    match get_res {
        // The doc was found: no need to index
        Ok(GetDocResponseOf { source: Some(doc), .. }) => {
            println!("document already indexed: {:?}", doc);
        },
        // The index exists, but the doc wasn't found: map and index
        Ok(_) => {
            println!("indexing doc");

            unimplemented!();
        },
        // No index: create it, then map and index
        Err(Error(ErrorKind::Api(ApiError::IndexNotFound { .. }), _)) => {
            println!("creating index");

            unimplemented!();
        },
        // Something went wrong: panic
        Err(e) => panic!(e)
    }
}

fn search(client: &Client) -> QueryResponseOf<Hit<MyType>> {
    let body = json_str!({
        query: {
            query_string: {
                query: "title"
            }
        }
    });

    let req = SearchRequest::for_index(INDEX, body);

    client.request(req).send()
                       .and_then(|res| res.query_response())
                       .unwrap()
}