use serde_json::{self, Value};

use elastic_types::prelude::*;
use ::object_fixtures::*;

#[test]
fn use_doc_as_generic_without_supplying_mapping_param() {
    fn use_document<TDocument>() where TDocument: DocumentType {
        assert!(true);
    }

    use_document::<SimpleType>();
}

#[test]
fn get_type_name() {
    assert_eq!("simpletype", SimpleTypeMapping::name());
}

#[test]
fn get_default_type_name() {
    assert_eq!("simpletype", SimpleType::name());
}

#[test]
fn get_custom_type_name() {
    assert_eq!("renamed_type", CustomType::name());
}

#[test]
fn derive_custom_type_mapping() {
    assert_eq!(ManualCustomTypeMapping, CustomType::mapping());
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
            },
            "field4": {
                "type": "nested"
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

#[test]
fn serialise_document() {
    let ser = serde_json::to_string(&Document::from(SimpleTypeMapping)).unwrap();

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
fn serialise_document_with_no_props() {
    let ser = serde_json::to_string(&Document::from(NoPropsMapping)).unwrap();

    let expected = json_str!({
        "properties": {
            
        }
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_document_for_custom_mapping() {
    let ser = serde_json::to_string(&Document::from(ManualCustomTypeMapping)).unwrap();

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
fn serialise_document_for_mapping_as_value() {
    let value = match serde_json::to_value(&Document::from(SimpleTypeMapping)) {
        Ok(Value::Object(value)) => value,
        _ => panic!("expected Ok(Value::Map)")
    };

    let field1_type = &value["properties"]["field1"]["type"];

    let ser = serde_json::ser::to_string(&field1_type).unwrap();

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