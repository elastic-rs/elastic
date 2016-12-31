#![feature(proc_macro)]

extern crate elastic_reqwest;
extern crate elastic_requests;
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
use std::io::Read;
use std::fs::File;

#[test]
fn test_parse_hits_simple() {
    let mut f = File::open("tests/samples/hits_only.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let deserialized: Response = serde_json::from_str(&s).unwrap();
    assert_eq!(deserialized.hits().into_iter().count(), 5);
}

#[test]
fn test_parse_simple_aggs() {
    slog_envlogger::init().unwrap();
    let mut f = File::open("tests/samples/aggregation_simple.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let deserialized: Response = serde_json::from_str(&s).unwrap();

    // let mut i = deserialized.aggs().unwrap().into_iter();

    // for x in i.by_ref().take(3) { println!("1") };
    // for x in i.take(4) { println!("2") };

    for i in deserialized.aggs().unwrap() {
        println!("Got record {:?}", i);
    }
    assert_eq!(deserialized.aggs().unwrap().into_iter().count(), 124);

    for i in deserialized.aggs().unwrap().into_iter().take(1) {
        println!("{:?}", i);
    }
}

#[test]
fn test_parse_3level_aggs() {
    let mut f = File::open("tests/samples/aggregation_3level.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let deserialized: Response = serde_json::from_str(&s).unwrap();

    assert_eq!(deserialized.aggs().unwrap().into_iter().count(), 201);
}

#[test]
fn test_parse_3level_multichild_aggs() {
    let mut f = File::open("tests/samples/aggregation_3level_multichild.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

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
    let mut f = File::open("tests/samples/aggregation_3level_multistats.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let deserialized: Response = serde_json::from_str(&s).unwrap();

    let min = "extstats_ack_pkts_sent_min";
    let avg = "stats_ack_pkts_sent_avg";
    let max = "extstats_ack_pkts_sent_max";
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
    assert_eq!(count, 61);

}

#[test]
fn test_parse_simple_aggs_no_empty_first_record() {

    let mut f = File::open("tests/samples/aggregation_simple.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

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
