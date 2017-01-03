use serde_json;

use elastic_types::prelude::*;
use ::boolean_fixtures::*;

#[test]
fn can_change_boolean_mapping() {
    fn takes_custom_mapping(_: Boolean<MyBooleanMapping>) -> bool {
        true
    }

    let boolean: Boolean<DefaultBooleanMapping> = Boolean::new(true);

    assert!(takes_custom_mapping(boolean.remap()));
}

#[test]
fn serialise_elastic_boolean() {
    let boolean: Boolean<DefaultBooleanMapping> = Boolean::new(true);

    let ser = serde_json::to_string(&boolean).unwrap();

    assert_eq!("true", ser);
}

#[test]
fn deserialise_elastic_boolean() {
    let boolean: Boolean<DefaultBooleanMapping> = serde_json::from_str("true").unwrap();

    assert_eq!(true, boolean);
}
