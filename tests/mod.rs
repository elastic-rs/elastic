#![feature(proc_macro)]

#[macro_use]
extern crate json_str;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate elastic_reqwest;
extern crate elastic_requests;
extern crate elastic_responses;

use elastic_responses::Response;
use std::io::Read;
use std::fs::File;

#[test]
fn test_parse_hits_simple() {
    let mut f = File::open("tests/samples/simple.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let deserialized: Response = serde_json::from_str(&s).unwrap();
    assert_eq!(deserialized.hits().into_iter().count(), 5);
}

#[test]
fn test_parse_simple_aggs() {
    let mut f = File::open("tests/samples/aggregation.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let deserialized: Response = serde_json::from_str(&s).unwrap();

    let mut index = 0;

    for i in deserialized.aggs().unwrap().into_iter().take(50000) {
        index+= 1;
//        println!("Got record {}:\n {:#?}", index, i);
        println!("{:?}", i);
    }

    //FIXME: Shouldn't be a move above
    //    for i in deserialized.aggs().unwrap().into_iter().take(1) {
    //        println!("{}", i);
    //    }

    panic!("done");
}

#[test]
fn test_parse_simple_aggs_no_empty_first_record() {
    let mut f = File::open("tests/samples/aggregation.json").unwrap();
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
