
extern crate elastic_responses;

#[macro_use]
extern crate json_str;

extern crate serde_json;

use elastic_responses::*;
use elastic_responses::error::*;
use std::fs::File;
use std::io::{Read, Cursor};

fn load_file_as_response(status: u16, p: &str) -> HttpResponse<Cursor<Vec<u8>>> {
    let mut f = File::open(p).unwrap();
    let mut s = Vec::new();
    f.read_to_end(&mut s).unwrap();
    
    HttpResponse::new(status, Cursor::new(s))
}

#[test]
fn test_parse_hits_simple() {
    let s = load_file_as_response(200, "tests/samples/hits_only.json");

    let deserialized = SearchResponse::from_response(s).unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 5);
}

#[test]
fn test_parse_simple_aggs() {
    let s = load_file_as_response(200, "tests/samples/aggregation_simple.json");
    let deserialized = SearchResponse::from_response(s).unwrap();

    assert_eq!(deserialized.aggs().into_iter().count(), 124);
}

#[test]
fn test_parse_3level_aggs() {
    let s = load_file_as_response(200, "tests/samples/aggregation_3level.json");
    let deserialized = SearchResponse::from_response(s).unwrap();

    assert_eq!(deserialized.aggs().into_iter().count(), 201);
}

#[test]
fn test_parse_3level_multichild_aggs() {
    let s = load_file_as_response(200, "tests/samples/aggregation_3level_multichild.json");
    let deserialized = SearchResponse::from_response(s).unwrap();

    let min = "min_ack_pkts_sent";
    let avg = "avg_ack_pkts_sent";
    let max = "max_ack_pkts_sent";
    let mut first = true;
    let mut count = 0;
    for i in deserialized.aggs().into_iter().take(500000) {
        count += 1;
        if first {
            assert!(i.contains_key(min));
            assert!(i.contains_key(max));
            assert!(i.contains_key(avg));
            first = false;
        }
    }
    assert_eq!(count, 201);
}

#[test]
fn test_parse_3level_multistats_aggs() {
    let s = load_file_as_response(200, "tests/samples/aggregation_3level_multistats.json");
    let deserialized = SearchResponse::from_response(s).unwrap();

    let min = "extstats_ack_pkts_sent_min";
    let avg = "stats_ack_pkts_sent_avg";
    let max = "extstats_ack_pkts_sent_max";
    let stddevu = "extstats_ack_pkts_sent_std_deviation_bounds_upper";
    let mut first = true;
    let mut count = 0;
    for i in deserialized.aggs().into_iter().take(500000) {
        count += 1;
        if first {
            assert!(i.contains_key(min));
            assert!(i.contains_key(max));
            assert!(i.contains_key(avg));
            assert!(i.contains_key(stddevu));
            first = false;
        }
    }
    assert_eq!(count, 61);
}

#[test]
fn test_parse_simple_aggs_no_empty_first_record() {
    let s = load_file_as_response(200, "tests/samples/aggregation_simple.json");
    let deserialized = SearchResponse::from_response(s).unwrap();

    let s = "timechart";
    let mut first = true;
    for i in deserialized.aggs().into_iter().take(50) {
        if first {
            assert!(i.contains_key(s));
            first = false;
        }
    }
}

#[test]
fn test_parse_found_doc_response() {
    let s = load_file_as_response(200, "tests/samples/get_found.json");
    let deserialized = GetResponse::from_response(s).unwrap();

    let id = deserialized.source
        .unwrap()
        .as_object()
        .and_then(|src| src.get("id"))
        .and_then(|id| id.as_u64());

    assert!(deserialized.found);
    assert_eq!("testindex", deserialized.index);
    assert_eq!("testtype", deserialized.ty);
    assert_eq!(Some(8), deserialized.version);
    assert_eq!(Some(1), id);
}

#[test]
fn test_parse_not_found_doc_response() {
    let s = load_file_as_response(404, "tests/samples/get_not_found.json");
    let deserialized = GetResponse::from_response(s).unwrap();

    assert!(!deserialized.found);
}

#[test]
fn test_parse_index_not_found_error() {
    let s = load_file_as_response(404, "tests/samples/error_index_not_found.json");
    let deserialized = GetResponse::from_response(s).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::IndexNotFound { ref index })
        if index == "carrots" => true,
        _ => false
    };

    assert!(valid);
}

#[test]
fn test_parse_parsing_error() {
    let s = load_file_as_response(400, "tests/samples/error_parsing.json");
    let deserialized = SearchResponse::from_response(s).unwrap_err();

    let valid = match deserialized {
        ResponseError::Api(ApiError::Parsing { line: 2, col: 9, ref reason })
        if reason == "Unknown key for a START_OBJECT in [qry]." => true,
        _ => false
    };

    assert!(valid);
}

#[test]
fn test_parse_other_error() {
    let s = load_file_as_response(500, "tests/samples/error_other.json");
    let deserialized = SearchResponse::from_response(s).unwrap_err();

    let reason = match deserialized {
        ResponseError::Api(ApiError::Other(ref err)) => err.get("reason")
                                       .and_then(|reason| reason.as_str())
                                       .map(|reason| reason.to_owned()),
        _ => None
    };

    assert_eq!(Some(String::from("An error that we don't know about.")), reason);
}
