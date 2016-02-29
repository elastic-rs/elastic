//! Elasticsearch Core Types
//!
//! A high-level implementation of the core types in Elasticsearch documents.
//! 
//! Types within this crate are self-contained and handle their own serialisation/deserialisation requirements.
//! Each type also supplies a `struct` for its [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html) properties.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types/")]
#![deny(missing_docs)]

#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate chrono;
extern crate serde;

pub mod date;
//pub mod string;

/// The base requirements for mapping an Elasticsearch type.
/// 
/// Each type will have its own implementing structures with extra type-specific mapping parameters.
pub trait ElasticMapping {
	/// Field-level index time boosting. Accepts a floating point number, defaults to `1.0`.
	fn get_boost() -> f32 {
		1.0
	}

	/// Should the field be stored on disk in a column-stride fashion, 
	/// so that it can later be used for sorting, aggregations, or scripting? 
	/// Accepts `true` (default) or `false`.
	fn get_doc_values() -> bool {
		true
	}

	/// Whether or not the field value should be included in the `_all` field? 
	/// Accepts true or false. 
	/// Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false. 
	/// Otherwise defaults to `true`.
	fn get_include_in_all() -> bool {
		match Self::get_index() {
			IndexAnalysis::No => false,
			_ => true
		}
	}

	/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
	fn get_index() -> IndexAnalysis {
		IndexAnalysis::NotAnalyzed
	}

	/// Whether the field value should be stored and retrievable separately from the `_source` field. 
	/// Accepts `true` or `false` (default).
	fn get_store() -> bool {
		false
	}
}

/// A type that can be indexed in Elasticsearch.
pub trait ElasticType {
	/// The mapping definition for this type.
	/// 
	/// The shape of the mapping is specific to each type, 
	/// but the values are specific to each implementation.
	type Mapping: ElasticMapping;
}

/// Should the field be searchable? Accepts `not_analyzed` (default) and `no`.
#[dervice(Debug, Copy)]
pub enum IndexAnalysis {
	/// This option applies only to string fields, for which it is the default. 
	/// The string field value is first analyzed to convert the string into terms 
	/// (e.g. a list of individual words), which are then indexed. 
	/// At search time, the query string is passed through (usually) the same analyzer 
	/// to generate terms in the same format as those in the index. 
	/// It is this process that enables full text search.
	Analyzed,
	/// Add the field value to the index unchanged, as a single term. 
	/// This is the default for all fields that support this option except for string fields. 
	/// `not_analyzed` fields are usually used with term-level queries for structured search.
	NotAnalyzed,
	/// Do not add this field value to the index. With this setting, the field will not be queryable.
	No
}

impl Default for IndexAnalysis {
	fn default() -> IndexAnalysis {
		IndexAnalysis::NotAnalyzed
	}
}