//! Mapping for the Elasticsearch `keyword` type.

use std::collections::BTreeMap;
use serde::{ self, Serialize, Serializer };
use ::mapping::ElasticFieldMapping;
use ::string::mapping::{ ElasticStringField, IndexOptions };

/// Elasticsearch datatype name.
pub const KEYWORD_DATATYPE: &'static str = "keyword";

/// The base requirements for mapping a `string` type.
///
/// Custom mappings can be defined by implementing `ElasticKeywordMapping`.
///
/// # Examples
///
/// Define a custom `ElasticKeywordMapping`:
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
/// #[derive(Debug, Clone, Default, ElasticKeywordMapping)]
/// pub struct MyStringMapping;
/// impl ElasticKeywordMapping for MyStringMapping {
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
/// # #[derive(Debug, Clone, Default, ElasticKeywordMapping)]
/// # pub struct MyStringMapping;
/// # impl ElasticKeywordMapping for MyStringMapping {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyStringMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "keyword",
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
/// impl ElasticKeywordMapping for MyStringMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// //We also need to implement the base `ElasticFieldMapping` and `serde::Serialize` for our custom mapping type
/// impl ElasticFieldMapping<()> for MyStringMapping {
/// 	type Visitor = ElasticKeywordMappingVisitor<MyStringMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		KEYWORD_DATATYPE
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
pub trait ElasticKeywordMapping where
Self: ElasticFieldMapping<()> + Sized + Serialize {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	fn analyzer() -> Option<&'static str> { None }

	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<f32> { None }

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> { None }

	/// Should global ordinals be loaded eagerly on refresh? 
	/// Accepts `true` or `false` (default).
	/// Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations. 
	fn eager_global_ordinals() -> Option<bool> { None }

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
	/// # #[derive(Debug, Clone, Default, ElasticKeywordMapping)]
	/// # pub struct MyStringMapping;
	/// # impl ElasticKeywordMapping for MyStringMapping {
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

	/// Accepts a `string` value which is substituted for any explicit null values. 
	/// Defaults to `null`, which means the field is treated as missing. 
	fn null_value() -> Option<&'static str> { None }

	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> { None }

	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	fn search_analyzer() -> Option<&'static str> { None }

	/// Which scoring algorithm or similarity should be used. 
	/// Defaults to `"classic"`, which uses TF/IDF. 
	fn similarity() -> Option<&'static str> { None }
}

/// Implement `serde` serialisation for a `keyword` mapping type.
macro_rules! keyword_ser {
    ($t:ident) => (
		impl serde::Serialize for $t {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
			where S: serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 15));

				try!(serializer.serialize_struct_elt(&mut state, "type", $t::data_type()));

				ser_field!(serializer, &mut state, $t::boost(), "boost");
				ser_field!(serializer, &mut state, $t::analyzer(), "analyzer");
				ser_field!(serializer, &mut state, $t::doc_values(), "doc_values");
				ser_field!(serializer, &mut state, $t::eager_global_ordinals(), "eager_global_ordinals");
				ser_field!(serializer, &mut state, $t::fields(), "fields");
				ser_field!(serializer, &mut state, $t::include_in_all(), "include_in_all");
				ser_field!(serializer, &mut state, $t::ignore_above(), "ignore_above");
				ser_field!(serializer, &mut state, $t::index(), "index");
				ser_field!(serializer, &mut state, $t::index_options(), "index_options");
				ser_field!(serializer, &mut state, $t::norms(), "norms");
				ser_field!(serializer, &mut state, $t::null_value(), "null_value");
				ser_field!(serializer, &mut state, $t::store(), "store");
				ser_field!(serializer, &mut state, $t::search_analyzer(), "search_analyzer");
				ser_field!(serializer, &mut state, $t::similarity(), "similarity");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Define a `keyword` mapping.
/// 
/// # Examples
/// 
/// ## Define mapping struct inline
/// 
/// The easiest way to define a mapping type is to let the macro do it for you:
/// 
/// ```
/// keyword_mapping!(MyMapping {
/// 	fn boost() -> Option<f32> { Some(1.07) }
/// });
/// ```
/// 
/// The above example will define a public struct for you and implement
/// `ElasticFieldMapping` and `ElasticKeywordMapping`, along with a few default traits:
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
/// impl ElasticKeywordMapping for MyMapping { 
/// 	fn boost() -> Option<f32> { Some(1.07) }
/// }
/// 
/// keyword_mapping!(MyMapping);
/// ```
#[macro_export]
macro_rules! keyword_mapping {
	($t:ident) => (
		impl $crate::mapping::ElasticFieldMapping<()> for $t {
			fn data_type() -> &'static str { $crate::string::keyword::mapping::KEYWORD_DATATYPE }
		}

		keyword_ser!($t);
	);
	($t:ident $b:tt) => (
		#[derive(Debug, Default, Clone, Copy)]
		pub struct $t;

		impl $crate::string::keyword::mapping::ElasticKeywordMapping for $t $b

		keyword_mapping!($t);
	)
}

/// Default mapping for `bool`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultKeywordMapping;
impl ElasticKeywordMapping for DefaultKeywordMapping { }

keyword_mapping!(DefaultKeywordMapping);

/// A multi-field string mapping.
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticKeywordFieldMapping {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	pub analyzer: Option<&'static str>,
	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	pub doc_values: Option<bool>,
	/// Should global ordinals be loaded eagerly on refresh? 
	/// Accepts `true` or `false` (default).
	/// Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations. 
	pub eager_global_ordinals: Option<bool>,
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
	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	pub store: Option<bool>,
	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	pub search_analyzer: Option<&'static str>,
	/// Which scoring algorithm or similarity should be used. 
	/// Defaults to `"classic"`, which uses TF/IDF. 
	pub similarity: Option<&'static str>
}

impl serde::Serialize for ElasticKeywordFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 12));

		try!(serializer.serialize_struct_elt(&mut state, "type", KEYWORD_DATATYPE));

		ser_field!(serializer, &mut state, self.analyzer, "analyzer");
		ser_field!(serializer, &mut state, self.doc_values, "doc_values");
		ser_field!(serializer, &mut state, self.eager_global_ordinals, "eager_global_ordinals");
		ser_field!(serializer, &mut state, self.include_in_all, "include_in_all");
		ser_field!(serializer, &mut state, self.ignore_above, "ignore_above");
		ser_field!(serializer, &mut state, self.index, "index");
		ser_field!(serializer, &mut state, self.index_options, "index_options");
		ser_field!(serializer, &mut state, self.norms, "norms");
		ser_field!(serializer, &mut state, self.store, "store");
		ser_field!(serializer, &mut state, self.search_analyzer, "search_analyzer");
		ser_field!(serializer, &mut state, self.similarity, "similarity");

		serializer.serialize_struct_end(state)
	}
}