#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::boolean_fixtures::*;

#[test]
fn serialise_mapping_default() {
	let mapping = DefaultBooleanMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "boolean"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyBooleanMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "boolean",
		"boost": 1.01,
		"doc_values": true,
		"index": "no",
		"store": true,
		"null_value": false
	});

	assert_eq!(expected, ser);
}
