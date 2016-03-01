#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::*;
use elastic_types::string::*;

struct MyMapping;
impl ElasticMapping for MyMapping {
	type Visitor = NullMappingVisitor;

	fn get_boost() -> Option<f32> {
		Some(1.01)
	}

	fn get_index() -> Option<IndexAnalysis> {
		Some(IndexAnalysis::No)
	}
}
impl ElasticStringMapping for MyMapping { }

impl serde::Serialize for MyMapping {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_struct("mapping", ElasticMappingVisitor::<MyMapping>::default())
    }
}

//This is a quick mockup struct that accesses the mapping on a struct
use std::marker::PhantomData;
struct MappingDispatch<T: ElasticType<M>, M: ElasticMapping = NullMapping> {
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>
}
impl <T: ElasticType<M>, M: ElasticMapping = NullMapping> MappingDispatch<T, M> {
	pub fn map(t: &T) -> Option<f32> {
		//Check out the Visitor associated type on the mapping
		let _ = M::get_visitor();

		//Return something to prove we're looking at a unique mapping
		M::get_boost()
	}
}

#[test]
fn can_access_mapping_fns() {
	let ty = ElasticString::<MyMapping>::new("");

	assert_eq!(Some(1.01), MappingDispatch::map(&ty));
}

#[test]
fn can_access_mapping_for_auto_impls() {
	let ty: i32 = 16;

	assert_eq!(None, MappingDispatch::<i32>::map(&ty));
}

#[test]
fn null_mapping_serialises_to_nothing() {
	let mapping = NullMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!("", ser);
}

#[test]
fn mapping_serialises_overriden_params() {
	let mapping = MyMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"boost":1.01,"index":"no"}"#, ser);
}