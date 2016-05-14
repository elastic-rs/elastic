#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::boolean::mapping::*;
use elastic_types::boolean::prelude::*;
use ::boolean_fixtures::*;

#[test]
fn can_change_boolean_mapping() {
	fn takes_custom_mapping(boolean: ElasticBoolean<MyBooleanMapping>) -> bool {
		true
	}

	let boolean: ElasticBoolean<DefaultBooleanMapping> = ElasticBoolean::new(true);

	assert!(takes_custom_mapping(boolean.remap()));
}

#[test]
fn serialise_elastic_string() {
	let boolean: ElasticBoolean<DefaultBooleanMapping> = ElasticBoolean::new(true);

	let ser = serde_json::to_string(&boolean).unwrap();

	assert_eq!("true", ser);
}

#[test]
fn deserialise_elastic_string() {
	let boolean: ElasticBoolean<DefaultBooleanMapping> = serde_json::from_str("true").unwrap();

	assert_eq!(true, boolean);
}
