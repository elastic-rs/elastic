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

	let expected = json!({
		"type": "date",
		"boost": 1.01,
		"doc_values": true,
		"include_in_all": false,
		"index": "no",
		"store": true,
		"format": "epoch_millis",
		"ignore_malformed": true,
		"null_value": "0",
		"precision_step": 6
	});

	assert_eq!(expected, ser);
}