#![allow(unused_attributes)]
#![feature(custom_derive)]

#![feature(custom_derive, custom_attribute, plugin, test)]
#![plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros)]

#[allow(plugin_as_library)]
#[macro_use]
extern crate json_str;

#[macro_use]
extern crate maplit;

#[allow(plugin_as_library)]
#[macro_use]
extern crate elastic_date_macros;

extern crate serde;
extern crate serde_json;
pub extern crate chrono;
extern crate geo as georust;
extern crate geojson;
extern crate test;

#[macro_use]
extern crate elastic_types;

pub mod date_fixtures {
	use std::marker::PhantomData;
	use elastic_types::mapping::prelude::*;
	use elastic_types::date::prelude::*;

	date_mapping!(MyDateMapping {
		fn null_value() -> Option<Date<F, Self>> {
			Some(Date::build(2015, 3, 14, 16, 45, 13, 778))
		}

		fn boost() -> Option<f32> 				{ Some(1.01) }

		fn index() -> Option<bool> 				{ Some(true) }

		fn doc_values() -> Option<bool> 		{ Some(true) }

		fn include_in_all() -> Option<bool> 	{ Some(false) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn ignore_malformed() -> Option<bool> 	{ Some(true) }
	});
}

pub mod string_fixtures {
	use std::collections::BTreeMap;
	use elastic_types::mapping::prelude::*;

	text_mapping!(MyTextMapping {
		fn fields() -> Option<BTreeMap<&'static str, ElasticStringField>> {
			let mut fields = BTreeMap::new();

			fields.insert("raw", ElasticStringField::Keyword(
				KeywordFieldMapping {
					analyzer: Some("my_analyzer"),
					..Default::default()
				})
			);

			fields.insert("count", ElasticStringField::TokenCount(
				ElasticTokenCountFieldMapping::default())
			);

			fields.insert("comp", ElasticStringField::Completion(
				ElasticCompletionFieldMapping::default())
			);

			Some(fields)
		}

		fn fielddata_frequency_filter() -> Option<FieldDataFrequencyFilter> { 
			Some(FieldDataFrequencyFilter { min: Some(0.0), ..Default::default() })
		}

		fn analyzer() -> Option<&'static str> 				{ Some("my_analyzer") }

		fn boost() -> Option<f32> 							{ Some(1.3) }

		fn eager_global_ordinals() -> Option<bool> 			{ Some(false) }

		fn fielddata() -> Option<bool> 						{ Some(true) }

		fn include_in_all() -> Option<bool> 				{ Some(true) }

		fn ignore_above() -> Option<u32> 					{ Some(512) }

		fn index() -> Option<bool> 							{ Some(false) }

		fn index_options() -> Option<IndexOptions> 			{ Some(IndexOptions::Freqs) }

		fn norms() -> Option<bool> 							{ Some(true) }

		fn position_increment_gap() -> Option<u32> 			{ Some(1) }

		fn store() -> Option<bool> 							{ Some(true) }

		fn search_analyzer() -> Option<&'static str> 		{ Some("my_analyzer") }

		fn search_quote_analyzer() -> Option<&'static str> 	{ Some("my_analyzer") }

		fn similarity() -> Option<&'static str> 			{ Some("BM25") }

		fn term_vector() -> Option<TermVector> 				{ Some(TermVector::Yes) }
	});

	keyword_mapping!(MyKeywordMapping {
		fn fields() -> Option<BTreeMap<&'static str, ElasticStringField>> {
			let mut fields = BTreeMap::new();

			fields.insert("text", ElasticStringField::Text(
				TextFieldMapping {
					analyzer: Some("my_analyzer"),
					..Default::default()
				})
			);

			fields.insert("count", ElasticStringField::TokenCount(
				ElasticTokenCountFieldMapping::default())
			);

			fields.insert("comp", ElasticStringField::Completion(
				ElasticCompletionFieldMapping::default())
			);

			Some(fields)
		}

		fn analyzer() -> Option<&'static str> 			{ Some("my_analyzer") }

		fn boost() -> Option<f32> 						{ Some(1.03) }

		fn doc_values() -> Option<bool> 				{ Some(true) }

		fn eager_global_ordinals() -> Option<bool> 		{ Some(false) }

		fn include_in_all() -> Option<bool> 			{ Some(false) }

		fn ignore_above() -> Option<u32> 				{ Some(256) }

		fn index() -> Option<bool> 						{ Some(true) }

		fn index_options() -> Option<IndexOptions> 		{ Some(IndexOptions::Docs) }

		fn norms() -> Option<bool> 						{ Some(false) }

		fn null_value() -> Option<&'static str> 		{ Some("my string") }

		fn store() -> Option<bool> 						{ Some(false) }

		fn search_analyzer() -> Option<&'static str> 	{ Some("my_analyzer") }

		fn similarity() -> Option<&'static str> 		{ Some("classic") }
	});
}

pub mod boolean_fixtures {
	use elastic_types::mapping::prelude::*;

	boolean_mapping!(MyBooleanMapping {
		fn boost() -> Option<f32> 			{ Some(1.01) }

		fn index() -> Option<bool> 			{ Some(false) }

		fn doc_values() -> Option<bool> 	{ Some(true) }

		fn store() -> Option<bool> 			{ Some(true) }

		fn null_value() -> Option<bool> 	{ Some(false) }
	});
}

pub mod number_fixtures {
	use elastic_types::mapping::prelude::*;

	integer_mapping!(MyIntegerMapping {
		fn coerce() -> Option<bool> 			{ Some(true) }

		fn boost() -> Option<f32> 				{ Some(1.1) }

		fn doc_values() -> Option<bool> 		{ Some(false) }

		fn ignore_malformed() -> Option<bool> 	{ Some(true) }

		fn include_in_all() -> Option<bool> 	{ Some(true) }

		fn index() -> Option<bool> 				{ Some(false) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn null_value() -> Option<i32> 			{ Some(42) }
	});

	long_mapping!(MyLongMapping {
		fn coerce() -> Option<bool> 			{ Some(true) }

		fn boost() -> Option<f32> 				{ Some(1.1) }

		fn doc_values() -> Option<bool> 		{ Some(false) }

		fn ignore_malformed() -> Option<bool> 	{ Some(true) }

		fn include_in_all() -> Option<bool> 	{ Some(true) }

		fn index() -> Option<bool> 				{ Some(false) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn null_value() -> Option<i64> 			{ Some(-42) }
	});

	short_mapping!(MyShortMapping {
		fn coerce() -> Option<bool> 			{ Some(true) }

		fn boost() -> Option<f32> 				{ Some(1.1) }

		fn doc_values() -> Option<bool> 		{ Some(false) }

		fn ignore_malformed() -> Option<bool> 	{ Some(true) }

		fn include_in_all() -> Option<bool> 	{ Some(true) }

		fn index() -> Option<bool> 				{ Some(false) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn null_value() -> Option<i16> 			{ Some(42) }
	});

	byte_mapping!(MyByteMapping {
		fn coerce() -> Option<bool> 			{ Some(true) }

		fn boost() -> Option<f32> 				{ Some(1.1) }

		fn doc_values() -> Option<bool> 		{ Some(false) }

		fn ignore_malformed() -> Option<bool> 	{ Some(true) }

		fn include_in_all() -> Option<bool> 	{ Some(true) }

		fn index() -> Option<bool> 				{ Some(false) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn null_value() -> Option<i8> 			{ Some(1) }
	});

	float_mapping!(MyFloatMapping {
		fn coerce() -> Option<bool> 			{ Some(true) }

		fn boost() -> Option<f32> 				{ Some(1.1) }

		fn doc_values() -> Option<bool> 		{ Some(false) }

		fn ignore_malformed() -> Option<bool> 	{ Some(true) }

		fn include_in_all() -> Option<bool> 	{ Some(true) }

		fn index() -> Option<bool> 				{ Some(false) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn null_value() -> Option<f32> 			{ Some(1.04) }
	});

	double_mapping!(MyDoubleMapping {
		fn coerce() -> Option<bool> 			{ Some(true) }

		fn boost() -> Option<f32> 				{ Some(1.1) }

		fn doc_values() -> Option<bool> 		{ Some(false) }

		fn ignore_malformed() -> Option<bool> 	{ Some(true) }

		fn include_in_all() -> Option<bool> 	{ Some(true) }

		fn index() -> Option<bool> 				{ Some(false) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn null_value() -> Option<f64> 			{ Some(-0.00002) }
	});
}

pub mod ip_fixtures {
	use std::net::Ipv4Addr;
	use elastic_types::mapping::prelude::*;

	ip_mapping!(MyIpMapping {
		fn boost() -> Option<f32> 				{ Some(1.01) }

		fn index() -> Option<bool> 				{ Some(false) }

		fn doc_values() -> Option<bool> 		{ Some(true) }

		fn store() -> Option<bool> 				{ Some(true) }

		fn null_value() -> Option<Ipv4Addr> 	{ Some(Ipv4Addr::new(127, 0, 0, 1)) }		
	});
}

pub mod geo_point_fixtures {
	use std::marker::PhantomData;
	use elastic_types::mapping::prelude::*;

	geo_point_mapping!(MyGeoPointMapping {
		fn geohash() -> Option<bool> 				{ Some(false) }

		fn geohash_precision() -> Option<Distance> 	{ Some(Distance(50.0, DistanceUnit::Meters)) }

		fn geohash_prefix() -> Option<bool> 		{ Some(true) }

		fn ignore_malformed() -> Option<bool> 		{ Some(true) }

		fn lat_lon() -> Option<bool> 				{ Some(true) }
	});
}

pub mod geo_shape_fixtures {
	use elastic_types::mapping::prelude::*;

	geo_shape_mapping!(MyGeoShapeMapping {
		fn tree() -> Option<Tree> { Some(Tree::Geohash) }

		fn precision() -> Option<Distance> 			{ Some(Distance(50.0, DistanceUnit::Meters)) }

		fn tree_levels() -> Option<i32> 			{ Some(8) }

		fn strategy() -> Option<Strategy> 			{ Some(Strategy::Recursive) }

		fn distance_error_pct() -> Option<f32> 		{ Some(0.5) }

		fn orientation() -> Option<Orientation> 	{ Some(Orientation::Clockwise) }

		fn points_only() -> Option<bool> 			{ Some(false) }
	});
}

pub mod object_fixtures {
	use serde;
	use chrono::{ DateTime, UTC };
	use elastic_types::prelude::*;

	#[derive(Default, Clone)]
	pub struct MySmlMapping;
	type_mapping!(ty MySmlMapping {
		fn props_len() -> usize { 3 }
		
		fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
		where S: serde::Serializer {
			try!(serializer.serialize_struct_elt(state, "integer", i32::mapping()));
			try!(serializer.serialize_struct_elt(state, "string", String::mapping()));
			try!(serializer.serialize_struct_elt(state, "date", DateTime::<UTC>::mapping()));

			Ok(())
		}
	});

	#[derive(Default, Clone)]
	pub struct MyMedMapping;
	type_mapping!(ty MyMedMapping {
		fn props_len() -> usize { 4 }
		
		fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
		where S: serde::Serializer {
			try!(serializer.serialize_struct_elt(state, "integer", i32::mapping()));
			try!(serializer.serialize_struct_elt(state, "string", String::mapping()));
			try!(serializer.serialize_struct_elt(state, "date", DateTime::<UTC>::mapping()));
			try!(serializer.serialize_struct_elt(state, "field", MySmlMapping));

			Ok(())
		}
	});

	#[derive(Default, Clone)]
	pub struct MyLrgMapping;
	type_mapping!(ty MyLrgMapping {
		fn props_len() -> usize { 4 }
		
		fn serialize_props<S>(serializer: &mut S, state: &mut S::StructState) -> Result<(), S::Error>
		where S: serde::Serializer {
			try!(serializer.serialize_struct_elt(state, "integer", i32::mapping()));
			try!(serializer.serialize_struct_elt(state, "string", String::mapping()));
			try!(serializer.serialize_struct_elt(state, "date", DateTime::<UTC>::mapping()));
			try!(serializer.serialize_struct_elt(state, "field", MyMedMapping));

			Ok(())
		}
	});
}

pub mod object;
pub mod geo_point;
pub mod geo_shape;
pub mod date;
pub mod string;
pub mod number;
pub mod boolean;
