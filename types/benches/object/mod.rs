#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use serde_json::ser::Serializer;
use elastic_types::mapping::prelude::*;
use ::object_fixtures::*;
use test::Bencher;

#[bench]
fn mapping_sml(b: &mut Bencher) {
	b.iter(|| {
		let ty = MySmlType::default();

		let mut writer = Vec::with_capacity(128);
		{
			let mut ser = Serializer::new(&mut writer);
			let _ = TypeMapper::map(&ty, &mut ser).unwrap();
		}

		writer
	});
}

#[bench]
fn mapping_med(b: &mut Bencher) {
	b.iter(|| {
		let ty = MyMedType::default();

		let mut writer = Vec::with_capacity(128);
		{
			let mut ser = Serializer::new(&mut writer);
			let _ = TypeMapper::map(&ty, &mut ser).unwrap();
		}

		writer
	});
}

#[bench]
fn mapping_lrg(b: &mut Bencher) {
	b.iter(|| {
		let ty = MyLrgType::default();

		let mut writer = Vec::with_capacity(128);
		{
			let mut ser = Serializer::new(&mut writer);
			let _ = TypeMapper::map(&ty, &mut ser).unwrap();
		}

		writer
	});
}