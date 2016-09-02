#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use std::net::Ipv4Addr;
use elastic_types::mapping::prelude::*;
use ::ip_fixtures::*;

#[test]
fn ipv4addr_has_default_mapping() {
	assert_eq!(DefaultIpMapping, Ipv4Addr::mapping());
}

#[test]
fn serialise_mapping_default() {
	let ser = FieldMapper::to_string(DefaultIpMapping).unwrap();

	let expected = json_str!({
		"type": "ip"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let ser = FieldMapper::to_string(MyIpMapping).unwrap();

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
