#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate slog_stdlog;
extern crate slog_envlogger;

use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::slice::Iter;

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


//QUESTION: Why do lifetimes go where they go, what am I missing?
//          What happens when lifetimes shadow each other?
//          Mental model for this?
//          Below works thanks to `misdreavus` on IRC, but I don't quite know why
impl<'a> IntoIterator for &'a Aggregations {
//    type Item = BTreeMap<&'a String, &'a Value>;
    type Item = BTreeMap<Cow<'a, String>, &'a Value>;
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

//FIXME: can this be run as a state-machine (ala https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
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
    current_row: Option<BTreeMap<Cow<'a, String>, &'a Value>>,
    current_row_finished: bool,
    //QUESTION: Tracking traversal usng a stack of Iterators make sense? Is Vec right for this?
    iter_stack: Vec<(Option<&'a String>, Iter<'a, Value>)>,
//    field_names_owned: Vec<String>,
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
                //FIXME: Bad for lib
                panic!("Not implemented, we only cater for bucket objects");
            }
        };

        AggregationIterator {
            current_row: None,
            current_row_finished: false,
//            field_names_owned: n,
            iter_stack: s,
            aggregations: a
        }
    }
}

impl<'a> Iterator for AggregationIterator<'a> {
    type Item = BTreeMap<Cow<'a, String>, &'a Value>;
//    type Item = BTreeMap<&'a String, &'a Value>;

//    fn next(&mut self) -> Option<BTreeMap<&'a String, &'a Value>> {
    fn next(&mut self) -> Option<BTreeMap<Cow<'a, String>, &'a Value>> {
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
                    debug!("ITER: Done!");
                    match self.current_row {
                        Some(_) => {
                            self.current_row = None
                        },
                        _ => ()
                    };
                    break;
                },
                Some(mut i) => {
                    let n = i.1.next();
                    //FIXME: can this fail?
                    let active_name = &i.0.unwrap();

                    //Iterate down?
                    let mut has_buckets = false;
                    //Save
                    self.iter_stack.push(i);

                    debug!("ITER: Depth {}", self.iter_stack.len());
                    //FIXME: Move this, to be able to process first line too
                    match n {
                        None => {
                            //Was nothing here, exit
                            debug!("ITER: Exit!");
                            self.iter_stack.pop();
                            continue;
                        },
                        Some(n) => {
                            //QUESTION: Destructuring/matching to this extent the right strategy?
                            match self.current_row {
                                Some(ref mut row) => {
                                    debug!("ITER: Row: {:?}", row);

                                    let o = match *n {
                                        Value::Object(ref o) => o,
                                        _ => panic!("Shouldn't get here!")
                                    };
                                    for (key, value) in o {
                                        match *value {
                                            Value::Object(ref c) => {
                                                //Child Aggregation
                                                if c.contains_key("buckets") {
                                                    has_buckets = true;
                                                    if let Value::Array(ref a) = c["buckets"] {
                                                        let i = a.iter();
                                                        self.iter_stack.push((Some(key), i));
                                                    }
                                                }
                                                //Simple Value Aggregation Name
                                                if c.contains_key("value") {
                                                    let v = c.get("value");
                                                    //FIXME: Can this fail?
                                                    debug!("ITER: Insert value! {} {:?}", key, v.unwrap());
                                                    //QUESTION: Cow right for this use-case? See below
                                                    row.insert(Cow::Borrowed(key), v.unwrap());
                                                }
                                            },
                                            _ => ()
                                        }
                                        //Bucket Aggregation Name
                                        if key == "key" {
                                            debug!("ITER: Insert bucket! {} {:?}", active_name, value);
                                            row.insert(Cow::Borrowed(active_name), value);
                                        }
                                        //Bucket Aggregation Count
                                        if key == "doc_count" {
                                            debug!("ITER: Insert bucket count! {} {:?}", active_name, value);
                                            let field_name = format!("{}_doc_count", active_name);
                                            row.insert(Cow::Owned(field_name), value);
                                        }
                                    }
                                    //Return here?
                                    if !has_buckets {
                                        return Some(row.clone());
                                    }
                                },
                                _ => ()
                            }
                        }
                    }

                    if !has_buckets {
                        debug!("ITER: Bucketless!");
                        break;
                    } else {
                        debug!("ITER: Dive!");
                    }
                }
            };
        }

        match self.current_row {
            //FIXME: Refactor to avoid this clone()
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
