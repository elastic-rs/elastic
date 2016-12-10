use serde_json;

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
	let mut template = IndexTemplate::new("test_template", "data-*", SimpleTypeMapping).unwrap();
	template.add_mapping(SimpleNestedTypeMapping).unwrap();
	template.order = 3;

	let ser = serde_json::to_string(&template).unwrap();

	let expected = json_str!({
		"template": "data-*",
		"order": 3,
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