#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

//pub mod reference;

use serde_json::Value;
use std::slice::Iter;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct Response {
    took: u64,
    timed_out: bool,
    _shards: Shards,
    hits: Hits,
    aggregations: Option<Aggregations>,
    status: Option<u16>
}

#[derive(Deserialize, Debug)]
pub struct Aggregations(Value);


impl<'a> IntoIterator for &'a Aggregations {
//    type Item = &'a Value;
    type Item = BTreeMap<&'a String, &'a Value>;
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

#[derive(Debug)]
pub struct AggregationIterator<'a> {
    start_at: Option<&'a Value>,
    current_name: Option<&'a String>,
    currect_buckets: Option<&'a Value>,
    currect_buckets_iter: Option<Iter<'a, Value>>,
    current_row: Option<BTreeMap<&'a String, &'a Value>>,
    call_stack: Option<Vec<&'a Value>>,
    parent_node: Option<&'a Value>,
    count: u64,
    aggregations: &'a Aggregations
}

impl<'a> AggregationIterator<'a> {
    fn new(a: &'a Aggregations) -> AggregationIterator<'a> {

        let v = match a {
            &Aggregations(ref v) => v
        };

        AggregationIterator {
            start_at: Some(v),
            current_name: None,
            currect_buckets: None,
            currect_buckets_iter: None,
            current_row: None,
            call_stack: None,
            parent_node: None,
            count: 0,
            aggregations: a
        }
    }
}

impl<'a> Iterator for AggregationIterator<'a> {
//    type Item = &'a Value;
    type Item = BTreeMap<&'a String, &'a Value>;

//    fn next(&mut self) -> Option<&'a Value> {
    fn next(&mut self) -> Option<BTreeMap<&'a String, &'a Value>> {

        let v = self.start_at.unwrap();

        match self.currect_buckets_iter {
            None => {


                match *v {
                    Value::Object(ref o) => {
                        for (key, child) in o {
                            println!("{}", key);
                            println!("{}", child);
                            if let Value::Object(ref c) = *child {
                                if c.contains_key("buckets") {
//                                    println!("Matched {:?}!", c["buckets"]);
                                    self.currect_buckets = Some(&c["buckets"]);
                                    if let Value::Array(ref a) = *self.currect_buckets.unwrap() {
//                                        println!("Got array");
                                        self.currect_buckets_iter = Some(a.iter());
                                    }
                                }
                            }
                        }
                    },
                    _ => {
                        //FIXME: do something sensible here
                        panic!("Not implemented");
                    }
                };
            },
            Some(ref mut i) => {
                println!("ITER: {:#?}", i.next());
            }

        }

        let r = Some(BTreeMap::new());
//        match self.start_at {
//            Some(x) => Some(x),
//            None => None
//        }
        r

    }
}

#[derive(Deserialize, Debug)]
struct Shards {
    total: u32,
    successful: u32,
    failed: u32
}

#[derive(Deserialize, Debug)]
pub struct Hits {
    total: u64,
    max_score: u64,
    hits: Vec<Value>
}

impl Response {
    pub fn hits(&self) -> &Vec<Value> {
        &self.hits.hits()
    }

    pub fn aggs(self) -> std::option::Option<Aggregations> {
        self.aggregations
    }
}

impl Hits {
    pub fn hits(&self) -> &Vec<Value> {
        &self.hits
    }
}

#[derive(Deserialize, Debug)]
struct Hit {
    _index: String
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
