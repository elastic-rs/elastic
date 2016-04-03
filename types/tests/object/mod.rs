#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::object_fixtures::*;

#[test]
fn serialise_mapping_type() {
	//Define an instance of our mapping type
	let mytype = MyType::default();
	let ser = TypeMapper::map_str(&mytype).unwrap();

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
	let ser = TypeMapper::map_str(&mytype).unwrap();

	let expected = json_str!({
		"properties":{
			"my_date":{
				"type":"date",
				"format":"basic_date_time"
			},
			"my_renamed_type":{
				"type":"object",
				"dynamic":true,
				"enabled":false,
				"include_in_all":true,
				"properties":{
					"my_date1":{
						"type":"date",
						"format":"basic_date_time"
					},
					"my_date2":{
						"type":"date",
						"boost":1.01,
						"doc_values":true,
						"include_in_all":false,
						"index":"no",
						"store":true,
						"format":"epoch_millis",
						"ignore_malformed":true,
						"null_value":"0",
						"precision_step":6
					},
					"my_string":{
						"type":"string"
					},"my_num":{
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

#[test]
fn serialise_mapping_dynamic() {
	let d_opts: Vec<String> = vec![
		Dynamic::True,
		Dynamic::False,
		Dynamic::Strict
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#"true"#,
		r#"false"#,
		r#""strict""#
	];

	let mut success = true;
	for i in 0..d_opts.len() {
		if expected_opts[i] != d_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}