//! Common mapping for the Elasticsearch `string` types.

use std::collections::BTreeMap;
use serde::{ Serialize, Serializer };
use ::mapping::{ IndexAnalysis, ElasticType };

pub use super::keyword::mapping::*;
pub use super::text::mapping::*;

/// Elasticsearch datatype name.
pub const TOKENCOUNT_DATATYPE: &'static str = "token_count";
/// Elasticsearch datatype name.
pub const COMPLETION_DATATYPE: &'static str = "completion";

/// Default mapping for `String`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultStringMapping;
impl TextMapping for DefaultStringMapping {
	fn fields() -> Option<BTreeMap<&'static str, ElasticStringField>> {
		let mut fields = BTreeMap::new();

		fields.insert("keyword", ElasticStringField::Keyword(
			KeywordFieldMapping {
				ignore_above: Some(256),
				..Default::default()
			})
		);

		Some(fields)
	}
}

impl ElasticType<DefaultStringMapping, TextFormat> for String { }

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

impl Serialize for IndexOptions {
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
	Keyword(KeywordFieldMapping),
	/// A `text` sub field.
	Text(TextFieldMapping)
}

impl Serialize for ElasticStringField {
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

impl Serialize for ElasticTokenCountFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 8));

		try!(serializer.serialize_struct_elt(&mut state, "type", TOKENCOUNT_DATATYPE));

		ser_field!(serializer, &mut state, self.analyzer, "analyzer");
		ser_field!(serializer, &mut state, self.boost, "boost");
		ser_field!(serializer, &mut state, self.doc_values, "doc_values");
		ser_field!(serializer, &mut state, self.index, "index");
		ser_field!(serializer, &mut state, self.include_in_all, "include_in_all");
		ser_field!(serializer, &mut state, self.precision_step, "precision_step");
		ser_field!(serializer, &mut state, self.store, "store");

		serializer.serialize_struct_end(state)
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

impl Serialize for ElasticCompletionFieldMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 7));

		try!(serializer.serialize_struct_elt(&mut state, "type", COMPLETION_DATATYPE));

		ser_field!(serializer, &mut state, self.analyzer, "analyzer");
		ser_field!(serializer, &mut state, self.search_analyzer, "search_analyzer");
		ser_field!(serializer, &mut state, self.payloads, "payloads");
		ser_field!(serializer, &mut state, self.preserve_separators, "preserve_separators");
		ser_field!(serializer, &mut state, self.preserve_position_increments, "preserve_position_increments");
		ser_field!(serializer, &mut state, self.max_input_length, "max_input_length");

		serializer.serialize_struct_end(state)
	}
}