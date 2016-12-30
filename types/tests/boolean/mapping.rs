use serde;
use serde_json;
use elastic_types;

use elastic_types::prelude::*;
use ::boolean_fixtures::*;

#[test]
fn bool_has_default_mapping() {
    assert_eq!(DefaultBooleanMapping, bool::mapping());
}

#[test]
fn serialise_mapping_default() {
    let ser = serde_json::to_string(&Field::from(DefaultBooleanMapping)).unwrap();

    let expected = json_str!({
        "type": "boolean"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
    let ser = serde_json::to_string(&Field::from(MyBooleanMapping)).unwrap();

    let expected = json_str!({
        "type": "boolean",
        "boost": 1.01,
        "doc_values": true,
        "index": false,
        "store": true,
        "null_value": false
    });

    assert_eq!(expected, ser);
}
