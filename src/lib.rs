#![feature(custom_derive)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate slog_stdlog;
extern crate slog_envlogger;

use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::slice::Iter;

//let mut i = deserialized.aggs().unwrap().into_iter();
//
//for x in i.by_ref().take(3) { println!("1") };
//for x in i.take(4) { println!("2") };
//
//for i in deserialized.aggs().unwrap() {
//    println!("Got record {:?}", i);
//}
//
//for i in deserialized.aggs().unwrap().into_iter().take(1) {
//    println!("{:?}", i);
//}

impl<T: Deserialize> ResponseOf<T> {
    pub fn hits(&self) -> &Vec<T> {
        &self.hits.hits()
    }

    pub fn aggs(&self) -> &Aggregations {
        //FIXME: Create empty aggregation, remove unwrap()
        self.aggregations.as_ref().unwrap()
    }
}

pub type Response = ResponseOf<Value>;

#[derive(Deserialize, Debug)]
pub struct ResponseOf<T: Deserialize> {
    took: u64,
    timed_out: bool,
    _shards: Shards,
    hits: Hits<T>,
    aggregations: Option<Aggregations>,
    status: Option<u16>
}

#[derive(Deserialize, Debug)]
pub struct Aggregations(Value);

impl<'a> IntoIterator for &'a Aggregations {
    type Item = RowData<'a>;
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

#[derive(Debug)]
pub struct AggregationIterator<'a> {
    current_row: Option<RowData<'a>>,
    current_row_finished: bool,
    iter_stack: Vec<(Option<&'a String>, Iter<'a, Value>)>,
    aggregations: &'a Aggregations
}

impl<'a> AggregationIterator<'a> {
    fn new(a: &'a Aggregations) -> AggregationIterator<'a> {
        let o = a.0.as_object()
            .expect("Not implemented, we only cater for bucket objects");
        //FIXME: Bad for lib // JPG: quick-error

        let s = o.into_iter().filter_map(|(key, child)| {
            child.as_object()
                .and_then(|child| child.get("buckets"))
                .and_then(Value::as_array)
                .map(|array| (Some(key), array.iter()))
        }).collect();

        AggregationIterator {
            current_row: None,
            current_row_finished: false,
            iter_stack: s,
            aggregations: a
        }
    }
}

type Object = BTreeMap<String, Value>;
type RowData<'a> = BTreeMap<Cow<'a, str>, &'a Value>;

fn insert_value<'a>(fieldname: &str, json_object: &'a Object, keyname: &str, rowdata: &mut RowData<'a>) {
    if let Some(v) = json_object.get(fieldname) {
        let field_name = format!("{}_{}", keyname, fieldname);
        debug! ("ITER: Insert value! {} {:?}", field_name, v);
        rowdata.insert(Cow::Owned(field_name), v);
    }
}

impl<'a> Iterator for AggregationIterator<'a> {
    type Item = RowData<'a>;

    fn next(&mut self) -> Option<RowData<'a>> {
        if self.current_row.is_none() {
            //New row
            self.current_row = Some(BTreeMap::new())
        }

        loop {
            if let Some(mut i) = self.iter_stack.pop() {
                let n = i.1.next();

                //FIXME: can this fail?
                let active_name = &i.0.unwrap();

                //Iterate down?
                let mut has_buckets = false;
                //Save
                self.iter_stack.push(i);

                debug! ("ITER: Depth {}", self.iter_stack.len());
                //FIXME: Move this, to be able to process first line too
                if let Some(n) = n {
                    if let Some(ref mut row) = self.current_row {
                        debug! ("ITER: Row: {:?}", row);

                        for (key, value) in n.as_object().expect("Shouldn't get here!") {
                            if let Some(c) = value.as_object() {
                                //Child Aggregation
                                if let Some(buckets) = c.get("buckets") {
                                    has_buckets = true;
                                    if let Value::Array(ref a) = *buckets {
                                        self.iter_stack.push((Some(key), a.iter()));
                                    }
                                    continue;
                                }
                                //Simple Value Aggregation Name
                                if let Some(v) = c.get("value") {
                                    debug! ("ITER: Insert value! {} {:?}", key, v);
                                    row.insert(Cow::Borrowed(key), v);
                                    continue;
                                }
                                //Stats fields
                                insert_value("count", c, key, row);
                                insert_value("min", c, key, row);
                                insert_value("max", c, key, row);
                                insert_value("avg", c, key, row);
                                insert_value("sum", c, key, row);
                                insert_value("sum_of_squares", c, key, row);
                                insert_value("variance", c, key, row);
                                insert_value("std_deviation", c, key, row);

                                if c.contains_key("std_deviation_bounds") {
                                    if let Some(child_values) = c.get("std_deviation_bounds").unwrap().as_object() {
                                        let u = child_values.get("upper");
                                        let l = child_values.get("lower");
                                        let un = format!("{}_std_deviation_bounds_upper", key);
                                        let ln = format!("{}_std_deviation_bounds_lower", key);
                                        debug! ("ITER: Insert std_dev_bounds! {} {} u: {:?} l: {:?}", un, ln, u.unwrap(), l.unwrap());
                                        row.insert(Cow::Owned(un), u.unwrap());
                                        row.insert(Cow::Owned(ln), l.unwrap());
                                    }
                                }
                            }

                            if key == "key" {
                                //Bucket Aggregation Name
                                debug! ("ITER: Insert bucket! {} {:?}", active_name, value);
                                row.insert(Cow::Borrowed(active_name), value);
                            } else if key == "doc_count" {
                                //Bucket Aggregation Count
                                debug! ("ITER: Insert bucket count! {} {:?}", active_name, value);
                                let field_name = format!("{}_doc_count", active_name);
                                row.insert(Cow::Owned(field_name), value);
                            }
                        }
                    }
                } else {
                    //Was nothing here, exit
                    debug! ("ITER: Exit!");
                    self.iter_stack.pop();
                    continue;
                }

                if !has_buckets {
                    debug! ("ITER: Bucketless!");
                    break;
                } else {
                    debug! ("ITER: Dive!");
                }
            } else {
                debug! ("ITER: Done!");
                self.current_row = None;
                break;
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
pub struct Hits<T: Deserialize> {
    total: u64,
    max_score: u64,
    hits: Vec<T>
}

impl<T: Deserialize> Hits<T> {
    pub fn hits(&self) -> &Vec<T> {
        // JPG http://stackoverflow.com/q/40006219/155423
        &self.hits
    }
}

#[derive(Deserialize, Debug)]
struct Hit {
    _index: String
}
