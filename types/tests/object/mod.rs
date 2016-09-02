#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::object_fixtures::*;

#[test]
fn get_type_name() {
	assert_eq!("simpletype", SimpleTypeMapping::name());
}

#[test]
fn serialise_mapping_type() {
	let ser = TypeMapper::to_string(SimpleTypeMapping).unwrap();

	let expected = json_str!({
		"properties":{
			"field1": {
				"type": "date",
				"format": "epoch_millis"
			},
			"field2": {
				"type": "nested",
				"properties": {
					"field": {
						"type": "integer"
					}
				}
			}
		}
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_as_value() {
	let value = TypeMapper::to_value(SimpleTypeMapping).unwrap();

	let ser = serde_json::ser::to_string(&value.lookup("properties.field1.type").unwrap()).unwrap();

	assert_eq!("\"date\"", &ser);
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