#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_derive, elastic_date_macros))]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::object_fixtures as expected_types;
use ::object_macro_fixtures::*;

#[test]
fn get_default_type_name() {
	assert_eq!("simpletype", SimpleTypeMapping::name());
}

#[test]
fn get_custom_type_name() {
	assert_eq!("renamed_type", CustomTypeMapping::name());
}

#[test]
fn serialise_mapping_type() {
	let ser = TypeMapper::to_string(SimpleTypeMapping).unwrap();

	let expected = TypeMapper::to_string(expected_types::SimpleTypeMapping).unwrap();

	assert_eq!(expected, ser);
}

#[test]
fn serialise_custom_mapping_type() {
	let ser = TypeMapper::to_string(CustomTypeMapping).unwrap();

	let expected = json_str!({
		"properties":{
			"field": {
				"type":"integer"
			},
			"renamed_field": {
				"type":"integer"
			}
		}
	});

	assert_eq!(expected, ser);
}