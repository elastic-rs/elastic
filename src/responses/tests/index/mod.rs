extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::error::*;
use elastic_responses::*;
use load_file;

#[test]
fn success_parse_response() {
    let f = load_file("tests/samples/index_success.json");
    let deserialized = parse::<IndexResponse>().from_reader(StatusCode::OK, f).unwrap();

    assert!(deserialized.created());
    assert_eq!("testindex", deserialized.index());
    assert_eq!("testtype", deserialized.ty());
    assert_eq!("1", deserialized.id());
    assert_eq!(Some(1), deserialized.version());
}

#[test]
fn error_parse_index_already_exists() {
    let f = load_file("tests/samples/error_index_already_exists.json");
    let deserialized = parse::<IndexResponse>().from_reader(StatusCode::BAD_REQUEST, f).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::IndexAlreadyExists { ref index }) if index == "carrots" => true,
        _ => false,
    };

    assert!(valid);
}
