extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;

#[test]
fn success_parse_response_exists() {
    let deserialized = parse::<IndicesExistsResponse>()
        .from_slice(StatusCode::OK, b"")
        .unwrap();

    assert!(deserialized.exists());
}

#[test]
fn success_parse_response_not_exists() {
    let deserialized = parse::<IndicesExistsResponse>()
        .from_slice(StatusCode::NOT_FOUND, b"")
        .unwrap();

    assert!(!deserialized.exists());
}
