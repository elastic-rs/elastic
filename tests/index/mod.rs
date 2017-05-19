extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use elastic_responses::error::*;
use ::load_file_as_response;

#[test]
fn success_parse_response() {
    let s = load_file_as_response(200, "tests/samples/index_success.json");
    let deserialized = s.into_response::<IndexResponse>().unwrap();

    assert!(deserialized.created);
    assert_eq!("testindex", deserialized.index);
    assert_eq!("testtype", deserialized.ty);
    assert_eq!("1", deserialized.id);
    assert_eq!(Some(1), deserialized.version);
}

#[test]
fn error_parse_mapping() {
    let s = load_file_as_response(404, "tests/samples/error_mapper_parsing.json");
    let deserialized = s.into_response::<IndexResponse>().unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::MapperParsing { ref reason })
        if reason == "failed to parse, document is empty" => true,
        _ => false
    };

    assert!(valid);
}