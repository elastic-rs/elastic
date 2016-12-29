use serde_json;

use elastic_types::mapping::prelude::*;
use ::string_fixtures::*;

#[test]
fn string_has_default_mapping() {
    assert_eq!(DefaultStringMapping, String::mapping());
}

#[test]
fn serialise_string_mapping_default() {
    let ser = FieldMapper::to_string(DefaultStringMapping).unwrap();

    let expected = json_str!({
        "type":"text",
        "fields":{
            "keyword":{
                "type":"keyword",
                "ignore_above":256
            }
        }
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_text_mapping_default() {
    let ser = FieldMapper::to_string(DefaultTextMapping).unwrap();

    let expected = json_str!({
        "type": "text"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_text_mapping_custom() {
    let ser = FieldMapper::to_string(MyTextMapping).unwrap();

    let expected = json_str!({
        "type":"text",
        "boost":1.3,
        "analyzer":"my_analyzer",
        "eager_global_ordinals":false,
        "fielddata":true,
        "fielddata_frequency_filter":{
            "min":0.0
        },
        "fields":{
            "comp":{
                "type":"completion"
            },
            "count":{
                "type":"token_count"
            },
            "raw":{
                "type":"keyword",
                "analyzer":"my_analyzer"
            }
        },
        "include_in_all":true,
        "ignore_above":512,
        "index":false,
        "index_options":"freqs",
        "norms":true,
        "position_increment_gap":1,
        "store":true,
        "search_analyzer":"my_analyzer",
        "search_quote_analyzer":"my_analyzer",
        "similarity":"BM25",
        "term_vector":"yes"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_keyword_mapping_default() {
    let ser = FieldMapper::to_string(DefaultKeywordMapping).unwrap();

    let expected = json_str!({
        "type": "keyword"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_keyword_mapping_custom() {
    let ser = FieldMapper::to_string(MyKeywordMapping).unwrap();

    let expected = json_str!({
        "type": "keyword",
        "boost": 1.03,
        "analyzer": "my_analyzer",
        "doc_values": true,
        "eager_global_ordinals": false,
        "fields": {
            "comp": {
                "type": "completion"
            },
            "count": {
                "type": "token_count"
            },
            "text": {
                "type": "text",
                "analyzer": "my_analyzer"
            }
        },
        "include_in_all": false,
        "ignore_above": 256,
        "index": true,
        "index_options": "docs",
        "norms": false,
        "null_value": "my string",
        "store": false,
        "search_analyzer": "my_analyzer",
        "similarity": "classic"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_field_filter() {
    let filter = FieldDataFrequencyFilter {
        min: Some(0.001),
        max: Some(0.1),
        min_segment_size: Some(500)
    };

    let ser = serde_json::to_string(&filter).unwrap();

    let expected = json_str!({
        "min": 0.001,
        "max": 0.1,
        "min_segment_size": 500
    });

    assert_eq!(expected, ser);
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
fn serialise_mapping_keyword_field() {
    let mapping = ElasticStringField::Keyword(
        KeywordFieldMapping {
            analyzer:               Some("my_analyzer"),
            doc_values:             Some(true),
            eager_global_ordinals:  Some(false),
            include_in_all:         Some(true),
            ignore_above:           Some(256),
            index:                  Some(false),
            index_options:          Some(IndexOptions::Docs),
            norms:                  Some(true),
            store:                  Some(true),
            search_analyzer:        Some("my_analyzer"),
            similarity:             Some("my_analyzer")
        }
    );
    let ser = serde_json::to_string(&mapping).unwrap();

    let expected = json_str!({
        "type":"keyword",
        "analyzer":"my_analyzer",
        "doc_values":true,
        "eager_global_ordinals":false,
        "include_in_all":true,
        "ignore_above":256,
        "index":false,
        "index_options":"docs",
        "norms":true,
        "store":true,
        "search_analyzer":"my_analyzer",
        "similarity":"my_analyzer"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_text_field() {
    let mapping = ElasticStringField::Text(
        TextFieldMapping {
            fielddata_frequency_filter: Some(
                FieldDataFrequencyFilter { 
                    min: Some(0.0), ..Default::default() 
                }
            ),
            analyzer:               Some("my_analyzer"),
            eager_global_ordinals:  Some(true),
            fielddata:              Some(false),
            include_in_all:         Some(false),
            ignore_above:           Some(512),
            index:                  Some(true),
            index_options:          Some(IndexOptions::Freqs),
            norms:                  Some(true),
            position_increment_gap: Some(1),
            store:                  Some(false),
            search_analyzer:        Some("my_analyzer"),
            search_quote_analyzer:  Some("my_analyzer"),
            similarity:             Some("BM25"),
            term_vector:            Some(TermVector::No)
        }
    );
    let ser = serde_json::to_string(&mapping).unwrap();

    let expected = json_str!({
        "type":"text",
        "analyzer":"my_analyzer",
        "eager_global_ordinals":true,
        "fielddata":false,
        "fielddata_frequency_filter":{
            "min":0.0
        },
        "include_in_all":false,
        "ignore_above":512,
        "index":true,
        "index_options":"freqs",
        "norms":true,
        "position_increment_gap":1,
        "store":false,
        "search_analyzer":"my_analyzer",
        "search_quote_analyzer":"my_analyzer",
        "similarity":"BM25",
        "term_vector":"no"
    });

    assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_token_count_field() {
    let mapping = ElasticStringField::TokenCount(
        ElasticTokenCountFieldMapping {
            analyzer:           Some("my_analyzer"),
            boost:              Some(1.3),
            doc_values:         Some(false),
            index:              Some(IndexAnalysis::No),
            include_in_all:     Some(true),
            precision_step:     Some(15),
            store:              Some(true)
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
            analyzer:                       Some("my_analyzer"),
            search_analyzer:                Some("my_analyzer"),
            payloads:                       Some(true),
            preserve_separators:            Some(false),
            preserve_position_increments:   Some(true),
            max_input_length:               Some(512)
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
