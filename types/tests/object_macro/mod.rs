use serde_json;
use elastic_types::mapping::prelude::*;
use ::object_fixtures as expected_types;
use ::object_macro_fixtures::*;

#[test]
fn get_default_type_name() {
	assert_eq!("simpletype", SimpleTypeMapping::name());
}

#[test]
fn get_custom_type_name() {
	assert_eq!("renamed_type", CustomTypeMapping::name());
}

#[test]
fn serialise_mapping_type() {
	let ser = TypeMapper::to_string(SimpleTypeMapping).unwrap();

	let expected = TypeMapper::to_string(expected_types::SimpleTypeMapping).unwrap();

	assert_eq!(expected, ser);
}

#[test]
fn serialise_custom_mapping_type() {
	let ser = TypeMapper::to_string(CustomTypeMapping).unwrap();

	let expected = json_str!({
		"properties":{
			"field": {
				"type":"integer"
			},
			"renamed_field": {
				"type":"integer"
			}
		}
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_index_mapping() {
	let ser = serde_json::to_string(&Index::default()).unwrap();

	let expected = json_str!({
		"mappings": {
			"simpletype": {
				"properties": {
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
			}
		}
	});

	assert_eq!(expected, ser);
}