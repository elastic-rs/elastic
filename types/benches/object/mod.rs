#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

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
		TypeMapper::to_string(MySmlMapping).unwrap()
	});
}

#[bench]
fn mapping_med(b: &mut Bencher) {
	b.iter(|| {
		TypeMapper::to_string(MyMedMapping).unwrap()
	});
}

#[bench]
fn mapping_lrg(b: &mut Bencher) {
	b.iter(|| {
		TypeMapper::to_string(MyLrgMapping).unwrap()
	});
}
