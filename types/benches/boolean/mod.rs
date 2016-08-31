#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::prelude::*;
use ::boolean_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
	b.iter(|| {
		FieldMapper::to_string(MyBooleanMapping).unwrap()
	});
}