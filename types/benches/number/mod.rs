#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use ::number_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyIntegerMapping;
		serde_json::to_string(&mapping).unwrap()
	});
}
