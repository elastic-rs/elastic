extern crate elastic_responses;
extern crate serde_json;

use elastic_responses::*;
use ::load_file;

#[test]
fn success_parse_command_response() {
    let f = load_file("tests/samples/acknowledged.json");
    let deserialized = parse::<CommandResponse>().from_reader(200, f).unwrap();

    assert!(deserialized.acknowledged());
}
