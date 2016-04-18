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

pub mod boolean_fixtures {
	use serde;
	use elastic_types::mapping::prelude::*;

	#[derive(Default, Clone)]
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
	impl_boolean_mapping!(MyBooleanMapping);
}

pub mod number_fixtures {
	use serde;
	use elastic_types::mapping::prelude::*;

	#[derive(Debug, Clone, Default)]
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
	impl_integer_mapping!(MyIntegerMapping);

	#[derive(Debug, Clone, Default)]
	pub struct MyLongMapping;
	impl ElasticLongMapping for MyLongMapping {
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

		fn null_value() -> Option<i64> {
			Some(42)
		}
	}
	impl_long_mapping!(MyLongMapping);

	#[derive(Debug, Clone, Default)]
	pub struct MyShortMapping;
	impl ElasticShortMapping for MyShortMapping {
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

		fn null_value() -> Option<i16> {
			Some(-42)
		}
	}
	impl_short_mapping!(MyShortMapping);

	#[derive(Debug, Clone, Default)]
	pub struct MyByteMapping;
	impl ElasticByteMapping for MyByteMapping {
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

		fn null_value() -> Option<i8> {
			Some(1)
		}
	}
	impl_byte_mapping!(MyByteMapping);

	#[derive(Debug, Clone, Default)]
	pub struct MyFloatMapping;
	impl ElasticFloatMapping for MyFloatMapping {
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

		fn null_value() -> Option<f32> {
			Some(1.04)
		}
	}
	impl_float_mapping!(MyFloatMapping);

	#[derive(Debug, Clone, Default)]
	pub struct MyDoubleMapping;
	impl ElasticDoubleMapping for MyDoubleMapping {
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

		fn null_value() -> Option<f64> {
			Some(-0.00002)
		}
	}
	impl_double_mapping!(MyDoubleMapping);
}

pub mod object_fixtures {
	use chrono;
	use chrono::{ DateTime, UTC };
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;
	use elastic_types::number::prelude::*;
	use elastic_types::string::prelude::*;
	use elastic_types::boolean::prelude::*;
	use ::date_fixtures::*;
	use ::number_fixtures::*;
	use ::boolean_fixtures::*;

	#[derive(Clone, Serialize, Deserialize, ElasticType)]
	#[elastic(ty="my_type", mapping="MyTypeMapping")]
	pub struct MyType {
		pub my_date1: DateTime<UTC>,
		pub my_date2: ElasticDate,
		pub my_date3: ElasticDate<EpochMillis, MyDateMapping>,
		pub my_string1: String,
		pub my_string2: ElasticString<DefaultStringMapping>,
		pub my_num1: i32,
		pub my_num2: ElasticInteger<MyIntegerMapping>,
		pub my_bool1: bool,
		pub my_bool2: ElasticBoolean<MyBooleanMapping>
	}

	impl Default for MyType {
		fn default() -> MyType {
			MyType {
				my_date1: chrono::UTC::now(),
				my_date2: ElasticDate::default(),
				my_date3: ElasticDate::<EpochMillis, MyDateMapping>::default(),
				my_string1: String::default(),
				my_string2: ElasticString::<DefaultStringMapping>::default(),
				my_num1: i32::default(),
				my_num2: ElasticInteger::<MyIntegerMapping>::default(),
				my_bool1: false,
				my_bool2: ElasticBoolean::<MyBooleanMapping>::default()
			}
		}
	}

	#[derive(Default, Clone)]
	struct MyTypeMapping;
	impl ElasticObjectMapping for MyTypeMapping {
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
	pub struct MyOtherType {
		pub my_date: ElasticDate,
		#[serde(rename="my_renamed_type")]
		pub my_type: MyType,
		#[serde(skip_serializing)]
		pub ignored: String,
		pub my_num: i32,
		pub my_strings: Vec<String>,
		pub my_dates: Vec<ElasticDate>
	}
}

pub mod object;
pub mod date;
pub mod string;
pub mod number;
pub mod boolean;
