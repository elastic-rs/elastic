#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(json_str)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::prelude::*;
use ::object_fixtures::*;
use test::Bencher;

#[bench]
fn mapping_sml(b: &mut Bencher) {
	b.iter(|| {
		let template = IndexTemplate::new("tmpl", "*", MySmlMapping).unwrap();

		serde_json::to_string(&template).unwrap()
	});
}

#[bench]
fn mapping_med(b: &mut Bencher) {
	b.iter(|| {
		let template = IndexTemplate::new("tmpl", "*", MyMedMapping).unwrap();

		serde_json::to_string(&template).unwrap()
	});
}

#[bench]
fn mapping_lrg(b: &mut Bencher) {
	b.iter(|| {
		let template = IndexTemplate::new("tmpl", "*", MyLrgMapping).unwrap();

		serde_json::to_string(&template).unwrap()
	});
}
