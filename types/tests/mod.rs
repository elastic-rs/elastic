#![allow(unused_attributes)]

#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate elastic_types;

pub mod date_fixtures {
	use serde;
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;

	//A custom date mapping
	#[derive(Default, Clone, Copy)]
	pub struct MyDateMapping;
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
	impl_date_mapping!(MyDateMapping, EpochMillis);
}

pub mod string_fixtures {
	use serde;
	use std::collections::BTreeMap;
	use elastic_types::mapping::prelude::*;

	#[derive(Default, Clone)]
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
				analyzer: 					Some("my_analyzer"),
				fielddata: 					Some(FieldData::Disabled),
				ignore_above: 				Some(50),
				index_options: 				Some(IndexOptions::Docs),
				norms: 						Some(Norms::Disabled),
				null_value: 				Some("my default value"),
				position_increment_gap: 	Some(8),
				search_analyzer: 			Some("my_search_analyzer"),
				search_quote_analyzer: 		Some("my_quote_search_analyzer"),
				similarity: 				Some("BM25"),
				term_vector: 				Some(TermVector::No)
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
	impl_string_mapping!(MyStringMapping);
}

pub mod object_fixtures {
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;
	use elastic_types::string::prelude::*;
	use ::date_fixtures::*;

	#[derive(Default, Clone, Serialize, Deserialize)]
	pub struct MyType {
		pub my_date1: DateTime,
		pub my_date2: DateTime<EpochMillis, MyDateMapping>,
		pub my_string: ElasticString<DefaultStringMapping>,
		pub my_num: i32
	}

	#[derive(Default, Clone)]
	struct MyTypeMapping;
	impl ElasticObjectMapping for MyTypeMapping {
		fn data_type() -> &'static str {
			"object"
		}

		fn dynamic() -> Option<bool> {
			Some(true)
		}

		fn enabled() -> Option<bool> {
			Some(false)
		}

		fn include_in_all() -> Option<bool> {
			Some(true)
		}
	}
	impl_object_mapping!(MyType, MyTypeMapping, "my_type", inner1, [my_date1, my_date2, my_string, my_num]);

	#[derive(Default, Clone, Serialize, Deserialize)]
	pub struct MyOtherType {
		pub my_date: DateTime,
		pub my_type: MyType,
		pub my_num: i32
	}
	#[derive(Default, Clone)]
	struct MyOtherTypeMapping;
	impl ElasticObjectMapping for MyOtherTypeMapping { }
	impl_object_mapping!(MyOtherType, MyOtherTypeMapping, "my_other_type", inner2, [my_date, my_type, my_num]);
}

pub mod object;
pub mod date;
pub mod string;