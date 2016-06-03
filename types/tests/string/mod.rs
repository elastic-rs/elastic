#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::string::mapping::*;
use elastic_types::string::prelude::*;
use ::string_fixtures::*;

#[test]
fn can_change_string_mapping() {
	fn takes_custom_mapping(_: ElasticString<MyStringMapping>) -> bool {
		true
	}

	let string: ElasticString<DefaultStringMapping> = ElasticString::new("stuff");

	assert!(takes_custom_mapping(string.remap()));
}

#[test]
fn serialise_elastic_string() {
	let string: ElasticString<DefaultStringMapping> = ElasticString::new("my string");

	let ser = serde_json::to_string(&string).unwrap();

	assert_eq!(r#""my string""#, ser);
}

#[test]
fn deserialise_elastic_string() {
	let string: ElasticString<DefaultStringMapping> = serde_json::from_str(r#""my string""#).unwrap();

	assert_eq!("my string", string);
}
