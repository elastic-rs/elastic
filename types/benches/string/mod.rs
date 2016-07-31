#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use ::string_fixtures::*;

use test::Bencher;

#[bench]
fn keyword_mapping(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyKeywordMapping;
		serde_json::to_string(&mapping).unwrap()
	});
}

#[bench]
fn text_mapping(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyTextMapping;
		serde_json::to_string(&mapping).unwrap()
	});
}