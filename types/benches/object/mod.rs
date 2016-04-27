#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::object_fixtures::*;
use test::Bencher;

#[bench]
fn mapping_sml(b: &mut Bencher) {
	b.iter(|| {
		TypeMapper::to_string(MySmlTypeMapping).unwrap()
	});
}

#[bench]
fn mapping_med(b: &mut Bencher) {
	b.iter(|| {
		TypeMapper::to_string(MyMedTypeMapping).unwrap()
	});
}

#[bench]
fn mapping_lrg(b: &mut Bencher) {
	b.iter(|| {
		TypeMapper::to_string(MyLrgTypeMapping).unwrap()
	});
}
