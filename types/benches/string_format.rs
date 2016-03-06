#![feature(test, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate test;
extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use std::collections::BTreeMap;
use elastic_types::mapping::*;

use test::Bencher;

pub struct MyStringMapping;
impl ElasticStringMapping for MyStringMapping { 
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

	fn analyzer() -> Option<&'static str> {
		Some("my_analyzer")
	}

	fn fielddata() -> Option<FieldData> {
		Some(FieldData::Disabled)
	}

	fn fields() -> Option<BTreeMap<&'static str, ElasticStringFieldMapping>> {
		let mut fields = BTreeMap::new();
 		fields.insert("raw", ElasticStringFieldMapping {
 			analyzer: Some("my_analyzer"),
 			..Default::default()
 		});
 		fields.insert("bm25_field", ElasticStringFieldMapping {
 			similarity: Some("BM25"),
 			..Default::default()
 		});
 		
 		Some(fields)
	}

	fn ignore_above() -> Option<usize> {
		Some(50)
	}

	fn index_options() -> Option<IndexOptions> {
		Some(IndexOptions::Docs)
	}

	fn norms() -> Option<Norms> {
		Some(Norms::Disabled)
	}

	fn null_value() -> Option<&'static str> {
		Some("my default value")
	}

	fn position_increment_gap() -> Option<usize> {
		Some(8)
	}

	fn search_analyzer() -> Option<&'static str> {
		Some("my_search_analyzer")
	}

	fn search_quote_analyzer() -> Option<&'static str> {
		Some("my_quote_search_analyzer")
	}

	fn similarity() -> Option<&'static str> {
		Some("my_similarity")
	}

	fn term_vector() -> Option<TermVector> {
		Some(TermVector::No)
	}
}

impl serde::Serialize for MyStringMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		serializer.serialize_struct("mapping", MyStringMapping::get_visitor())
	}
}

#[bench]
fn serialise_string_format(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyStringMapping;
		serde_json::to_string(&mapping).unwrap()
	});
}