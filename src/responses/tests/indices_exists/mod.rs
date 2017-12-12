extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;

#[test]
fn success_parse_response_exists() {
    let deserialized = parse::<IndicesExistsResponse>().from_slice(200, b"").unwrap();

    assert!(deserialized.exists());
}

#[test]
fn success_parse_response_not_exists() {
    let deserialized = parse::<IndicesExistsResponse>().from_slice(404, b"").unwrap();

    assert!(!deserialized.exists());
}
