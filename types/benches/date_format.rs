#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::{ DateTime, Format, BasicDateTime, EpochMillis };

use test::Bencher;

struct MyDateMapping;
impl ElasticDateMapping<EpochMillis> for MyDateMapping {
	fn boost() -> Option<f32> {
		Some(1.01)
	}

	fn index() -> Option<IndexAnalysis> {
		Some(IndexAnalysis::No)
	}

	fn doc_values() -> Option<bool> {
		Some(true)
	}

	fn include_in_all() -> Option<bool> {
		Some(false)
	}

	fn store() -> Option<bool> {
		Some(true)
	}

	fn null_value() -> Option<&'static str> {
		Some("0")
	}

	fn ignore_malformed() -> Option<bool> {
		Some(true)
	}

	fn precision_step() -> Option<i32> {
		Some(6)
	}
}

impl serde::Serialize for MyDateMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		serializer.serialize_struct("mapping", ElasticDateMappingVisitor::<EpochMillis, MyDateMapping>::default())
	}
}

#[bench]
fn serialise_date_format(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyDateMapping;
		serde_json::to_string(&mapping).unwrap()
	});
}