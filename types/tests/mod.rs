#![allow(unused_attributes)]

#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate json_str;

#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate elastic_date_macros;

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate geo as georust;

#[macro_use]
extern crate elastic_types;

pub mod date_fixtures {
	use std::marker::PhantomData;
	use serde;
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;

	//A custom date mapping
	#[derive(Default, Clone, Copy, ElasticDateMapping)]
	pub struct MyDateMapping<T: DateFormat = EpochMillis> {
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
			Some(ElasticDate::<T>::build(2015, 3, 14, 16, 45, 13, 778))
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

	#[derive(Debug, Clone, Default, ElasticLongMapping)]
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

	#[derive(Debug, Clone, Default, ElasticShortMapping)]
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

	#[derive(Debug, Clone, Default, ElasticByteMapping)]
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

	#[derive(Debug, Clone, Default, ElasticFloatMapping)]
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

	#[derive(Debug, Clone, Default, ElasticDoubleMapping)]
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
}

pub mod geo_point_fixtures {
	use std::marker::PhantomData;
	use serde;
	use georust::Coordinate;
	use elastic_types::mapping::prelude::*;
	use elastic_types::geo::point::prelude::*;

	#[derive(Debug, Clone, Default, ElasticGeoPointMapping)]
	pub struct MyGeoPointMapping<T: GeoPointFormat = GeoPointObject> {
		phantom: PhantomData<T>
	}
	impl <T: GeoPointFormat> ElasticGeoPointMapping<T> for MyGeoPointMapping<T> {
		fn geohash() -> Option<bool> {
	        Some(false)
	    }

	    fn geohash_precision() -> Option<u8> {
	        Some(12)
	    }

	    fn geohash_prefix() -> Option<bool> {
	        Some(true)
	    }

	    fn ignore_malformed() -> Option<bool> {
	        Some(true)
	    }

	    fn lat_lon() -> Option<bool> {
	        Some(true)
	    }

	    fn precision_step() -> Option<i32> {
	        Some(128)
	    }
	}
}

pub mod geo_shape_fixtures {
	
}

pub mod object_fixtures {
	use chrono::{ DateTime, UTC };
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;
	use elastic_types::number::prelude::*;
	use elastic_types::string::prelude::*;
	use elastic_types::boolean::prelude::*;
	use ::date_fixtures::*;
	use ::number_fixtures::*;
	use ::boolean_fixtures::*;

	#[derive(Serialize, Deserialize, ElasticType)]
	#[elastic(ty="my_type", mapping="MyTypeMapping")]
	pub struct MyType {
		pub my_date1: DateTime<UTC>,
		pub my_date2: ElasticDate<DefaultDateFormat>,
		pub my_date3: ElasticDate<EpochMillis, MyDateMapping>,
		pub my_string1: String,
		pub my_string2: ElasticString<DefaultStringMapping>,
		pub my_num1: i32,
		pub my_num2: ElasticInteger<MyIntegerMapping>,
		pub my_bool1: bool,
		pub my_bool2: ElasticBoolean<MyBooleanMapping>
	}

	#[derive(Default, Clone)]
	pub struct MyTypeMapping;
	impl ElasticObjectMapping for MyTypeMapping {
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

	#[derive(Serialize, Deserialize, ElasticType)]
	pub struct MyOtherType {
		pub my_date: ElasticDate<DefaultDateFormat>,
		#[serde(rename="my_renamed_type")]
		pub my_type: MyType,
		#[serde(skip_serializing)]
		pub ignored: String,
		pub my_num: i32,
		pub my_strings: Vec<String>,
		pub my_dates: Vec<ElasticDate<DefaultDateFormat>>
	}
}

pub mod object;
pub mod geo_point;
pub mod date;
pub mod string;
pub mod number;
pub mod boolean;
