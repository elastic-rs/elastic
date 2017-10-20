extern crate elastic_responses;
extern crate serde_json;

use serde_json::Value;
use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file;

#[test]
fn success_parse_found_response() {
    let f = load_file("tests/samples/delete_found.json");
    let deserialized = parse::<DeleteResponse>().from_reader(200, f).unwrap();

    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(8), deserialized.version());

    assert!(deserialized.found());
    assert!(deserialized.deleted());
}

#[test]
fn success_parse_not_found_response() {
    let f = load_file("tests/samples/delete_not_found.json");
    let deserialized = parse::<DeleteResponse>().from_reader(404, f).unwrap();

    assert!(!deserialized.found());
    assert!(!deserialized.deleted());
}
