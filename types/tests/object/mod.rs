#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use serde_json::ser::Serializer;
use elastic_types::mapping::prelude::*;
use elastic_types::date::prelude::*;
use elastic_types::string::prelude::*;
use ::date_fixtures::*;
use ::object_fixtures::*;

#[test]
fn serialise_mapping_type() {
	//Define an instance of our mapping type
	let mytype = MyType::default();

	//Build a serialiser and use the mapper to serialise the mapping for the given type
	let mut writer = Vec::with_capacity(128);
	{
		let mut ser = Serializer::new(&mut writer);
		let _ = TypeMapper::map(&mytype, &mut ser).unwrap();
	}
	let ser = String::from_utf8(writer).unwrap();

	let expected = json_str!({
		"properties": {
			"my_date1":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_date2": {
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
			},
			"my_string":{
				"type":"string"
			},
			"my_num":{
				"type":"object"
			}
		}
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_type_as_nested() {
	//Define an instance of our mapping type
	let mytype = MyOtherType::default();

	//Build a serialiser and use the mapper to serialise the mapping for the given type
	let mut writer = Vec::with_capacity(256);
	{
		let mut ser = Serializer::new(&mut writer);
		let _ = TypeMapper::map(&mytype, &mut ser).unwrap();
	}
	let ser = String::from_utf8(writer).unwrap();

	let expected = json_str!({
		"properties": {
			"my_date":{
				"type":"date",
				"format":"basic_date_time"
			}, 
			"my_type": {
				"type": "nested",
				"properties": {
					"my_date1":{
						"type":"date",
						"format":"basic_date_time"
					},
					"my_date2": {
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
					},
					"my_string":{
						"type":"string"
					},
					"my_num":{
						"type":"object"
					}
				}
			},
			"my_num":{
				"type":"object"
			}
		}
	});

	assert_eq!(expected, ser);
}