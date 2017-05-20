extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file;

#[test]
fn success_parse_found_doc_response() {
    let f = load_file("tests/samples/get_found.json");
    let deserialized = parse::<GetResponse>().from_reader(200, f).unwrap();

    let id = deserialized.source
        .unwrap()
        .as_object()
        .and_then(|src| src.get("id"))
        .and_then(|id| id.as_u64());

    assert!(deserialized.found);
    assert_eq!("testindex", deserialized.index);
    assert_eq!("testtype", deserialized.ty);
    assert_eq!("1", deserialized.id);
    assert_eq!(Some(8), deserialized.version);
    assert_eq!(Some(1), id);
}

#[test]
fn success_parse_not_found_doc_response() {
    let f = load_file("tests/samples/get_not_found.json");
    let deserialized = parse::<GetResponse>().from_reader(404, f).unwrap();

    assert!(!deserialized.found);
}

#[test]
fn error_parse_index_not_found() {
    let f = load_file("tests/samples/error_index_not_found.json");
    let deserialized = parse::<GetResponse>().from_reader(404, f).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::IndexNotFound { ref index })
        if index == "carrots" => true,
        _ => false
    };

    assert!(valid);
}