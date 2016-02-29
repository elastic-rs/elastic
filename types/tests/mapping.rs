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