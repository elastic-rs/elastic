use serde_json;
use elastic_types::prelude::*;
use ::object_fixtures as expected_types;
use ::object_macro_fixtures::*;

#[test]
fn get_default_type_name() {
    assert_eq!("simpletype", SimpleType::name());
}

#[test]
fn get_custom_type_name() {
    assert_eq!("renamed_type", CustomType::name());
}

#[test]
fn serialise_mapping_type() {
    let ser = serde_json::to_string(&Document::from(SimpleTypeMapping)).unwrap();

    let expected = serde_json::to_string(&Document::from(expected_types::SimpleTypeMapping)).unwrap();

    assert_eq!(expected, ser);
}

#[test]
fn serialise_custom_mapping_type() {
    let ser = serde_json::to_string(&Document::from(CustomTypeMapping)).unwrap();

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
fn serialise_mapping_with_wrapped_types() {
    let ser = serde_json::to_string(&Document::from(WrappedMapping)).unwrap();

    let expected = json_str!({
        "properties":{
            "field1": {
                "type": "integer"
            },
            "field2": {
                "type": "boolean"
            },
            "field3": {
                "type":"text",
                "fields":{
                    "keyword":{
                        "type":"keyword",
                        "ignore_above":256
                    }
                }
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