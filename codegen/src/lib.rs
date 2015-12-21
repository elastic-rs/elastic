#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate regex;

pub mod parse;
pub mod ast;

#[test]
fn can_parse_from_file() {
	let mut f = std::fs::File::open("api/bulk.json").unwrap();

	let _ = parse::from_reader(&mut f).unwrap();
}

#[test]
fn can_replace_type_fields_in_json() {
	let rpl = parse::replace_type_fields(r#""url": "/{ type}/type", "type": "value" "#);
	assert!(rpl == r#""url": "/{field_type}/type", "field_type" : "value" "#);
}