#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, json_str, elastic_types_derive, elastic_date_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::boolean_fixtures::*;

#[test]
fn bool_has_default_mapping() {
	assert_eq!(DefaultBooleanMapping, bool::mapping());
}

#[test]
fn serialise_mapping_default() {
	let ser = FieldMapper::to_string(DefaultBooleanMapping).unwrap();

	let expected = json_str!({
		"type": "boolean"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let ser = FieldMapper::to_string(MyBooleanMapping).unwrap();

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
