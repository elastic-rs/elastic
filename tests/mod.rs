#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate elastic;

use elastic::client::{self, FromDoc};
use elastic::types::prelude::*;

#[derive(Serialize, ElasticType)]
struct MyType {
    id: i32,
    title: &'static str
}

#[test]
fn index_request_from_doc() {
    let doc = MyType {
        id: 1,
        title: "A title"
    };

    let index = client::Index::from("test_index");

    let req = client::IndexRequest::from_doc((index, &doc));
}