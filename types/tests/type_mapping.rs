#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use elastic_types::date::*;

//A type we can add to Elasticsearch
#[derive(Default, Clone)]
struct MyType {
	pub my_date1: DateTime,
	pub my_date2: DateTime<EpochMillis, MyDateMapping>,
	pub my_string: String,
	pub my_num: i32
}

#[derive(Default, Clone)]
struct MyDateMapping;
impl ElasticDateMapping<EpochMillis> for MyDateMapping {
	fn boost() -> Option<f32> {
		Some(1.01)
	}

	fn index() -> Option<IndexAnalysis> {
		Some(IndexAnalysis::No)
	}
}

//TODO: derive this
impl serde::Serialize for MyDateMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer {
		serializer.serialize_struct("mapping", MyDateMapping::get_visitor())
	}
}

//TODO: Start Derive -----

impl <'a> ElasticDataType<MyTypeMapping<'a>, ()> for MyType { }

//The mapping for our type
use std::marker::PhantomData;

#[derive(Default)]
struct MyTypeMapping<'a> {
	phantom: PhantomData<&'a ()>
}

impl <'a> ElasticMapping<()> for MyTypeMapping<'a> {
	type Visitor = MyTypeMappingVisitor<'a>;
}

//Serialisation for our mapping
use std::borrow::Cow;
use std::borrow::Borrow;

struct MyTypeMappingVisitor<'a> { 
	data: Cow<'a, MyType>
}
impl <'a> Default for MyTypeMappingVisitor<'a> {
	fn default() -> MyTypeMappingVisitor<'a> {
		MyTypeMappingVisitor {
			data: Cow::Owned(MyType::default())
		}
	}
}

impl <'a> serde::ser::MapVisitor for MyTypeMappingVisitor<'a> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		MappingDispatch::map("my_date1", &self.data.my_date1, serializer);
		MappingDispatch::map("my_date2", &self.data.my_date2, serializer);
		MappingDispatch::map("my_string", &self.data.my_string, serializer);
		//TODO: This is an issue because we need to be able to deferentiate types with NullMapping. See how serde does it
		MappingDispatch::<i32>::map("my_num", &self.data.my_num, serializer);

		Ok(None)
	}
}

impl <'a> serde::Serialize for MyTypeMapping<'a> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: serde::Serializer
	{
		serializer.serialize_struct("mapping", MyTypeMappingVisitor::default())
	}
}

//TODO: End derive -----

//TODO: Standardise this in the main crate
struct MappingDispatch<T: ElasticDataType<M, F>, M: ElasticMapping<F> = NullMapping, F = ()> {
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>,
	phantom_f: PhantomData<F>
}
impl <T: ElasticDataType<M, F>, M: ElasticMapping<F> = NullMapping, F = ()> MappingDispatch<T, M, F> {
	pub fn map<S>(key: &'static str, _: &T, serializer: &mut S) -> Result<(), S::Error> 
	where S: serde::Serializer {
		serializer.serialize_struct_elt(key, M::default())
	}
}

#[test]
fn serialise_mapping_type() {
	let mapping = MyTypeMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"my_date1":{"type":"date","format":"basic_date_time"},"my_date2":{"type":"date","boost":1.01,"index":"no","format":"epoch_millis"},"my_string":{"type":"string"},"my_num":{"type":"object"}}"#, ser);
}