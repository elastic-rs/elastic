//! Mapping for the Elasticsearch `string` type.

use std::collections::BTreeMap;
use std::marker::PhantomData;
use serde;
use serde::{ Serializer, Serialize };
use ::mapping::{ ElasticTypeMapping, ElasticTypeVisitor, IndexAnalysis };

/// The base requirements for mapping a `string` type.
///
/// Custom mappings can be defined by implementing `ElasticStringMapping`.
///
/// # Examples
///
/// Define a custom `ElasticStringMapping`:
///
/// ## Derive Mapping
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(elastic_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::string::prelude::*;
///
/// #[derive(Debug, Clone, Default, ElasticStringMapping)]
/// pub struct MyStringMapping;
/// impl ElasticStringMapping for MyStringMapping {
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
/// # #![plugin(elastic_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::mapping::prelude::*;
/// # use elastic_types::string::prelude::*;
/// # #[derive(Debug, Clone, Default, ElasticStringMapping)]
/// # pub struct MyStringMapping;
/// # impl ElasticStringMapping for MyStringMapping {
/// # 	//Overload the mapping functions here
/// # 	fn boost() -> Option<f32> {
///	# 		Some(1.5)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyStringMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "string",
/// 	"boost": 1.5
/// }
/// # );
/// # assert_eq!(json, &mapping);
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
/// impl ElasticStringMapping for MyStringMapping {
/// 	//Overload the mapping functions here
/// 	fn boost() -> Option<f32> {
///			Some(1.5)
///		}
/// }
///
/// //We also need to implement the base `ElasticTypeMapping` and `serde::Serialize` for our custom mapping type
/// impl ElasticTypeMapping<()> for MyStringMapping {
/// 	type Visitor = ElasticStringMappingVisitor<MyStringMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		"string"
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
pub trait ElasticStringMapping where
Self: ElasticTypeMapping<()> + Sized + Serialize {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn boost() -> Option<f32> {
		None
	}

	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	fn doc_values() -> Option<bool> {
		None
	}

	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	fn include_in_all() -> Option<bool> {
		None
	}

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn index() -> Option<IndexAnalysis> {
		None
	}

	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	fn store() -> Option<bool> {
		None
	}

	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	fn analyzer() -> Option<&'static str> {
		None
	}

	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting?
	/// Accepts disabled or `paged_bytes` (default).
	/// Not analyzed fields will use doc values in preference to fielddata.
	fn fielddata() -> Option<FieldData> {
		None
	}

	/// Multi-fields allow the same string value to be indexed in multiple ways for different purposes,
	/// such as one field for search and a multi-field for sorting and aggregations,
	/// or the same string value analyzed by different analyzers.
	fn fields() -> Option<BTreeMap<&'static str, ElasticStringFieldMapping>> {
		None
	}

	/// Do not index or analyze any string longer than this value. Defaults to 0 (disabled).
	fn ignore_above() -> Option<usize> {
		None
	}

	/// What information should be stored in the index, for search and highlighting purposes.
	/// Defaults to positions for analyzed fields, and to docs for not_analyzed fields.
	fn index_options() -> Option<IndexOptions> {
		None
	}

	/// Whether field-length should be taken into account when scoring queries.
	fn norms() -> Option<Norms> {
		None
	}

	/// Accepts a string value which is substituted for any explicit null values.
	/// Defaults to `null`, which means the field is treated as missing.
	fn null_value() -> Option<&'static str> {
		None
	}

	/// Whether field-length should be taken into account when scoring queries.
	fn position_increment_gap() -> Option<usize> {
		None
	}

	/// The analyzer that should be used at search time on analyzed fields.
	/// Defaults to the analyzer setting.
	fn search_analyzer() -> Option<&'static str> {
		None
	}

	/// The analyzer that should be used at search time when a phrase is encountered.
	/// Defaults to the search_analyzer setting.
	fn search_quote_analyzer() -> Option<&'static str> {
		None
	}

	/// Which scoring algorithm or similarity should be used. Defaults to `default`, which uses TF/IDF.
	fn similarity() -> Option<&'static str> {
		None
	}

	/// Whether term vectors should be stored for an analyzed field. Defaults to `no`.
	fn term_vector() -> Option<TermVector> {
		None
	}
}

/// Default mapping for `String`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultStringMapping;
impl ElasticStringMapping for DefaultStringMapping { }

impl_string_mapping!(DefaultStringMapping);

/// Base visitor for serialising string mappings.
#[derive(Debug, PartialEq)]
pub struct ElasticStringMappingVisitor<T> where T: ElasticStringMapping {
	phantom: PhantomData<T>
}

impl <T> ElasticTypeVisitor for ElasticStringMappingVisitor<T> where
T: ElasticStringMapping {
	fn new() -> Self {
		ElasticStringMappingVisitor {
			phantom: PhantomData
		}
	}
}
impl <T> serde::ser::MapVisitor for ElasticStringMappingVisitor<T> where
T: ElasticStringMapping {
	#[cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity))]
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", T::data_type()));

		if let Some(boost) = T::boost() {
			try!(serializer.serialize_struct_elt("boost", boost));
		}

		if let Some(doc_values) = T::doc_values() {
			try!(serializer.serialize_struct_elt("doc_values", doc_values));
		}

		if let Some(include_in_all) = T::include_in_all() {
			try!(serializer.serialize_struct_elt("include_in_all", include_in_all));
		}

		if let Some(index) = T::index() {
			try!(serializer.serialize_struct_elt("index", index));
		}

		if let Some(store) = T::store() {
			try!(serializer.serialize_struct_elt("store", store));
		}

		if let Some(analyzer) = T::analyzer() {
			try!(serializer.serialize_struct_elt("analyzer", analyzer));
		}

		if let Some(fields) = T::fields() {
			try!(serializer.serialize_struct_elt("fields", fields));
		}

		match T::fielddata() {
			Some(FieldData::PagedBytes(None, None)) => (),
			Some(fielddata) => try!(serializer.serialize_struct_elt("fielddata", fielddata)),
			None => ()
		}

		if let Some(ignore_above) = T::ignore_above() {
			try!(serializer.serialize_struct_elt("ignore_above", ignore_above));
		}

		if let Some(index_options) = T::index_options() {
			try!(serializer.serialize_struct_elt("index_options", index_options));
		}

		if let Some(norms) = T::norms() {
			try!(serializer.serialize_struct_elt("norms", norms));
		}

		if let Some(null_value) = T::null_value() {
			try!(serializer.serialize_struct_elt("null_value", null_value));
		}

		if let Some(position_increment_gap) = T::position_increment_gap() {
			try!(serializer.serialize_struct_elt("position_increment_gap", position_increment_gap));
		}

		if let Some(search_analyzer) = T::search_analyzer() {
			try!(serializer.serialize_struct_elt("search_analyzer", search_analyzer));
		}

		if let Some(search_quote_analyzer) = T::search_quote_analyzer() {
			try!(serializer.serialize_struct_elt("search_quote_analyzer", search_quote_analyzer))
		}

		if let Some(similarity) = T::similarity() {
			try!(serializer.serialize_struct_elt("similarity", similarity))
		}

		if let Some(term_vector) = T::term_vector() {
			try!(serializer.serialize_struct_elt("term_vector", term_vector));
		}

		Ok(None)
	}
}

/// A multi-field string mapping.
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticStringFieldMapping {
/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	pub analyzer: Option<&'static str>,
	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting?
	/// Accepts disabled or `paged_bytes` (default).
	/// Not analyzed fields will use doc values in preference to fielddata.
	pub fielddata: Option<FieldData>,
	/// Do not index or analyze any string longer than this value. Defaults to 0 (disabled).
	pub ignore_above: Option<usize>,
	/// What information should be stored in the index, for search and highlighting purposes.
	/// Defaults to positions for analyzed fields, and to docs for not_analyzed fields.
	pub index_options: Option<IndexOptions>,
	/// Whether field-length should be taken into account when scoring queries.
	pub norms: Option<Norms>,
	/// Accepts a string value which is substituted for any explicit null values.
	/// Defaults to null, which means the field is treated as missing.
	pub null_value: Option<&'static str>,
	/// Whether field-length should be taken into account when scoring queries.
	pub position_increment_gap: Option<usize>,
	/// The analyzer that should be used at search time on analyzed fields.
	/// Defaults to the analyzer setting.
	pub search_analyzer: Option<&'static str>,
	/// The analyzer that should be used at search time when a phrase is encountered.
	/// Defaults to the search_analyzer setting.
	pub search_quote_analyzer: Option<&'static str>,
	/// Which scoring algorithm or similarity should be used. Defaults to `default`, which uses TF/IDF.
	pub similarity: Option<&'static str>,
	/// Whether term vectors should be stored for an analyzed field. Defaults to `no`.
	pub term_vector: Option<TermVector>
}

impl serde::Serialize for ElasticStringFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_struct("fields", ElasticStringFieldMappingVisitor::new(&self))
	}
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ElasticStringFieldMappingVisitor<'a> {
	data: &'a ElasticStringFieldMapping
}
impl <'a> ElasticStringFieldMappingVisitor<'a> {
	#[doc(hidden)]
	pub fn new(field: &'a ElasticStringFieldMapping) -> Self {
		ElasticStringFieldMappingVisitor {
			data: field
		}
	}
}

impl <'a> serde::ser::MapVisitor for ElasticStringFieldMappingVisitor<'a> {
	#[cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity))]
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		if let Some(analyzer) = self.data.analyzer {
			try!(serializer.serialize_struct_elt("analyzer", analyzer));
		}

		match self.data.fielddata {
			Some(FieldData::PagedBytes(None, None)) => (),
			Some(fielddata) => try!(serializer.serialize_struct_elt("fielddata", fielddata)),
			None => ()
		}

		if let Some(ignore_above) = self.data.ignore_above {
			try!(serializer.serialize_struct_elt("ignore_above", ignore_above));
		}

		if let Some(index_options) = self.data.index_options {
			try!(serializer.serialize_struct_elt("index_options", index_options));
		}

		if let Some(norms) = self.data.norms {
			try!(serializer.serialize_struct_elt("norms", norms));
		}

		if let Some(null_value) = self.data.null_value {
			try!(serializer.serialize_struct_elt("null_value", null_value));
		}

		if let Some(position_increment_gap) = self.data.position_increment_gap {
			try!(serializer.serialize_struct_elt("position_increment_gap", position_increment_gap));
		}

		if let Some(search_analyzer) = self.data.search_analyzer {
			try!(serializer.serialize_struct_elt("search_analyzer", search_analyzer));
		}

		if let Some(search_quote_analyzer) = self.data.search_quote_analyzer {
			try!(serializer.serialize_struct_elt("search_quote_analyzer", search_quote_analyzer))
		}

		if let Some(similarity) = self.data.similarity {
			try!(serializer.serialize_struct_elt("similarity", similarity))
		}

		if let Some(term_vector) = self.data.term_vector {
			try!(serializer.serialize_struct_elt("term_vector", term_vector));
		}

		Ok(None)
	}
}

/// Can the field use in memory fielddata for sorting, aggregations, or scripting?
#[derive(Debug, Clone, Copy)]
pub enum FieldData {
	/// Allow in-memory fielddata.
	PagedBytes(Option<FieldDataLoading>, Option<FieldDataFilter>),
	/// Disallow in-memory fielddata.
	Disabled
}

impl serde::Serialize for FieldData {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		match *self {
			FieldData::PagedBytes(None, None) => Ok(()),
			fielddata => serializer.serialize_struct("fielddata", FieldDataVisitor::new(fielddata))
		}
	}
}

struct FieldDataVisitor {
	value: FieldData
}
impl FieldDataVisitor {
	pub fn new(value: FieldData) -> FieldDataVisitor {
		FieldDataVisitor {
			value: value
		}
	}
}

impl serde::ser::MapVisitor for FieldDataVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		match self.value {
			FieldData::Disabled => try!(serializer.serialize_struct_elt("format", "disabled")),
			FieldData::PagedBytes(loading, filter) => {
				if let Some(loading) = loading {
					try!(serializer.serialize_struct_elt("loading", loading));
				}

				if let Some(filter) = filter {
					try!(serializer.serialize_struct_elt("filter", filter));
				}
			}
		}

		Ok(None)
	}
}

/// This per-field setting controls when fielddata is loaded into memory.
#[derive(Debug, Clone, Copy)]
pub enum FieldDataLoading {
	/// Fielddata is only loaded into memory when it is needed. (default).
	Lazy,
	/// Fielddata is loaded into memory before a new search segment becomes visible to search.
	/// This can reduce the latency that a user may experience if their search request has to
	/// trigger lazy loading from a big segment.
	Eager,
	/// Loading fielddata into memory is only part of the work that is required.
	/// After loading the fielddata for each segment, Elasticsearch builds the
	/// Global ordinals data structure to make a list of all unique terms across all the segments in a shard.
	/// By default, global ordinals are built lazily. If the field has a very high cardinality,
	/// global ordinals may take some time to build, in which case you can use eager loading instead.
	EagerGlobalOrdinals
}

impl serde::Serialize for FieldDataLoading {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			FieldDataLoading::Lazy => "lazy",
			FieldDataLoading::Eager => "eager",
			FieldDataLoading::EagerGlobalOrdinals => "eager_global_ordinals"
		})
	}
}

/// Fielddata filtering can be used to reduce the number of terms loaded into memory, and thus reduce memory usage.
#[derive(Debug, Clone, Copy)]
pub enum FieldDataFilter {
	/// The frequency filter allows you to only load terms whose term frequency falls between a min and max value,
	/// which can be expressed an absolute number (when the number is bigger than 1.0) or as a percentage (eg 0.01 is 1% and 1.0 is 100%).
	/// Frequency is calculated per segment. Percentages are based on the number of docs which have a value for the field,
	/// as opposed to all docs in the segment.
	Frequency(FrequencyFilter),
	/// Terms can also be filtered by regular expression - only values which match the regular expression are loaded.
	/// Note: the regular expression is applied to each term in the field, not to the whole field value.
	Regex(RegexFilter)
}

/// The frequency filter allows you to only load terms whose term frequency falls between a min and max value,
/// which can be expressed an absolute number (when the number is bigger than 1.0) or as a percentage (eg 0.01 is 1% and 1.0 is 100%).
/// Frequency is calculated per segment. Percentages are based on the number of docs which have a value for the field,
/// as opposed to all docs in the segment.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct FrequencyFilter {
	/// The min frequency.
	pub min: f32,
	/// The max frequency.
	pub max: f32,
	/// The minimum segment size before loading.
	pub min_segment_size: usize
}

/// Terms can also be filtered by regular expression - only values which match the regular expression are loaded.
/// Note: the regular expression is applied to each term in the field, not to the whole field value.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct RegexFilter {
	/// The regex pattern.
	pub pattern: &'static str
}

impl serde::Serialize for FieldDataFilter {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_struct("filter", FieldDataFilterVisitor::new(*self))
	}
}

struct FieldDataFilterVisitor {
	value: FieldDataFilter
}
impl FieldDataFilterVisitor {
	pub fn new(value: FieldDataFilter) -> FieldDataFilterVisitor {
		FieldDataFilterVisitor {
			value: value
		}
	}
}

impl serde::ser::MapVisitor for FieldDataFilterVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		match self.value {
			FieldDataFilter::Frequency(freq) => try!(serializer.serialize_struct_elt("frequency", freq)),
			FieldDataFilter::Regex(pat) => try!(serializer.serialize_struct_elt("regex", pat))
		}

		Ok(None)
	}
}

/// The index_options parameter controls what information is added to the inverted index, for search and highlighting purposes.
#[derive(Debug, Clone, Copy)]
pub enum IndexOptions {
	/// Only the doc number is indexed. Can answer the question Does this term exist in this field?
	Docs,
	/// Doc number and term frequencies are indexed.
	/// Term frequencies are used to score repeated terms higher than single terms.
	Freqs,
	/// Doc number, term frequencies, and term positions (or order) are indexed.
	/// Positions can be used for proximity or phrase queries.
	Positions,
	/// Doc number, term frequencies, positions,
	/// and start and end character offsets (which map the term back to the original string) are indexed.
	/// Offsets are used by the postings highlighter.
	Offsets
}

impl serde::Serialize for IndexOptions {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			IndexOptions::Docs => "docs",
			IndexOptions::Freqs => "freqs",
			IndexOptions::Positions => "positions",
			IndexOptions::Offsets => "offsets"
		})
	}
}

/// Whether field-length should be taken into account when scoring queries.
#[derive(Debug, Clone, Copy)]
pub enum Norms {
	/// Enabled norms with eagerness.
	Enabled {
		/// Whether the loading is eager or lazy.
		loading: NormsLoading
	},
	/// Disabled norms.
	Disabled
}

struct NormsVisitor {
	value: Norms
}
impl NormsVisitor {
	pub fn new(value: Norms) -> NormsVisitor {
		NormsVisitor {
			value: value
		}
	}
}

impl serde::ser::MapVisitor for NormsVisitor {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		match self.value {
			Norms::Enabled { loading: l } => try!(serializer.serialize_struct_elt("loading", l)),
			Norms::Disabled => try!(serializer.serialize_struct_elt("enabled", false))
		}

		Ok(None)
	}
}

impl serde::Serialize for Norms {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_struct("norms", NormsVisitor::new(*self))
	}
}

/// Whether the norms should be loaded into memory eagerly (`eager`),
/// whenever a new segment comes online, or they can loaded lazily (`lazy`, default).
#[derive(Debug, Clone, Copy)]
pub enum NormsLoading {
	/// Load norms whenever a new segment comes online.
	Eager,
	/// Load norms lazily.
	Lazy
}

impl serde::Serialize for NormsLoading {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_str(match *self {
			NormsLoading::Eager => "eager",
			NormsLoading::Lazy => "lazy"
		})
	}
}

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
