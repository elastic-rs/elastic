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
	pub date: DateTime,
	pub string: String,
	pub num: i32
}

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

//TODO: This can probably be standardised
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
		MappingDispatch::map("date", &self.data.date, serializer);
		MappingDispatch::map("string", &self.data.string, serializer);
		//TODO: This is an issue because we need to be able to deferentiate types with NullMapping. See how serde does it
		MappingDispatch::<i32>::map("num", &self.data.num, serializer);

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

	assert_eq!(r#"{"date":{"type":"date","format":"basic_date_time"},"string":{"type":"string"},"num":{"type":"object"}}"#, ser);
}