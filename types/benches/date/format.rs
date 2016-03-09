#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::prelude::*;

use ::date_fixtures::*;

use test::Bencher;

#[bench]
fn serialise_date_format(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyDateMapping;
		serde_json::to_string(&mapping).unwrap()
	});
}