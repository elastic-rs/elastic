extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file_as_response;

#[test]
fn success_parse_found_doc_response() {
    let s = load_file_as_response(200, "tests/samples/get_found.json");
    let deserialized = GetResponse::from_response(s).unwrap();

    let id = deserialized.source
        .unwrap()
        .as_object()
        .and_then(|src| src.get("id"))
        .and_then(|id| id.as_u64());

    assert!(deserialized.found);
    assert_eq!("testindex", deserialized.index);
    assert_eq!("testtype", deserialized.ty);
    assert_eq!(Some(8), deserialized.version);
    assert_eq!(Some(1), id);
}

#[test]
fn success_parse_not_found_doc_response() {
    let s = load_file_as_response(404, "tests/samples/get_not_found.json");
    let deserialized = GetResponse::from_response(s).unwrap();

    assert!(!deserialized.found);
}

#[test]
fn error_parse_index_not_found() {
    let s = load_file_as_response(404, "tests/samples/error_index_not_found.json");
    let deserialized = GetResponse::from_response(s).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::IndexNotFound { ref index })
        if index == "carrots" => true,
        _ => false
    };

    assert!(valid);
}