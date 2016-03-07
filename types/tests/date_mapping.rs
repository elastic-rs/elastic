#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::*;

//A custom date mapping
#[derive(Default)]
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

//TODO: derive this
impl serde::Serialize for MyDateMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("mapping", MyDateMapping::get_visitor())
	}
}

#[test]
fn serialise_mapping_default() {
	let mapping = DefaultDateMapping::<BasicDateTime>::new();
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"type":"date","format":"basic_date_time"}"#, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyDateMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"type":"date","boost":1.01,"doc_values":true,"include_in_all":false,"index":"no","store":true,"format":"epoch_millis","ignore_malformed":true,"null_value":"0","precision_step":6}"#, ser);
}