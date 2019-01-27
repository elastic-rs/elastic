extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use load_file;

#[test]
fn success_parse_ping_response() {
    let f = load_file("tests/samples/ping_success.json");
    let deserialized = parse::<PingResponse>()
        .from_reader(StatusCode::OK, f)
        .unwrap();

    assert_eq!("Scorcher", deserialized.name());
}
