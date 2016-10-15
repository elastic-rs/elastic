#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_derive, elastic_date_macros))]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::number_fixtures::*;

#[test]
fn i32_has_default_mapping() {
	assert_eq!(DefaultIntegerMapping, i32::mapping());
}

#[test]
fn i64_has_default_mapping() {
	assert_eq!(DefaultLongMapping, i64::mapping());
}

#[test]
fn i16_has_default_mapping() {
	assert_eq!(DefaultShortMapping, i16::mapping());
}

#[test]
fn i8_has_default_mapping() {
	assert_eq!(DefaultByteMapping, i8::mapping());
}

#[test]
fn f32_has_default_mapping() {
	assert_eq!(DefaultFloatMapping, f32::mapping());
}

#[test]
fn f64_has_default_mapping() {
	assert_eq!(DefaultDoubleMapping, f64::mapping());
}

#[test]
fn serialise_mapping_integer_default() {
	let ser = FieldMapper::to_string(DefaultIntegerMapping).unwrap();

	let expected = json_str!({
		"type": "integer"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_integer_custom() {
	let ser = FieldMapper::to_string(MyIntegerMapping).unwrap();

	let expected = json_str!({
		"type": "integer",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 42,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_long_default() {
	let ser = FieldMapper::to_string(DefaultLongMapping).unwrap();

	let expected = json_str!({
		"type": "long"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_long_custom() {
	let ser = FieldMapper::to_string(MyLongMapping).unwrap();

	let expected = json_str!({
		"type": "long",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": -42,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_short_default() {
	let ser = FieldMapper::to_string(DefaultShortMapping).unwrap();

	let expected = json_str!({
		"type": "short"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_short_custom() {
	let ser = FieldMapper::to_string(MyShortMapping).unwrap();

	let expected = json_str!({
		"type": "short",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 42,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_byte_default() {
	let ser = FieldMapper::to_string(DefaultByteMapping).unwrap();

	let expected = json_str!({
		"type": "byte"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_byte_custom() {
	let ser = FieldMapper::to_string(MyByteMapping).unwrap();

	let expected = json_str!({
		"type": "byte",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 1,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_double_default() {
	let ser = FieldMapper::to_string(DefaultDoubleMapping).unwrap();

	let expected = json_str!({
		"type": "double"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_double_custom() {
	let ser = FieldMapper::to_string(MyDoubleMapping).unwrap();

	let expected = json_str!({
		"type": "double",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": -0.00002,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_float_default() {
	let ser = FieldMapper::to_string(DefaultFloatMapping).unwrap();

	let expected = json_str!({
		"type": "float"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_float_custom() {
	let ser = FieldMapper::to_string(MyFloatMapping).unwrap();

	let expected = json_str!({
		"type": "float",
		"coerce": true,
		"boost": 1.1,
		"doc_values": false,
		"ignore_malformed": true,
		"include_in_all": true,
		"null_value": 1.04,
		"store": true
	});

	assert_eq!(expected, ser);
}
