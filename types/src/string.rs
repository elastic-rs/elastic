//! Implementation of the Elasticsearch `string` type.
//! 
//! Strings are stored as a sequence of tokens, constructed based on the given `analyzer`.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

/// A Rust representation of an Elasticsearch `string`.
pub trait ElasticStringLike {
	fn get_analyzer(&self) -> Option<&'static str>;
	fn get_boost(&self) -> Option<&'static f32>;
	fn get_doc_values(&self) -> Option<&'static bool>;
	fn get_fielddata(&self) -> Option<&'static FieldData>;
}

#[derive(Default)]
/// A representation of an Elasticsearch `string`.
/// 
/// String types have a number of static values describing how they're analysed at
pub struct ElasticString {
	data: String,
	pub analyzer: Option<&'static str>,
	pub boost: Option<&'static f32>,
	pub doc_values: Option<&'static bool>,
	pub fielddata: Option<&'static FieldData>
}

impl From<String> for ElasticString {
	fn from(string: String) -> ElasticString {
		ElasticString {
			data: string,
			..Default::default()
		}
	}
}

pub enum FieldData {
	PagedBytes,
	Disabled
}

impl FieldData {
	fn parse(fd: &str) -> FieldData {
		match fd {
			"disabled" => FieldData::Disabled,
			_ => FieldData::PagedBytes
		}
	}
}

impl Default for FieldData {
	fn default() -> FieldData {
		FieldData::PagedBytes
	}
}

impl ToString for FieldData {
	fn to_string(&self) -> String {
		match *self {
			FieldData::Disabled => "disabled".to_string(),
			FieldData::PagedBytes => "paged_bytes".to_string()
		}
	}
}