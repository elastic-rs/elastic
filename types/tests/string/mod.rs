#![cfg(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate json_str;
#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate elastic_date_macros;

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::string::mapping::*;
use elastic_types::string::prelude::*;

#[test]
fn can_change_string_mapping() {
	panic!("implement")
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
