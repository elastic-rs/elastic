#![feature(test, plugin)]
#![plugin(elastic_macros)]

extern crate test;
extern crate chrono;
extern crate elastic_types;

pub mod format;

use elastic_types::date::prelude::*;

use test::Bencher;

#[bench]
fn parse_date_from_format(b: &mut Bencher) {
	b.iter(|| {
		DateTime::<BasicDateTime>::parse("20150620T134501.034Z").unwrap()
	});
}

#[bench]
fn parse_date_to_string(b: &mut Bencher) {
	let dt: DateTime = DateTime::now();

	b.iter(|| {
		dt.format()
	});
}

#[bench]
fn parse_date_from_epoch(b: &mut Bencher) {
	b.iter(|| {
		DateTime::<EpochMillis>::parse("1435935302478").unwrap()
	});
}

#[bench]
fn parse_date_to_epoch(b: &mut Bencher) {
	let dt = DateTime::<EpochMillis>::now();

	b.iter(|| {
		dt.format()
	});
}

#[bench]
fn get_date_fmt_vec(b: &mut Bencher) {
	b.iter(|| {
		BasicDateTime::fmt()
	});
}
