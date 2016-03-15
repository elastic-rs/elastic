#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::prelude::*;
use ::date_fixtures::*;

#[test]
fn serialise_mapping_default() {
	let mapping = DefaultDateMapping::<BasicDateTime>::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json!({
		"type": "date",
		"format": "basic_date_time"
	});
	
	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyDateMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = mydatemapping_json();

	assert_eq!(expected, ser);
}