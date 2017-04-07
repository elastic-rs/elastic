extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use ::load_file_as_response;

#[test]
fn success_parse_ping_response() {
    let s = load_file_as_response(200, "tests/samples/ping_success.json");
    let deserialized = PingResponse::from_response(s).unwrap();

    assert_eq!("Scorcher", &deserialized.name);
}
