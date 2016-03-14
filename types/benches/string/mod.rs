#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use ::string_fixtures::*;

use test::Bencher;

#[bench]
fn serialise_string_format(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyStringMapping;
		serde_json::to_string(&mapping).unwrap()
	});
}