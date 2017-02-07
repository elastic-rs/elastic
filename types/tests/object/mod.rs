use serde_json::{self, Value};

use elastic_types::prelude::*;
use ::object_fixtures::*;

#[test]
fn get_type_name() {
    assert_eq!("simpletype", SimpleTypeMapping::name());
}

#[test]
fn serialise_mapping_type_with_mapper() {
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
fn serialise_mapping_type() {
    let ser = serde_json::to_string(&Document::from(SimpleTypeMapping)).unwrap();
    let expected = serde_json::to_string(&Document::from(SimpleTypeMapping)).unwrap();

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_as_value() {
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