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
    type Item = RowData<'a>; // JPG - str?
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

// struct Foo<'a>(&'a String);

// fn foo() {
//     let a = String::from("a"); // a
//     let b = String::from("b"); // |  b
//                                // |  |
//     Foo(&b);                   // |  |
//                                // |  |
//     let c = a;                 //    |  c
//                                //    |  |
// }




//FIXME: can this be run as a state-machine (ala https://hoverbear.org/2016/10/12/rust-state-machine-pattern/)
//#[derive(Debug)]
//enum AggStates {
//    AtRoot,
//    AtName,
//    InBuckets,
//    AtValues,
//    RowFinished
//}

// Why we might want multiple lifetimes.
// #[derive(Copy)]
// struct Cache<'c, 's> {
//     thing: &'c BTreeMap<usize, &'s str>,
//     position: usize,
// }

// fn foo<'c, 's>(cache: Cache<'c, 's>) -> &'s str {
//     unimplemented!()
// }

// fn main() {
//     let many_strings = "foo";                           // 0 |
//     let z = {                                           // 1 |
//         let map = BTreeMap! { 0 => many_strings };      // 2 | |
//         let cache = Cache { thing: &map, position: 0 }; // 3 | |
//         foo(cache);                                     // 4 | |
//     };                                                  // 5 |
//     println!("{}", z);                                  // 6 |
// }

#[derive(Debug)]
pub struct AggregationIterator<'a> {
    current_row: Option<RowData<'a>>,
    current_row_finished: bool,
    //QUESTION: Tracking traversal usng a stack of Iterators make sense? Is Vec right for this?
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
    type Item = RowData<'a>; // JPG type alias?

    fn next(&mut self) -> Option<RowData<'a>> {
        if self.current_row.is_none() {
            //New row
            self.current_row = Some(BTreeMap::new())
        }

        loop {
            match self.iter_stack.pop() {
                None => {
                    debug! ("ITER: Done!"); // JPG: no space on macro invocation
                    self.current_row = None;
                    break;
                },
                Some(mut i) => {
                    let n = i.1.next(); // JPG tuples used like this are hard to follow
                    //FIXME: can this fail?
                    let active_name = &i.0.unwrap();

                    //Iterate down?
                    let mut has_buckets = false;
                    //Save
                    self.iter_stack.push(i);

                    debug! ("ITER: Depth {}", self.iter_stack.len());
                    //FIXME: Move this, to be able to process first line too
                    match n {
                        None => {
                            //Was nothing here, exit
                            debug! ("ITER: Exit!");
                            self.iter_stack.pop();
                            continue;
                        },
                        Some(n) => {
                            //QUESTION: Destructuring/matching to this extent the right strategy?
                            match self.current_row {
                                Some(ref mut row) => {
                                    debug! ("ITER: Row: {:?}", row);

                                    let o = n.as_object().expect("Shouldn't get here!");
                                    for (key, value) in o {
                                        match *value {
                                            Value::Object(ref c) => {
                                                //Child Aggregation
                                                if let Some(buckets) = c.get("buckets") {
                                                    has_buckets = true;
                                                    if let Value::Array(ref a) = *buckets {
                                                        let i = a.iter();
                                                        self.iter_stack.push((Some(key), i));
                                                    }
                                                    continue;
                                                }
                                                //Simple Value Aggregation Name
                                                if let Some(v) = c.get("value") {
                                                    debug! ("ITER: Insert value! {} {:?}", key, v);
                                                    //QUESTION: Cow right for this use-case? See below
                                                    row.insert(Cow::Borrowed(key), v);
                                                    continue;
                                                }
                                                //Stats
                                                //FIXME: Can be done in loop?

                                                //Stats fields
                                                insert_value("count",c,key,row);
                                                insert_value("min",c,key,row);
                                                insert_value("max",c,key,row);
                                                insert_value("avg",c,key,row);
                                                insert_value("sum",c,key,row);
                                                insert_value("sum_of_squares",c,key,row);
                                                insert_value("variance",c,key,row);
                                                insert_value("std_deviation",c,key,row);

                                                //TODO: std_deviation_bounds
//                                                if c.contains_key("std_deviation_bounds") {
//                                                    let u = c.get("std_deviation_bounds").unwrap().get("upper");
//                                                    let l = c.get("std_deviation_bounds").unwrap().get("lower");
//                                                    let un = format!("{}_std_deviation_bounds_upper", key);
//                                                    let ln = format!("{}_std_deviation_bounds_lower", key);
//                                                    debug! ("ITER: Insert std_dev_bounds! {} {} u: {:?} l: {:?}", un, ln, u.unwrap(), l.unwrap());
//                                                    row.insert(Cow::Owned(un), u.unwrap());
//                                                    row.insert(Cow::Owned(ln), l.unwrap());
//                                                }
                                            },
                                            _ => ()
                                        }
                                        //Bucket Aggregation Name
                                        if key == "key" {
                                            debug! ("ITER: Insert bucket! {} {:?}", active_name, value);
                                            row.insert(Cow::Borrowed(active_name), value);
                                        }
                                        //Bucket Aggregation Count
                                        if key == "doc_count" {
                                            debug! ("ITER: Insert bucket count! {} {:?}", active_name, value);
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
                        debug! ("ITER: Bucketless!");
                        break;
                    } else {
                        debug! ("ITER: Dive!");
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

// JPG place impls closer to defns
impl Response {
    pub fn hits(&self) -> &Vec<Value> {
        &self.hits.hits()
    }

    pub fn aggs(&self) -> Option<&Aggregations> {
        self.aggregations.as_ref()
    }
}

impl Hits {
    pub fn hits(&self) -> &Vec<Value> { // JPG http://stackoverflow.com/q/40006219/155423
        &self.hits
    }
}

#[derive(Deserialize, Debug)]
struct Hit {
    _index: String // JPG suspicious
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
