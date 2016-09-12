#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::prelude::*;
use ::object_fixtures::*;

#[test]
fn get_template_name() {
	let t = IndexTemplate::new("test_template", "", SimpleTypeMapping).unwrap();

	assert_eq!("test_template", t.name);
}

#[test]
fn get_template_pattern() {
	let t = IndexTemplate::new("", "data-*", SimpleTypeMapping).unwrap();

	assert_eq!("data-*", t.template);
}

#[test]
fn serialise_index_template() {
	let mut template = IndexTemplate::new("test_template", "*", SimpleTypeMapping).unwrap();
	template.add_mapping(SimpleNestedTypeMapping).unwrap();
	template.order = 0;

	let ser = serde_json::to_string(&template).unwrap();

	let expected = json_str!({
		"template": "*",
		"order": 0,
		"mappings": {
			"simplenestedtype": {
				"properties": {
					"field": {
						"type": "integer"
					}
				}
			},
			"simpletype": {
				"properties": {
					"field1": {
						"format": "epoch_millis",
						"type": "date"
					},
					"field2": {
						"properties": {
							"field": {
								"type": "integer"
							}
						},
						"type": "nested"
					}
				}
			}
		}
	});

	assert_eq!(expected, ser);
}