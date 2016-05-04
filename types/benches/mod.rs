#![allow(unused_attributes)]

#![feature(custom_derive, custom_attribute, plugin, test)]
#![plugin(serde_macros)]
#![plugin(elastic_types_macros)]

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate test;
#[macro_use]
extern crate elastic_types;

pub mod date_fixtures {
	use std::marker::PhantomData;
	use serde;
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;

	//A custom date mapping
	#[derive(Default, Clone, ElasticDateMapping)]
	pub struct MyDateMapping<T: DateFormat> {
		phantom: PhantomData<T>
	}
	impl <T: DateFormat> ElasticDateMapping<T> for MyDateMapping<T> {
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

		fn null_value() -> Option<ElasticDate<T>> {
			Some(ElasticDate::<T>::now())
		}

		fn ignore_malformed() -> Option<bool> {
			Some(true)
		}

		fn precision_step() -> Option<i32> {
			Some(6)
		}
	}
}

pub mod string_fixtures {
	use std::collections::BTreeMap;
	use elastic_types::mapping::prelude::*;

	#[derive(Default, Clone, ElasticStringMapping)]
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
				analyzer: Some("my_analyzer"),
				fielddata: Some(FieldData::Disabled),
				ignore_above: Some(50),
				index_options: Some(IndexOptions::Docs),
				norms: Some(Norms::Disabled),
				null_value: Some("my default value"),
				position_increment_gap: Some(8),
				search_analyzer: Some("my_search_analyzer"),
				search_quote_analyzer: Some("my_quote_search_analyzer"),
				similarity: Some("BM25"),
				term_vector: Some(TermVector::No)
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
}

pub mod boolean_fixtures {
	use elastic_types::mapping::prelude::*;

	#[derive(Default, Clone, ElasticBooleanMapping)]
	pub struct MyBooleanMapping;
	impl ElasticBooleanMapping for MyBooleanMapping {
		fn boost() -> Option<f32> {
			Some(1.01)
		}

		fn index() -> Option<IndexAnalysis> {
			Some(IndexAnalysis::No)
		}

		fn doc_values() -> Option<bool> {
			Some(true)
		}

		fn store() -> Option<bool> {
			Some(true)
		}

		fn null_value() -> Option<bool> {
			Some(false)
		}
	}
}

pub mod number_fixtures {
	use elastic_types::mapping::prelude::*;

	#[derive(Debug, Clone, Default, ElasticIntegerMapping)]
	pub struct MyIntegerMapping;
	impl ElasticIntegerMapping for MyIntegerMapping {
		fn coerce() -> Option<bool> {
			Some(true)
		}

		fn boost() -> Option<f32> {
			Some(1.1)
		}

		fn doc_values() -> Option<bool> {
			Some(false)
		}

		fn ignore_malformed() -> Option<bool> {
			Some(true)
		}

		fn include_in_all() -> Option<bool> {
			Some(true)
		}

		fn index() -> Option<IndexAnalysis> {
			Some(IndexAnalysis::No)
		}

		fn precision_step() -> Option<u32> {
			Some(2147483647)
		}

		fn store() -> Option<bool> {
			Some(true)
		}

		fn null_value() -> Option<i32> {
			Some(42)
		}
	}
}

pub mod object_fixtures {
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;
	use elastic_types::string::prelude::*;

	#[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	#[elastic(mapping="MySmlTypeMapping")]
	pub struct MySmlType {
		pub my_date1: ElasticDate<DefaultFormat>,
		pub my_string: ElasticString<DefaultStringMapping>,
		pub my_num: i32
	}
	#[derive(Default, Clone)]
	pub struct MySmlTypeMapping;
	impl ElasticObjectMapping for MySmlTypeMapping {
		fn data_type() -> &'static str {
			"object"
		}

		fn dynamic() -> Option<Dynamic> {
			Some(Dynamic::True)
		}

		fn enabled() -> Option<bool> {
			Some(false)
		}

		fn include_in_all() -> Option<bool> {
			Some(true)
		}
	}

	#[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	#[elastic(mapping="MyMedTypeMapping")]
	pub struct MyMedType {
		pub my_date1: ElasticDate<DefaultFormat>,
		pub my_string: ElasticString<DefaultStringMapping>,
		pub my_num: i32,
		pub my_type: MySmlType
	}
	#[derive(Default, Clone)]
	pub struct MyMedTypeMapping;
	impl ElasticObjectMapping for MyMedTypeMapping {
		fn data_type() -> &'static str {
			"object"
		}

		fn dynamic() -> Option<Dynamic> {
			Some(Dynamic::True)
		}

		fn enabled() -> Option<bool> {
			Some(false)
		}

		fn include_in_all() -> Option<bool> {
			Some(true)
		}
	}

	#[derive(Default, Clone, Serialize, Deserialize, ElasticType)]
	#[elastic(mapping="MyLrgTypeMapping")]
	pub struct MyLrgType {
		pub my_date1: ElasticDate<DefaultFormat>,
		pub my_string: ElasticString<DefaultStringMapping>,
		pub my_num: i32,
		pub my_type: MyMedType
	}

	#[derive(Default, Clone)]
	pub struct MyLrgTypeMapping;
	impl ElasticObjectMapping for MyLrgTypeMapping {
		fn data_type() -> &'static str {
			"object"
		}

		fn dynamic() -> Option<Dynamic> {
			Some(Dynamic::True)
		}

		fn enabled() -> Option<bool> {
			Some(false)
		}

		fn include_in_all() -> Option<bool> {
			Some(true)
		}
	}
}

pub mod date;
pub mod string;
pub mod number;
pub mod boolean;
pub mod object;
