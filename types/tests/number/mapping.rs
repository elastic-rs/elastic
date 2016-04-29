#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::number_fixtures::*;

#[test]
fn serialise_mapping_integer_default() {
	let mapping = DefaultIntegerMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "integer"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_integer_custom() {
	let mapping = MyIntegerMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "integer",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 42,
		"precision_step": 2147483647,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_long_default() {
	let mapping = DefaultLongMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "long"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_long_custom() {
	let mapping = MyLongMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "long",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 42,
		"precision_step": 2147483647,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_short_default() {
	let mapping = DefaultShortMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "short"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_short_custom() {
	let mapping = MyShortMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "short",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": -42,
		"precision_step": 2147483647,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_byte_default() {
	let mapping = DefaultByteMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "byte"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_byte_custom() {
	let mapping = MyByteMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "byte",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 1,
		"precision_step": 2147483647,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_double_default() {
	let mapping = DefaultDoubleMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "double"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_double_custom() {
	let mapping = MyDoubleMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "double",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": -0.00002,
		"precision_step": 2147483647,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_float_default() {
	let mapping = DefaultFloatMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "float"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_float_custom() {
	let mapping = MyFloatMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "float",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 1.04,
		"precision_step": 2147483647,
		"store": true
	});

	assert_eq!(expected, ser);
}