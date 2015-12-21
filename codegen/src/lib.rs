#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate regex;

pub mod parse;
pub mod ast;

#[test]
fn can_parse_from_file() {
	let mut f = std::fs::File::open("api/bulk.json").unwrap();
	let parsed = parse::from_reader(&mut f).unwrap();

	assert!(parsed.name.unwrap() == "bulk".to_string());
}