#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use std::collections::BTreeMap;
use elastic_types::mapping::*;
use elastic_types::string::mapping::*;
use elastic_types::string::*;

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
 			similarity: Some("BM25"),
 			..Default::default()
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

impl serde::Serialize for MyStringMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
		where S: serde::Serializer
	{
		serializer.serialize_struct("mapping", MyStringMapping::get_visitor())
	}
}

#[test]
fn serialise_mapping_default() {
	let mapping = DefaultStringMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"type":"string"}"#, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyStringMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"type":"string","boost":1.01,"doc_values":true,"include_in_all":false,"index":"no","store":true,"analyzer":"my_analyzer","fields":{"bm25_field":{"analyzer":null,"fielddata":null,"ignore_above":null,"index_options":null,"norms":null,"null_value":null,"position_increment_gap":null,"search_analyzer":null,"search_quote_analyzer":null,"similarity":"BM25","term_vector":null},"raw":{"analyzer":"my_analyzer","fielddata":null,"ignore_above":null,"index_options":null,"norms":null,"null_value":null,"position_increment_gap":null,"search_analyzer":null,"search_quote_analyzer":null,"similarity":null,"term_vector":null}},"fielddata":{"format":"disabled"},"ignore_above":50,"index_options":"docs","norms":{"enabled":false},"null_value":"my default value","position_increment_gap":8,"search_analyzer":"my_search_analyzer","search_quote_analyzer":"my_quote_search_analyzer","similarity":"my_similarity","term_vector":"no"}"#, ser);
}

#[test]
fn serialise_mapping_field_data() {
	let fd_opts: Vec<String> = vec![
		FieldData::Disabled,
		FieldData::PagedBytes(None, None),
		FieldData::PagedBytes(Some(FieldDataLoading::Lazy), None),
		FieldData::PagedBytes(None, Some(FieldDataFilter::Regex(RegexFilter { pattern: ".*" }))),
		FieldData::PagedBytes(Some(FieldDataLoading::Lazy), Some(FieldDataFilter::Regex(RegexFilter { pattern: ".*" })))
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#"{"format":"disabled"}"#,
		r#""#,
		r#"{"loading":"lazy"}"#,
		r#"{"filter":{"regex":{"pattern":".*"}}}"#,
		r#"{"loading":"lazy","filter":{"regex":{"pattern":".*"}}}"#
	];

	let mut success = true;
	for i in 0..fd_opts.len() {
		if expected_opts[i] != &fd_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn serialise_mapping_field_data_loading() {
	let fd_opts: Vec<String> = vec![
		FieldDataLoading::Lazy,
		FieldDataLoading::Eager,
		FieldDataLoading::EagerGlobalOrdinals
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""lazy""#,
		r#""eager""#,
		r#""eager_global_ordinals""#
	];

	let mut success = true;
	for i in 0..fd_opts.len() {
		if expected_opts[i] != &fd_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn serialise_mapping_field_filter() {
	let fd_opts: Vec<String> = vec![
		FieldDataFilter::Frequency(FrequencyFilter {
			min: 0.001,
			max: 0.1,
			min_segment_size: 500
		}),
		FieldDataFilter::Regex(RegexFilter {
			pattern: "^#.*"
		})
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#"{"frequency":{"min":0.001,"max":0.1,"min_segment_size":500}}"#,
		r#"{"regex":{"pattern":"^#.*"}}"#
	];

	let mut success = true;
	for i in 0..fd_opts.len() {
		if expected_opts[i] != &fd_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn serialise_mapping_index_options() {
	let io_opts: Vec<String> = vec![
		IndexOptions::Docs,
		IndexOptions::Freqs,
		IndexOptions::Positions,
		IndexOptions::Offsets
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""docs""#,
		r#""freqs""#,
		r#""positions""#,
		r#""offsets""#
	];

	let mut success = true;
	for i in 0..io_opts.len() {
		if expected_opts[i] != &io_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn serialise_mapping_norms() {
	let n_opts: Vec<String> = vec![
		Norms::Enabled {
			loading: NormsLoading::Eager
		},
		Norms::Disabled
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#"{"loading":"eager"}"#,
		r#"{"enabled":false}"#
	];

	let mut success = true;
	for i in 0..n_opts.len() {
		if expected_opts[i] != &n_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn serialise_mapping_norms_loading() {
	let n_opts: Vec<String> = vec![
		NormsLoading::Eager,
		NormsLoading::Lazy
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""eager""#,
		r#""lazy""#
	];

	let mut success = true;
	for i in 0..n_opts.len() {
		if expected_opts[i] != &n_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn serialise_mapping_terms_vector() {
	let v_opts: Vec<String> = vec![
		TermVector::No,
		TermVector::Yes,
		TermVector::WithPositions,
		TermVector::WithOffsets,
		TermVector::WithPositionsOffsets
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		r#""no""#,
		r#""yes""#,
		r#""with_positions""#,
		r#""with_offsets""#,
		r#""with_positions_offsets""#
	];

	let mut success = true;
	for i in 0..v_opts.len() {
		if expected_opts[i] != &v_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}