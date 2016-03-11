#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use std::collections::BTreeMap;
use elastic_types::mapping::prelude::*;
use ::string_fixtures::*;

#[test]
fn serialise_mapping_default() {
	let mapping: DefaultStringMapping = DefaultStringMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json!({
		"type": "string"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyStringMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json!({
		"type": "string",
		"boost": 1.01,
		"doc_values": true,
		"include_in_all": false,
		"index": "no",
		"store": true,
		"analyzer": "my_analyzer",
		"fields": {
			"bm25_field": {
				"analyzer": "my_analyzer",
				"fielddata": {
					"format": "disabled"
				},
				"ignore_above": 50,
				"index_options": "docs",
				"norms": {
					"enabled": false
				},
				"null_value": "my default value",
				"position_increment_gap": 8,
				"search_analyzer": "my_search_analyzer",
				"search_quote_analyzer": "my_quote_search_analyzer",
				"similarity": "BM25",
				"term_vector": "no"
			},
			"raw": {
				"analyzer": "my_analyzer"
			}
		},
		"fielddata": {
			"format": "disabled"
		},
		"ignore_above": 50,
		"index_options": "docs",
		"norms": {
			"enabled": false
		},
		"null_value": "my default value",
		"position_increment_gap": 8,
		"search_analyzer": "my_search_analyzer",
		"search_quote_analyzer": "my_quote_search_analyzer",
		"similarity": "my_similarity",
		"term_vector": "no"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_field_data() {
	let fd_opts: Vec<String> = vec![
		FieldData::Disabled,
		FieldData::PagedBytes(Some(FieldDataLoading::Lazy), None),
		FieldData::PagedBytes(None, Some(FieldDataFilter::Regex(RegexFilter { pattern: ".*" }))),
		FieldData::PagedBytes(Some(FieldDataLoading::Lazy), Some(FieldDataFilter::Regex(RegexFilter { pattern: ".*" }))),
		FieldData::PagedBytes(None, None),
	]
	.iter()
	.map(|i| serde_json::to_string(i).unwrap())
	.collect();

	let expected_opts = vec![
		json!({ "format": "disabled" }),
		json!({ "loading": "lazy" }),
		json!({
			"filter": {
				"regex": {
					"pattern": ".*"
				}
			}
		}),
		json!({
			"loading": "lazy",
			"filter": {
				"regex": {
					"pattern": ".*"
				}
			}
		}),
		String::from("")
	];

	let mut success = true;
	for i in 0..fd_opts.len() {
		if expected_opts[i] != fd_opts[i] {
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
		if expected_opts[i] != fd_opts[i] {
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
		json!({
			"frequency": {
				"min": 0.001,
				"max": 0.1,
				"min_segment_size": 500
			}
		}),
		json!({
			"regex": {
				"pattern": "^#.*"
			}
		})
	];

	let mut success = true;
	for i in 0..fd_opts.len() {
		if expected_opts[i] != fd_opts[i] {
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
		if expected_opts[i] != io_opts[i] {
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
		json!({ "loading": "eager" }),
		json!({ "enabled": false })
	];

	let mut success = true;
	for i in 0..n_opts.len() {
		if expected_opts[i] != n_opts[i] {
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
		if expected_opts[i] != n_opts[i] {
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
		if expected_opts[i] != v_opts[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}