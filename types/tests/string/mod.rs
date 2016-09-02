#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::string::prelude::*;
use ::string_fixtures::*;

#[test]
fn can_change_keyword_mapping() {
	fn takes_custom_mapping(_: Keyword<MyKeywordMapping>) -> bool {
		true
	}

	let string: Keyword<DefaultKeywordMapping> = Keyword::new("stuff");

	assert!(takes_custom_mapping(string.remap()));
}

#[test]
fn serialise_elastic_keyword() {
	let string: Keyword<DefaultKeywordMapping> = Keyword::new("my string");

	let ser = serde_json::to_string(&string).unwrap();

	assert_eq!(r#""my string""#, ser);
}

#[test]
fn deserialise_elastic_keyword() {
	let string: Keyword<DefaultKeywordMapping> = serde_json::from_str(r#""my string""#).unwrap();

	assert_eq!("my string", string);
}

#[test]
fn can_change_text_mapping() {
	fn takes_custom_mapping(_: Text<MyTextMapping>) -> bool {
		true
	}

	let string: Text<DefaultTextMapping> = Text::new("stuff");

	assert!(takes_custom_mapping(string.remap()));
}

#[test]
fn serialise_elastic_text() {
	let string: Text<DefaultTextMapping> = Text::new("my string");

	let ser = serde_json::to_string(&string).unwrap();

	assert_eq!(r#""my string""#, ser);
}

#[test]
fn deserialise_elastic_text() {
	let string: Text<DefaultTextMapping> = serde_json::from_str(r#""my string""#).unwrap();

	assert_eq!("my string", string);
}