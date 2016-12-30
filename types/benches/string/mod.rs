#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::prelude::*;
use ::string_fixtures::*;

use test::Bencher;

#[bench]
fn keyword_mapping(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&MyKeywordMapping).unwrap()
	});
}

#[bench]
fn text_mapping(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&MyTextMapping).unwrap()
	});
}