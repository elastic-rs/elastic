#![feature(test, plugin)]
#![plugin(json_str)]

extern crate serde;
extern crate serde_json;
extern crate test;
extern crate chrono;
extern crate elastic_types;

use elastic_types::prelude::*;
use ::date_fixtures::*;

use test::Bencher;

#[bench]
fn parse_string(b: &mut Bencher) {
	b.iter(|| {
		Date::<BasicDateTime>::parse("20150620T134501.034Z").unwrap()
	});
}

#[bench]
fn fmt_string(b: &mut Bencher) {
	let dt: Date<DefaultDateFormat> = Date::now();

	b.iter(|| {
		dt.format()
	});
}

#[bench]
fn parse_epoch(b: &mut Bencher) {
	b.iter(|| {
		Date::<EpochMillis>::parse("1435935302478").unwrap()
	});
}

#[bench]
fn fmt_epoch(b: &mut Bencher) {
	let dt = Date::<EpochMillis>::now();

	b.iter(|| {
		dt.format()
	});
}

#[bench]
fn mapping(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&MyDateMapping).unwrap()
	});
}
