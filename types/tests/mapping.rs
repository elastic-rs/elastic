#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::*;
use elastic_types::string::*;

struct MyMapping;
impl ElasticStringMapping for MyMapping { 
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
}

impl serde::Serialize for MyMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		serializer.serialize_struct("mapping", ElasticMappingVisitor::<MyMapping>::default())
	}
}

//This is a quick mockup struct that accesses the mapping on a struct
use std::marker::PhantomData;
struct MappingDispatch<T: ElasticType<M, F>, M: ElasticMapping<F> = NullMapping, F = ()> {
	phantom_m: PhantomData<M>,
	phantom_t: PhantomData<T>,
	phantom_f: PhantomData<F>
}
impl <T: ElasticType<M, F>, M: ElasticMapping<F> = NullMapping, F = ()> MappingDispatch<T, M, F> {
	pub fn map(t: &T) -> &'static str {
		//Check out the Visitor associated type on the mapping
		let _ = M::get_visitor();

		//Return the type of this mapping to prove we're looking at something unique
		M::field_type()
	}
}

#[test]
fn can_access_mapping_fns() {
	let ty = ElasticString::<MyMapping>::new("");

	assert_eq!("string", MappingDispatch::map(&ty));
}

#[test]
fn can_access_mapping_for_auto_impls() {
	let ty: i32 = 16;

	//For auto impls, we need to send along at least the type too as a generic param
	assert_eq!("", MappingDispatch::<i32>::map(&ty));
}

#[test]
fn serialise_mapping_null() {
	let mapping = NullMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!("", ser);
}

#[test]
fn serialise_mapping_index() {
	let index_opts: Vec<String> = vec![
		IndexAnalysis::Analyzed,
		IndexAnalysis::NotAnalyzed,
		IndexAnalysis::No
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""analyzed""#,
		r#""not_analyzed""#,
		r#""no""#
	];

	let mut success = true;
	for i in 0..index_opts.len() {
		if expected_opts[i] != &index_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}