#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::boolean::mapping::*;
use elastic_types::boolean::prelude::*;

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