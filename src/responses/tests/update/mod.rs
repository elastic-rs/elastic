// TODO:
// - error: action_request_validation
// - error: document_missing_exception

extern crate elastic_responses;
extern crate serde_json;

use serde_json::Value;
use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file;

#[test]
fn success_parse_updated_doc_response() {
    let f = load_file("tests/samples/update_updated.json");
    let deserialized = parse::<UpdateResponse>().from_reader(200, f).unwrap();

    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(8), deserialized.version());

    assert!(deserialized.updated());
}

#[test]
fn success_parse_noop_doc_response() {
    let f = load_file("tests/samples/update_noop.json");
    let deserialized = parse::<UpdateResponse>().from_reader(200, f).unwrap();

    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(8), deserialized.version());

    assert!(!deserialized.updated());
}
