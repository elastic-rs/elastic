//! Mapping for the Elasticsearch `text` type.

use std::collections::BTreeMap;
use serde::{ self, Serializer, Serialize };
use ::mapping::ElasticFieldMapping;
use ::string::mapping::{ ElasticStringField, IndexOptions };

/// Elasticsearch datatype name.
pub const TEXT_DATATYPE: &'static str = "text";

/// The base requirements for mapping a `string` type.
///
/// Custom mappings can be defined by implementing `ElasticTextMapping`.
///
/// # Examples
///
/// Define a custom `ElasticTextMapping`:
///
/// ## Derive Mapping
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::string::prelude::*;
///
/// #[derive(Debug, Clone, Default, ElasticTextMapping)]
/// pub struct MyStringMapping;
/// impl ElasticTextMapping for MyStringMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
/// # fn main() {}
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(elastic_types_macros)]
/// # #[macro_use]
/// # extern crate json_str;
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::mapping::prelude::*;
/// # #[derive(Debug, Clone, Default, ElasticTextMapping)]
/// # pub struct MyStringMapping;
/// # impl ElasticTextMapping for MyStringMapping {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyStringMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "text",
/// 	"boost": 1.5
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
///
/// ## Manually
///
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::string::prelude::*;
///
/// #[derive(Debug, Clone, Default)]
/// pub struct MyStringMapping;
/// impl ElasticTextMapping for MyStringMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// //We also need to implement the base `ElasticFieldMapping` and `serde::Serialize` for our custom mapping type
/// impl ElasticFieldMapping<()> for MyStringMapping {
/// 	type Visitor = ElasticTextMappingVisitor<MyStringMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		TEXT_DATATYPE
/// 	}
/// }
///
/// impl serde::Serialize for MyStringMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
pub trait ElasticTextMapping where
Self: ElasticFieldMapping<()> + Sized + Serialize {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	fn analyzer() -> Option<&'static str> { None }

	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<f32> { None }

	/// Should global ordinals be loaded eagerly on refresh? 
	/// Accepts `true` or `false` (default).
	/// Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations. 
	fn eager_global_ordinals() -> Option<bool> { None }

	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting? 
	/// Accepts `true` or `false` (default).
	fn fielddata() -> Option<bool> { None }

	/// Expert settings which allow to decide which values to load in memory when `fielddata` is enabled. 
	/// By default all values are loaded.
	fn fielddata_frequency_filter() -> Option<FieldDataFrequencyFilter> { None }

	/// Multi-fields allow the same string value to be indexed in multiple ways for different purposes,
	/// such as one field for search and a multi-field for sorting and aggregations,
	/// or the same string value analyzed by different analyzers.
	///
	/// # Examples
	///
	/// Subfields are provided as simple `struct`s, so you don't need to define a separate type
	/// to map them:
	///
	/// ```
	/// # #![feature(plugin, custom_derive, custom_attribute)]
	/// # #![plugin(json_str, elastic_types_macros)]
	/// # #[macro_use]
	/// # extern crate elastic_types;
	/// # extern crate serde;
	/// # use std::collections::BTreeMap;
	/// # use elastic_types::mapping::prelude::*;
	/// # use elastic_types::string::prelude::*;
	/// # #[derive(Debug, Clone, Default, ElasticTextMapping)]
	/// # pub struct MyStringMapping;
	/// # impl ElasticTextMapping for MyStringMapping {
	/// fn fields() -> Option<BTreeMap<&'static str, ElasticStringField>> {
	///		let mut fields = BTreeMap::new();
	///
	/// 	//Add a `token_count` as a sub field
	/// 	fields.insert("count", ElasticStringField::TokenCount(
	/// 		ElasticTokenCountFieldMapping::default())
	/// 	);
	///
	/// 	//Add a `completion` suggester as a sub field
	/// 	fields.insert("comp", ElasticStringField::Completion(
	/// 		ElasticCompletionFieldMapping::default())
	/// 	);
	///
	/// 	Some(fields)
	///	}
	/// # }
	/// # fn main() {}
	/// ```
	fn fields() -> Option<BTreeMap<&'static str, ElasticStringField>> { None }

	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	fn include_in_all() -> Option<bool> { None }

	/// The maximum number of characters to index.
	/// Any characters over this length will be ignored.
	fn ignore_above() -> Option<u32> { None }

	/// Should the field be searchable? Accepts `true` (default) or `false`.
	fn index() -> Option<bool> { None }

	/// What information should be stored in the index, for search and highlighting purposes. Defaults to `Positions`.
	fn index_options() -> Option<IndexOptions> { None }

	/// Whether field-length should be taken into account when scoring queries. Accepts `true` (default) or `false`.
	fn norms() -> Option<bool> { None }

	/// The number of fake term position which should be inserted between each element of an array of strings. 
	/// Defaults to the `position_increment_gap` configured on the analyzer which defaults to `100`. 
	/// `100` was chosen because it prevents phrase queries with reasonably large slops (less than `100`) 
	/// from matching terms across field values.
	fn position_increment_gap() -> Option<u32> { None }

	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> { None }

	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	fn search_analyzer() -> Option<&'static str> { None }


	/// The analyzer that should be used at search time when a phrase is encountered. 
	/// Defaults to the `search_analyzer` setting.
	fn search_quote_analyzer() -> Option<&'static str> { None }

	/// Which scoring algorithm or similarity should be used. 
	/// Defaults to `"classic"`, which uses TF/IDF. 
	fn similarity() -> Option<&'static str> { None }

	/// Whether term vectors should be stored for an `analyzed` field. 
	/// Defaults to `No`.
	fn term_vector() -> Option<TermVector> { None }
}

/// Implement `serde` serialisation for a `keyword` mapping type.
macro_rules! text_ser {
    ($t:ident) => (
		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 18));

				try!(serializer.serialize_struct_elt(&mut state, "type", $t::data_type()));

				ser_field!(serializer, &mut state, $t::boost(), "boost");
				ser_field!(serializer, &mut state, $t::analyzer(), "analyzer");
				ser_field!(serializer, &mut state, $t::eager_global_ordinals(), "eager_global_ordinals");
				ser_field!(serializer, &mut state, $t::fielddata(), "fielddata");
				ser_field!(serializer, &mut state, $t::fielddata_frequency_filter(), "fielddata_frequency_filter");
				ser_field!(serializer, &mut state, $t::fields(), "fields");
				ser_field!(serializer, &mut state, $t::include_in_all(), "include_in_all");
				ser_field!(serializer, &mut state, $t::ignore_above(), "ignore_above");
				ser_field!(serializer, &mut state, $t::index(), "index");
				ser_field!(serializer, &mut state, $t::index_options(), "index_options");
				ser_field!(serializer, &mut state, $t::norms(), "norms");
				ser_field!(serializer, &mut state, $t::position_increment_gap(), "position_increment_gap");
				ser_field!(serializer, &mut state, $t::store(), "store");
				ser_field!(serializer, &mut state, $t::search_analyzer(), "search_analyzer");
				ser_field!(serializer, &mut state, $t::search_quote_analyzer(), "search_quote_analyzer");
				ser_field!(serializer, &mut state, $t::similarity(), "similarity");
				ser_field!(serializer, &mut state, $t::term_vector(), "term_vector");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Define a `text` mapping.
/// 
/// # Examples
/// 
/// ## Define mapping struct inline
/// 
/// The easiest way to define a mapping type is to let the macro do it for you:
/// 
/// ```
/// text_mapping!(MyMapping {
/// 	fn boost() -> Option<f32> { Some(1.07) }
/// });
/// ```
/// 
/// The above example will define a public struct for you and implement
/// `ElasticFieldMapping` and `ElasticTextMapping`, along with a few default traits:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping;
/// ```
/// 
/// ## Define mapping for existing struct
/// 
/// If you want to control the default implementations yourself, you can define your
/// mapping type and just pass it the macro to implement `ElasticFieldMapping`:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping;
/// impl ElasticTextMapping for MyMapping { 
/// 	fn boost() -> Option<f32> { Some(1.07) }
/// }
/// 
/// text_mapping!(MyMapping);
/// ```
#[macro_export]
macro_rules! text_mapping {
	($t:ident) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			fn data_type() -> &'static str { $crate::string::text::mapping::TEXT_DATATYPE }
		}

		text_ser!($t);
	);
	($t:ident $b:tt) => (
		#[derive(Debug, Default, Clone, Copy)]
		pub struct $t;

		impl $crate::string::text::mapping::ElasticTextMapping for $t $b

		text_mapping!($t);
	)
}

/// Default mapping for `bool`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultTextMapping;
impl ElasticTextMapping for DefaultTextMapping { }

text_mapping!(DefaultTextMapping);

/// Term vectors contain information about the terms produced by the analysis process.
#[derive(Debug, Clone, Copy)]
pub enum TermVector {
	/// No term vectors are stored. (default)
	No,
	/// Just the terms in the field are stored.
	Yes,
	/// Terms and positions are stored.
	WithPositions,
	/// Terms and character offsets are stored.
	WithOffsets,
	/// Terms, positions, and character offsets are stored.
	WithPositionsOffsets
}

impl serde::Serialize for TermVector {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			TermVector::No => "no",
			TermVector::Yes => "yes",
			TermVector::WithPositions => "with_positions",
			TermVector::WithOffsets => "with_offsets",
			TermVector::WithPositionsOffsets => "with_positions_offsets"
		})
	}
}

/// Fielddata for term frequency as a percentage range.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct FieldDataFrequencyFilter {
	/// The min frequency percentage.
	pub min: Option<f32>,
	/// The max frequency percentage.
	pub max: Option<f32>,
	/// The minimum number of docs a segment should contain.
	pub min_segment_size: Option<i32>
}

impl serde::Serialize for FieldDataFrequencyFilter {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: serde::Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 3));

		ser_field!(serializer, &mut state, self.min, "min");
		ser_field!(serializer, &mut state, self.max, "max");
		ser_field!(serializer, &mut state, self.min_segment_size, "min_segment_size");

		serializer.serialize_struct_end(state)
	}
}

/// A multi-field string mapping.
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticTextFieldMapping {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	pub analyzer: Option<&'static str>,
	/// Should global ordinals be loaded eagerly on refresh? 
	/// Accepts `true` or `false` (default).
	/// Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations. 
	pub eager_global_ordinals: Option<bool>,
	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting? 
	/// Accepts `true` or `false` (default).
	pub fielddata: Option<bool>,
	/// Expert settings which allow to decide which values to load in memory when `fielddata` is enabled. 
	/// By default all values are loaded.
	pub fielddata_frequency_filter: Option<FieldDataFrequencyFilter>,
	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	pub include_in_all: Option<bool>,
	/// The maximum number of characters to index.
	/// Any characters over this length will be ignored.
	pub ignore_above: Option<u32>,
	/// Should the field be searchable? Accepts `true` (default) or `false`.
	pub index: Option<bool>,
	/// What information should be stored in the index, for search and highlighting purposes. Defaults to `Positions`.
	pub index_options: Option<IndexOptions>,
	/// Whether field-length should be taken into account when scoring queries. Accepts `true` (default) or `false`.
	pub norms: Option<bool>,
	/// The number of fake term position which should be inserted between each element of an array of strings. 
	/// Defaults to the `position_increment_gap` configured on the analyzer which defaults to `100`. 
	/// `100` was chosen because it prevents phrase queries with reasonably large slops (less than `100`) 
	/// from matching terms across field values.
	pub position_increment_gap: Option<u32>,
	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	pub store: Option<bool>,
	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	pub search_analyzer: Option<&'static str>,
	/// The analyzer that should be used at search time when a phrase is encountered. 
	/// Defaults to the `search_analyzer` setting.
	pub search_quote_analyzer: Option<&'static str>,
	/// Which scoring algorithm or similarity should be used. 
	/// Defaults to `"classic"`, which uses TF/IDF. 
	pub similarity: Option<&'static str>,
	/// Whether term vectors should be stored for an `analyzed` field. 
	/// Defaults to `No`.
	pub term_vector: Option<TermVector>
}

impl serde::Serialize for ElasticTextFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 16));

		try!(serializer.serialize_struct_elt(&mut state, "type", TEXT_DATATYPE));

		ser_field!(serializer, &mut state, self.analyzer, "analyzer");
		ser_field!(serializer, &mut state, self.eager_global_ordinals, "eager_global_ordinals");
		ser_field!(serializer, &mut state, self.fielddata, "fielddata");
		ser_field!(serializer, &mut state, self.fielddata_frequency_filter, "fielddata_frequency_filter");
		ser_field!(serializer, &mut state, self.include_in_all, "include_in_all");
		ser_field!(serializer, &mut state, self.ignore_above, "ignore_above");
		ser_field!(serializer, &mut state, self.index, "index");
		ser_field!(serializer, &mut state, self.index_options, "index_options");
		ser_field!(serializer, &mut state, self.norms, "norms");
		ser_field!(serializer, &mut state, self.position_increment_gap, "position_increment_gap");
		ser_field!(serializer, &mut state, self.store, "store");
		ser_field!(serializer, &mut state, self.search_analyzer, "search_analyzer");
		ser_field!(serializer, &mut state, self.search_quote_analyzer, "search_quote_analyzer");
		ser_field!(serializer, &mut state, self.similarity, "similarity");
		ser_field!(serializer, &mut state, self.term_vector, "term_vector");

		serializer.serialize_struct_end(state)
	}
}