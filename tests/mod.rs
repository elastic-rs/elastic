
extern crate elastic_responses;

#[macro_use]
extern crate log;

#[macro_use]
extern crate json_str;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate slog_stdlog;
extern crate slog_envlogger;

use elastic_responses::*;
use std::fs::File;
use std::io::Read;

fn load_file(p :&str) -> String {
    let mut f = File::open(p).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}

#[test]
fn test_parse_hits_simple() {
    let s = load_file("tests/samples/hits_only.json");
    let deserialized: QueryResponse = serde_json::from_str(&s).unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 5);
}

#[test]
fn test_parse_simple_aggs() {
    let s = load_file("tests/samples/aggregation_simple.json");
    let deserialized: QueryResponse = serde_json::from_str(&s).unwrap();

    assert_eq!(deserialized.aggs().into_iter().count(), 124);
}

#[test]
fn test_parse_3level_aggs() {
    let s = load_file("tests/samples/aggregation_3level.json");
    let deserialized: QueryResponse = serde_json::from_str(&s).unwrap();

    assert_eq!(deserialized.aggs().into_iter().count(), 201);
}

#[test]
fn test_parse_3level_multichild_aggs() {
    let s = load_file("tests/samples/aggregation_3level_multichild.json");
    let deserialized: QueryResponse = serde_json::from_str(&s).unwrap();

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
    let s = load_file("tests/samples/aggregation_3level_multistats.json");
    let deserialized: QueryResponse = serde_json::from_str(&s).unwrap();

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
    let s = load_file("tests/samples/aggregation_simple.json");
    let deserialized: QueryResponse = serde_json::from_str(&s).unwrap();

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
    let s = load_file("tests/samples/get_found.json");
    let deserialized: GetDocResponse = serde_json::from_str(&s).unwrap();

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
    let s = load_file("tests/samples/get_not_found.json");
    let deserialized: GetDocResponse = serde_json::from_str(&s).unwrap();

    assert!(!deserialized.found);
}

#[test]
fn test_parse_index_not_found_error() {
    let s = load_file("tests/samples/error_index_not_found.json");
    let deserialized: ApiError = serde_json::from_str(&s).unwrap();

    let error = deserialized.error();

    assert_eq!(404, deserialized.status);
    assert_eq!(ApiErrorKind::IndexNotFound { index: "carrots" }, error);
}

#[test]
fn test_parse_parsing_error() {
    let s = load_file("tests/samples/error_parsing.json");
    let deserialized: ApiError = serde_json::from_str(&s).unwrap();

    let error = deserialized.error();

    assert_eq!(400, deserialized.status);
    assert_eq!(ApiErrorKind::Parsing { line: 2, col: 9, reason: "Unknown key for a START_OBJECT in [qry]." }, error);
}

#[test]
fn test_parse_other_error() {
    let s = load_file("tests/samples/error_other.json");
    let deserialized: ApiError = serde_json::from_str(&s).unwrap();

    let error = deserialized.error();

    let reason = match error {
        ApiErrorKind::Other(err) => err.as_object()
                                    .and_then(|err| err.get("reason"))
                                    .and_then(|reason| reason.as_str()),
        _ => None
    };

    assert_eq!(500, deserialized.status);
    assert_eq!(Some("An error that we don't know about."), reason);
}