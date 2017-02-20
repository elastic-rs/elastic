#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate elastic;

use elastic::prelude::*;

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

    let index = Index::from("test_index");

    let req = IndexRequest::try_for_doc((index, &doc));

    assert!(req.is_ok());
}

#[test]
fn index_request_from_doc_with_id() {
    let doc = MyType {
        id: 1,
        title: "A title"
    };

    let index = Index::from("test_index");
    let id = Id::from(doc.id.to_string());

    let req = IndexRequest::try_for_doc((index, id, &doc));

    assert!(req.is_ok());
}

#[test]
fn mapping_request_from_doc() {
	let doc = MyType {
		id: 1,
		title: "A title"
	};

	let index = Index::from("test_index");

	let req = IndicesPutMappingRequest::try_for_doc((index, &doc));

	assert!(req.is_ok());
}

#[test]
fn mapping_request_from_mapping() {
	let index = Index::from("test_index");

	let req = IndicesPutMappingRequest::try_for_mapping((index, MyType::mapping()));

	assert!(req.is_ok());
}

#[test]
fn body_from_doc() {
	let doc = MyType {
		id: 1,
		title: "A title"
	};

	let body = Body::try_for_doc(&doc);

	assert!(body.is_ok());
}