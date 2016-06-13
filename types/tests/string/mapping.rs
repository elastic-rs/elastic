#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

extern crate serde;
extern crate serde_json;
extern crate elastic_types;

use elastic_types::mapping::prelude::*;
use ::string_fixtures::*;

#[test]
fn serialise_mapping_default() {
	let mapping = DefaultStringMapping::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "string"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyStringMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "string",
		"boost": 1.01,
		"doc_values": true,
		"include_in_all": false,
		"index": "no",
		"store": true,
		"analyzer": "my_analyzer",
		"fields": {
			"comp": {
				"type": "completion"
			},
			"count": {
				"type": "token_count"
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
		json_str!({ "format": "disabled" }),
		json_str!({ "loading": "lazy" }),
		json_str!({
			"filter": {
				"regex": {
					"pattern": ".*"
				}
			}
		}),
		json_str!({
			"loading": "lazy",
			"filter": {
				"regex": {
					"pattern": ".*"
				}
			}
		}),
		String::new()
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
		json_str!({
			"frequency": {
				"min": 0.001,
				"max": 0.1,
				"min_segment_size": 500
			}
		}),
		json_str!({
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
		json_str!({ "loading": "eager" }),
		json_str!({ "enabled": false })
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

#[test]
fn serialise_mapping_string_field() {
	let mapping = ElasticStringField::String(
		ElasticStringFieldMapping {
			analyzer: Some("my_analyzer"),
			fielddata: Some(FieldData::Disabled),
			ignore_above: Some(1),
			index_options: Some(IndexOptions::Docs),
			norms: Some(Norms::Disabled),
			position_increment_gap: Some(1),
			search_analyzer: Some("my_analyzer"),
			search_quote_analyzer: Some("my_analyzer"),
			similarity: Some("my_similarity"),
			term_vector: Some(TermVector::No)
		}
	);
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"analyzer": "my_analyzer",
		"fielddata": {
			"format": "disabled"
		},
		"ignore_above": 1,
		"index_options": "docs",
		"norms": {
			"enabled": false
		},
		"position_increment_gap": 1,
		"search_analyzer": "my_analyzer",
		"search_quote_analyzer": "my_analyzer",
		"similarity": "my_similarity",
		"term_vector": "no"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_token_count_field() {
	let mapping = ElasticStringField::TokenCount(
		ElasticTokenCountFieldMapping {
			analyzer: Some("my_analyzer"),
			boost: Some(1.3),
			doc_values: Some(false),
			index: Some(IndexAnalysis::No),
			include_in_all: Some(true),
			precision_step: Some(15),
			store: Some(true)
		}
	);
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "token_count",
		"analyzer": "my_analyzer",
		"boost": 1.3,
		"doc_values": false,
		"index": "no",
		"include_in_all": true,
		"precision_step": 15,
		"store": true
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_completion_field() {
	let mapping = ElasticStringField::Completion(
		ElasticCompletionFieldMapping {
			analyzer: Some("my_analyzer"),
			search_analyzer: Some("my_analyzer"),
			payloads: Some(true),
			preserve_separators: Some(false),
			preserve_position_increments: Some(true),
			max_input_length: Some(512)
		}
	);
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "completion",
		"analyzer": "my_analyzer",
		"search_analyzer": "my_analyzer",
		"payloads": true,
		"preserve_separators": false,
		"preserve_position_increments": true,
		"max_input_length": 512
	});

	assert_eq!(expected, ser);
}
