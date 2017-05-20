extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file;

#[test]
fn success_parse_response() {
    let f = load_file("tests/samples/index_success.json");
    let deserialized = parse::<IndexResponse>().from_read(200, f).unwrap();

    assert!(deserialized.created);
    assert_eq!("testindex", deserialized.index);
    assert_eq!("testtype", deserialized.ty);
    assert_eq!("1", deserialized.id);
    assert_eq!(Some(1), deserialized.version);
}

#[test]
fn error_parse_mapping() {
    let f = load_file("tests/samples/error_mapper_parsing.json");
    let deserialized = parse::<IndexResponse>().from_read(404, f).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::MapperParsing { ref reason })
        if reason == "failed to parse, document is empty" => true,
        _ => false
    };

    assert!(valid);
}