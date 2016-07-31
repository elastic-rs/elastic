//! Mapping for the Elasticsearch `keyword` type.

use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde::{ self, Serialize, Serializer };
use ::mapping::{ ElasticFieldMapping, ElasticTypeVisitor };
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

/// Default mapping for `text`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultKeywordMapping;
impl ElasticKeywordMapping for DefaultKeywordMapping { }

impl_keyword_mapping!(DefaultKeywordMapping);

/// Base visitor for serialising string mappings.
#[derive(Debug, PartialEq)]
pub struct ElasticKeywordMappingVisitor<M> where M: ElasticKeywordMapping {
	phantom: PhantomData<M>
}

impl <M> ElasticTypeVisitor for ElasticKeywordMappingVisitor<M> where
M: ElasticKeywordMapping {
	fn new() -> Self {
		ElasticKeywordMappingVisitor {
			phantom: PhantomData
		}
	}
}
impl <M> serde::ser::MapVisitor for ElasticKeywordMappingVisitor<M> where
M: ElasticKeywordMapping {
	#[cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity))]
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", M::data_type()));

		ser_field!(serializer, M::boost(), "boost");
		ser_field!(serializer, M::analyzer(), "analyzer");
		ser_field!(serializer, M::doc_values(), "doc_values");
		ser_field!(serializer, M::eager_global_ordinals(), "eager_global_ordinals");
		ser_field!(serializer, M::fields(), "fields");
		ser_field!(serializer, M::include_in_all(), "include_in_all");
		ser_field!(serializer, M::ignore_above(), "ignore_above");
		ser_field!(serializer, M::index(), "index");
		ser_field!(serializer, M::index_options(), "index_options");
		ser_field!(serializer, M::norms(), "norms");
		ser_field!(serializer, M::null_value(), "null_value");
		ser_field!(serializer, M::store(), "store");
		ser_field!(serializer, M::search_analyzer(), "search_analyzer");
		ser_field!(serializer, M::similarity(), "similarity");

		Ok(None)
	}
}

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
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_struct("fields", ElasticKeywordFieldMappingVisitor::new(&self))
	}
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ElasticKeywordFieldMappingVisitor<'a> {
	data: &'a ElasticKeywordFieldMapping
}
impl <'a> ElasticKeywordFieldMappingVisitor<'a> {
	#[doc(hidden)]
	pub fn new(field: &'a ElasticKeywordFieldMapping) -> Self {
		ElasticKeywordFieldMappingVisitor {
			data: field
		}
	}
}

impl <'a> serde::ser::MapVisitor for ElasticKeywordFieldMappingVisitor<'a> {
	#[cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity))]
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", KEYWORD_DATATYPE));

		ser_sub_field!(serializer, self.data.analyzer, "analyzer");
		ser_sub_field!(serializer, self.data.doc_values, "doc_values");
		ser_sub_field!(serializer, self.data.eager_global_ordinals, "eager_global_ordinals");
		ser_sub_field!(serializer, self.data.include_in_all, "include_in_all");
		ser_sub_field!(serializer, self.data.ignore_above, "ignore_above");
		ser_sub_field!(serializer, self.data.index, "index");
		ser_sub_field!(serializer, self.data.index_options, "index_options");
		ser_sub_field!(serializer, self.data.norms, "norms");
		ser_sub_field!(serializer, self.data.store, "store");
		ser_sub_field!(serializer, self.data.search_analyzer, "search_analyzer");
		ser_sub_field!(serializer, self.data.similarity, "similarity");

		Ok(None)
	}
}