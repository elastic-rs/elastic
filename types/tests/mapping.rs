#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::*;
use elastic_types::string::*;

struct MyMapping;
impl ElasticMapping for MyMapping {
	fn get_boost() -> Option<f32> {
		Some(1.01)
	}
}
impl ElasticStringMapping for MyMapping { }

fn get_mapping_field<T, M>(t: &T) -> Option<f32> where M: ElasticMapping, T: ElasticType<M> {
	M::get_boost()
}

#[test]
fn can_access_mapping_fns() {
	let ty = ElasticString::<MyMapping>::new("");

	assert_eq!(Some(1.01), get_mapping_field(&ty));
}

#[test]
fn can_access_mapping_for_auto_impls() {
	let ty = 16;

	assert_eq!(None, get_mapping_field::<_, NullMapping>(&ty));
}

#[test]
fn null_mapping_serialises_to_nothing() {
	let mapping = NullMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!("", ser);
}

#[test]
fn elastic_mapping_serialises_overriden_params() {
	
}

//TODO: Need a standard MappingVisitor<M: ElasticMapping>
//This needs to be implemented for each core type