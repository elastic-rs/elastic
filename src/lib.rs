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
    type Item = BTreeMap<&'a String, &'a Value>;
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

//#[derive(Debug)]
//enum AggStates {
//    AtRoot,
//    AtName,
//    InBuckets,
//    AtValues,
//    RowFinished
//}

#[derive(Debug)]
pub struct AggregationIterator<'a> {
    //    state: AggStates,
    //    start_at: Option<&'a Value>,
//    current_name: Option<&'a String>,
//    currect_buckets: Option<&'a Value>,
//    currect_buckets_iter: Option<Iter<'a, Value>>,
    current_row: Option<BTreeMap<&'a String, &'a Value>>,
    iter_stack: Vec<(Option<&'a String>, Iter<'a, Value>)>,
//    parent_node: Option<&'a Value>,
//    count: u64,
    aggregations: &'a Aggregations
}

impl<'a> AggregationIterator<'a> {
    fn new(a: &'a Aggregations) -> AggregationIterator<'a> {
        let v = match a {
            &Aggregations(ref v) => v
        };

        let mut s: Vec<(Option<&String>, Iter<Value>)> = Vec::new();

        match *v {
            Value::Object(ref o) => {
                for (key, child) in o {
                    if let Value::Object(ref c) = *child {
                        if c.contains_key("buckets") {
                            if let Value::Array(ref a) = c["buckets"] {
                                let i = a.iter();
                                s.push((Some(&key), i));
                            }
                        }
                    }
                }
            },
            _ => {
                //FIXME: do something sensible here
                panic!("Not implemented, only caters for bucket objects");
            }
        };

        AggregationIterator {
            //            state: AggStates::AtRoot,
            //            start_at: Some(v),
//            current_name: None,
//            currect_buckets: None,
//            currect_buckets_iter: None,
            current_row: None,
            iter_stack: s,
//            parent_node: None,
//            count: 0,
            aggregations: a
        }
    }
}

//fn has_child_aggs() {
//
//}

impl<'a> Iterator for AggregationIterator<'a> {
    type Item = BTreeMap<&'a String, &'a Value>;

    fn next(&mut self) -> Option<BTreeMap<&'a String, &'a Value>> {
        match self.current_row {
            None => {
                //New row
                self.current_row = Some(BTreeMap::new())
            },
            Some(_) => ()
        };

        loop {
            match self.iter_stack.pop() {
                None => {
//                    println!("Done!");
//                    return None;
                    match self.current_row {
//                        None => {
//                            //New row
//                            self.current_row = Some(BTreeMap::new())
//                        },
                        Some(_) => {
                            self.current_row = None
                        },
                        _ => ()
                    };
                    break;
                },
                Some(mut i) => {
                    //                    println!("ITER: {:#?}", i);
                    let n = i.1.next();
                    //                    println!("N: {:?}", n);
                    let active_name = &i.0.unwrap();
//                    println!("Active Name! {}", active_name);
                    let mut has_buckets = false;
                    self.iter_stack.push(i);

//                    println!("Loop {:?} {}", active_name, self.iter_stack.len());
                    //FIXME: Move this, to be able to process first line too
                    match n {
                        None => {
                            //Was nothing here, exit
//                            println!("Exit!");
                            self.iter_stack.pop();
                            break;
                        },
                        Some(n) => {
                            match self.current_row {
                                Some(ref mut r) => {
                                    let row = r;
                                    //                                    println!("Insert! {} {:?}", active_name, value);
                                    //                                    row.insert(active_name, value);

                                    let o = match *n {
                                        Value::Object(ref o) => o,
                                        _ => panic!("Shouldn't get here!")
                                    };
                                    for (key, value) in o {
//                                        println!("KEY: {:?}, VALUE: {:?}", key, value);
                                        match *value {
                                            Value::Object(ref c) => {
                                                //Child Aggregation
                                                if c.contains_key("buckets") {
                                                    has_buckets = true;
                                                    if let Value::Array(ref a) = c["buckets"] {
//                                                        println!("Current Colection: {}", key);
                                                        let i = a.iter();
                                                        self.iter_stack.push((Some(key), i));
                                                    }
                                                }
                                                //Simple Value Aggregation Name
                                                if c.contains_key("value") {
                                                    let v = c.get("value");
//                                                    println!("Insert! {} {:?}", key, v.unwrap());
                                                    row.insert(key, v.unwrap());
                                                }
                                            },
                                            _ => ()
                                        }
                                        //Bucket Aggregation Name
                                        if key == "key" {
//                                            println!("Insert! {} {:?}", key, value);
                                            row.insert(active_name, value);
                                        }
                                    }
                                },
                                _ => ()
                            }
                        }
                    }

                    if !has_buckets {
//                        println!("Bucketless!");
                        self.iter_stack.pop();
                        break;
                    } else {
//                        println!("Dive!");
//                        continue;
                    }
                }
            };
        }

        match self.current_row {
            //FIXME: Refactor to avoid this
            Some(ref x) => Some(x.clone()),
            None => None
        }
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
