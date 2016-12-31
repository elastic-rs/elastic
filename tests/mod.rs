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

    //FIXME: take all?
//    for i in deserialized.aggs().unwrap().into_iter().take(50000) {
//        println!("Got record {:?}", i);
//    }
    assert_eq!(deserialized.aggs().unwrap().into_iter().count(), 124);

    //FIXME: Shouldn't be a move above
    //    for i in deserialized.aggs().unwrap().into_iter().take(1) {
    //        println!("{}", i);
    //    }
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

    let min = String::from("min_ack_pkts_sent");
    let avg = String::from("avg_ack_pkts_sent");
    let max = String::from("max_ack_pkts_sent");
    let mut first = true;
    let mut count = 0;
    for i in deserialized.aggs().unwrap().into_iter().take(500000) {
        count += 1;
        if first {
            assert!(i.contains_key(&min));
            assert!(i.contains_key(&max));
            assert!(i.contains_key(&avg));
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

    let min = String::from("extstats_ack_pkts_sent_min");
    let avg = String::from("stats_ack_pkts_sent_avg");
    let max = String::from("extstats_ack_pkts_sent_max");
    let mut first = true;
    let mut count = 0;
    for i in deserialized.aggs().unwrap().into_iter().take(500000) {
        count += 1;
        if first {
            assert!(i.contains_key(&min));
            assert!(i.contains_key(&max));
            assert!(i.contains_key(&avg));
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

    let s = String::from("timechart");
    let mut first = true;
    for i in deserialized.aggs().unwrap().into_iter().take(50) {
        if first {
            assert!(i.contains_key(&s));
            first = false;
        }
    }
}
