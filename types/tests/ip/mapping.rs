use serde;
use serde_json;
use elastic_types;

use std::net::Ipv4Addr;
use elastic_types::prelude::*;
use ::ip_fixtures::*;

#[test]
fn ipv4addr_has_default_mapping() {
    assert_eq!(DefaultIpMapping, Ipv4Addr::mapping());
}

#[test]
fn serialise_mapping_default() {
    let ser = serde_json::to_string(&Field::from(DefaultIpMapping)).unwrap();

    let expected = json_str!({
        "type": "ip"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
    let ser = serde_json::to_string(&Field::from(MyIpMapping)).unwrap();

    let expected = json_str!({
        "type": "ip",
        "boost": 1.01,
        "doc_values": true,
        "index": false,
        "store": true,
        "null_value": "127.0.0.1"
    });

    assert_eq!(expected, ser);
}
