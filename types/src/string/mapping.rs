//! Common mapping for the Elasticsearch `string` types.

use std::collections::BTreeMap;
use serde::{ self, Serializer };
use ::mapping::{ ElasticFieldMapping, IndexAnalysis };

pub use super::keyword::mapping::*;
pub use super::text::mapping::*;

macro_rules! ser_sub_field {
	($s:ident, $f:expr, $n:expr) => (
		if let Some(f) = $f {
			try!($s.serialize_struct_elt($n, f));
		}
	)
}

/// Elasticsearch datatype name.
pub const TOKENCOUNT_DATATYPE: &'static str = "token_count";
/// Elasticsearch datatype name.
pub const COMPLETION_DATATYPE: &'static str = "completion";

/// Default mapping for `String`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultStringMapping;
impl ElasticTextMapping for DefaultStringMapping {
	fn fields() -> Option<BTreeMap<&'static str, ElasticStringField>> {
		let mut fields = BTreeMap::new();

		fields.insert("keyword", ElasticStringField::Keyword(
			ElasticKeywordFieldMapping {
				ignore_above: Some(256),
				..Default::default()
			})
		);

		Some(fields)
	}
}

impl_text_mapping!(DefaultStringMapping);

/// The `index_options` parameter controls what information is added to the inverted index, for search and highlighting purposes.
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

/// A string sub-field type.
///
/// String types can have a number of alternative field representations for different purposes.
#[derive(Debug, Clone, Copy)]
pub enum ElasticStringField {
	/// A `token_count` sub field.
	TokenCount(ElasticTokenCountFieldMapping),
	/// A `completion` suggester sub field.
	Completion(ElasticCompletionFieldMapping),
	/// A `keyword` sub field.
	Keyword(ElasticKeywordFieldMapping),
	/// A `text` sub field.
	Text(ElasticTextFieldMapping)
}

impl serde::Serialize for ElasticStringField {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		match *self {
			ElasticStringField::TokenCount(m) => m.serialize(serializer),
			ElasticStringField::Completion(m) => m.serialize(serializer),
			ElasticStringField::Keyword(m) => m.serialize(serializer),
			ElasticStringField::Text(m) => m.serialize(serializer)
		}
	}
}

/// A multi-field string mapping for a [token count](https://www.elastic.co/guide/en/elasticsearch/reference/current/token-count.html).
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticTokenCountFieldMapping {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	pub analyzer: Option<&'static str>,
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	pub boost: Option<f32>,
	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	pub doc_values: Option<bool>,
	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	pub index: Option<IndexAnalysis>,
	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	pub include_in_all: Option<bool>,
	/// Controls the number of extra terms that are indexed to make range queries faster.
	/// Defaults to `32`.
	pub precision_step: Option<u32>,
	/// Whether the field value should be stored and retrievable separately from the `_source` field.
	/// Accepts `true` or `false` (default).
	pub store: Option<bool>
}

impl serde::Serialize for ElasticTokenCountFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_struct("fields", ElasticTokenCountFieldMappingVisitor::new(&self))
	}
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ElasticTokenCountFieldMappingVisitor<'a> {
	data: &'a ElasticTokenCountFieldMapping
}
impl <'a> ElasticTokenCountFieldMappingVisitor<'a> {
	#[doc(hidden)]
	pub fn new(field: &'a ElasticTokenCountFieldMapping) -> Self {
		ElasticTokenCountFieldMappingVisitor {
			data: field
		}
	}
}

impl <'a> serde::ser::MapVisitor for ElasticTokenCountFieldMappingVisitor<'a> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", TOKENCOUNT_DATATYPE));

		ser_sub_field!(serializer, self.data.analyzer, "analyzer");
		ser_sub_field!(serializer, self.data.boost, "boost");
		ser_sub_field!(serializer, self.data.doc_values, "doc_values");
		ser_sub_field!(serializer, self.data.index, "index");
		ser_sub_field!(serializer, self.data.include_in_all, "include_in_all");
		ser_sub_field!(serializer, self.data.precision_step, "precision_step");
		ser_sub_field!(serializer, self.data.store, "store");

		Ok(None)
	}
}

/// A multi-field string mapping for a [completion suggester](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-suggesters-completion.html#search-suggesters-completion).
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticCompletionFieldMapping {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	pub analyzer: Option<&'static str>,
	/// The search analyzer to use, defaults to value of analyzer.
	pub search_analyzer: Option<&'static str>,
	/// Enables the storing of payloads, defaults to `false`.
	pub payloads: Option<bool>,
	/// Preserves the separators, defaults to `true`.
	/// If disabled, you could find a field starting with Foo Fighters,
	/// if you suggest for foof.
	pub preserve_separators: Option<bool>,
	/// Enables position increments, defaults to `true`.
	/// If disabled and using stopwords analyzer,
	/// you could get a field starting with The Beatles, if you suggest for b.
	/// > Note: You could also achieve this by indexing two inputs, Beatles and The Beatles,
	/// no need to change a simple analyzer, if you are able to enrich your data.
	pub preserve_position_increments: Option<bool>,
	/// Limits the length of a single input, defaults to `50` `UTF-16` code points.
	/// This limit is only used at index time to reduce the total number of characters per input
	/// string in order to prevent massive inputs from bloating the underlying datastructure.
	/// The most usecases won’t be influenced by the default value since prefix completions
	/// hardly grow beyond prefixes longer than a handful of characters.
	/// (Old name "max_input_len" is deprecated)
	pub max_input_length: Option<u32>
}

impl serde::Serialize for ElasticCompletionFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		serializer.serialize_struct("fields", ElasticCompletionFieldMappingVisitor::new(&self))
	}
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ElasticCompletionFieldMappingVisitor<'a> {
	data: &'a ElasticCompletionFieldMapping
}
impl <'a> ElasticCompletionFieldMappingVisitor<'a> {
	#[doc(hidden)]
	pub fn new(field: &'a ElasticCompletionFieldMapping) -> Self {
		ElasticCompletionFieldMappingVisitor {
			data: field
		}
	}
}

impl <'a> serde::ser::MapVisitor for ElasticCompletionFieldMappingVisitor<'a> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", COMPLETION_DATATYPE));

		ser_sub_field!(serializer, self.data.analyzer, "analyzer");
		ser_sub_field!(serializer, self.data.search_analyzer, "search_analyzer");
		ser_sub_field!(serializer, self.data.payloads, "payloads");
		ser_sub_field!(serializer, self.data.preserve_separators, "preserve_separators");
		ser_sub_field!(serializer, self.data.preserve_position_increments, "preserve_position_increments");
		ser_sub_field!(serializer, self.data.max_input_length, "max_input_length");

		Ok(None)
	}
}

/// A multi-field string mapping.
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticKeywordFieldMapping {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	analyzer: Option<&'static str>,
	/// Should the field be stored on disk in a column-stride fashion,
	/// so that it can later be used for sorting, aggregations, or scripting?
	/// Accepts `true` (default) or `false`.
	doc_values: Option<bool>,
	/// Should global ordinals be loaded eagerly on refresh? 
	/// Accepts `true` or `false` (default).
	/// Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations. 
	eager_global_ordinals: Option<bool>,
	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	include_in_all: Option<bool>,
	/// The maximum number of characters to index.
	/// Any characters over this length will be ignored.
	ignore_above: Option<u32>,
	/// Should the field be searchable? Accepts `true` (default) or `false`.
	index: Option<bool>,
	/// What information should be stored in the index, for search and highlighting purposes. Defaults to `Positions`.
	index_options: Option<IndexOptions>,
	/// Whether field-length should be taken into account when scoring queries. Accepts `true` (default) or `false`.
	norms: Option<bool>,
	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	store: Option<bool>,
	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	search_analyzer: Option<&'static str>,
	/// Which scoring algorithm or similarity should be used. 
	/// Defaults to `"classic"`, which uses TF/IDF. 
	similarity: Option<&'static str>
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

/// A multi-field string mapping.
#[derive(Debug, Default, Clone, Copy)]
pub struct ElasticTextFieldMapping {
	/// The analyzer which should be used for analyzed string fields,
	/// both at index-time and at search-time (unless overridden by the `search_analyzer`).
	/// Defaults to the default index analyzer, or the `standard` analyzer.
	analyzer: Option<&'static str>,
	/// Should global ordinals be loaded eagerly on refresh? 
	/// Accepts `true` or `false` (default).
	/// Enabling this is a good idea on fields that are frequently used for (significant) terms aggregations. 
	eager_global_ordinals: Option<bool>,
	/// Can the field use in-memory fielddata for sorting, aggregations, or scripting? 
	/// Accepts `true` or `false` (default).
	fielddata: Option<bool>,
	/// Expert settings which allow to decide which values to load in memory when `fielddata` is enabled. 
	/// By default all values are loaded.
	fielddata_frequency_filter: Option<FieldDataFrequencyFilter>,
	/// Whether or not the field value should be included in the `_all` field?
	/// Accepts true or false.
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
	/// Otherwise defaults to `true`.
	include_in_all: Option<bool>,
	/// The maximum number of characters to index.
	/// Any characters over this length will be ignored.
	ignore_above: Option<u32>,
	/// Should the field be searchable? Accepts `true` (default) or `false`.
	index: Option<bool>,
	/// What information should be stored in the index, for search and highlighting purposes. Defaults to `Positions`.
	index_options: Option<IndexOptions>,
	/// Whether field-length should be taken into account when scoring queries. Accepts `true` (default) or `false`.
	norms: Option<bool>,
	/// The number of fake term position which should be inserted between each element of an array of strings. 
	/// Defaults to the `position_increment_gap` configured on the analyzer which defaults to `100`. 
	/// `100` was chosen because it prevents phrase queries with reasonably large slops (less than `100`) 
	/// from matching terms across field values.
	position_increment_gap: Option<u32>,
	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	store: Option<bool>,
	/// The analyzer that should be used at search time on analyzed fields. 
	/// Defaults to the analyzer setting.
	search_analyzer: Option<&'static str>,
	/// The analyzer that should be used at search time when a phrase is encountered. 
	/// Defaults to the `search_analyzer` setting.
	search_quote_analyzer: Option<&'static str>,
	/// Which scoring algorithm or similarity should be used. 
	/// Defaults to `"classic"`, which uses TF/IDF. 
	similarity: Option<&'static str>,
	/// Whether term vectors should be stored for an `analyzed` field. 
	/// Defaults to `No`.
	term_vector: Option<TermVector>
}

impl serde::Serialize for ElasticTextFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
	where S: Serializer
	{
		serializer.serialize_struct("fields", ElasticTextFieldMappingVisitor::new(&self))
	}
}

#[doc(hidden)]
#[derive(Debug)]
pub struct ElasticTextFieldMappingVisitor<'a> {
	data: &'a ElasticTextFieldMapping
}
impl <'a> ElasticTextFieldMappingVisitor<'a> {
	#[doc(hidden)]
	pub fn new(field: &'a ElasticTextFieldMapping) -> Self {
		ElasticTextFieldMappingVisitor {
			data: field
		}
	}
}

impl <'a> serde::ser::MapVisitor for ElasticTextFieldMappingVisitor<'a> {
	#[cfg_attr(feature = "nightly-testing", allow(cyclomatic_complexity))]
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: serde::Serializer {
		try!(serializer.serialize_struct_elt("type", TEXT_DATATYPE));

		ser_sub_field!(serializer, self.data.analyzer, "analyzer");
		ser_sub_field!(serializer, self.data.eager_global_ordinals, "eager_global_ordinals");
		ser_sub_field!(serializer, self.data.fielddata, "fielddata");
		ser_sub_field!(serializer, self.data.fielddata_frequency_filter, "fielddata_frequency_filter");
		ser_sub_field!(serializer, self.data.include_in_all, "include_in_all");
		ser_sub_field!(serializer, self.data.ignore_above, "ignore_above");
		ser_sub_field!(serializer, self.data.index, "index");
		ser_sub_field!(serializer, self.data.index_options, "index_options");
		ser_sub_field!(serializer, self.data.norms, "norms");
		ser_sub_field!(serializer, self.data.position_increment_gap, "position_increment_gap");
		ser_sub_field!(serializer, self.data.store, "store");
		ser_sub_field!(serializer, self.data.search_analyzer, "search_analyzer");
		ser_sub_field!(serializer, self.data.search_quote_analyzer, "search_quote_analyzer");
		ser_sub_field!(serializer, self.data.similarity, "similarity");
		ser_sub_field!(serializer, self.data.term_vector, "term_vector");

		Ok(None)
	}
}