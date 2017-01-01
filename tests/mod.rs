#![feature(proc_macro)]

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

use elastic_responses::Response;
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
    let deserialized: Response = serde_json::from_str(&s).unwrap();

    assert_eq!(deserialized.hits().into_iter().count(), 5);
}

#[test]
fn test_parse_simple_aggs() {
    let s = load_file("tests/samples/aggregation_simple.json");
    let deserialized: Response = serde_json::from_str(&s).unwrap();

    assert_eq!(deserialized.aggs().unwrap().into_iter().count(), 124);
}

#[test]
fn test_parse_3level_aggs() {
    let s = load_file("tests/samples/aggregation_3level.json");
    let deserialized: Response = serde_json::from_str(&s).unwrap();

    assert_eq!(deserialized.aggs().unwrap().into_iter().count(), 201);
}

#[test]
fn test_parse_3level_multichild_aggs() {
    let s = load_file("tests/samples/aggregation_3level_multichild.json");
    let deserialized: Response = serde_json::from_str(&s).unwrap();

    let min = "min_ack_pkts_sent";
    let avg = "avg_ack_pkts_sent";
    let max = "max_ack_pkts_sent";
    let mut first = true;
    let mut count = 0;
    for i in deserialized.aggs().unwrap().into_iter().take(500000) {
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
    let deserialized: Response = serde_json::from_str(&s).unwrap();

    let min = "extstats_ack_pkts_sent_min";
    let avg = "stats_ack_pkts_sent_avg";
    let max = "extstats_ack_pkts_sent_max";
    let stddevu = "extstats_ack_pkts_sent_std_deviation_bounds_upper";
    let mut first = true;
    let mut count = 0;
    for i in deserialized.aggs().unwrap().into_iter().take(500000) {
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
    let deserialized: Response = serde_json::from_str(&s).unwrap();

    let s = "timechart";
    let mut first = true;
    for i in deserialized.aggs().unwrap().into_iter().take(50) {
        if first {
            assert!(i.contains_key(s));
            first = false;
        }
    }
}
