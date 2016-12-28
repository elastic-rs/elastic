#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

//pub mod reference;

use serde_json::Value;

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


impl IntoIterator for Aggregations {
    type Item = Value;
    type IntoIter = AggregationIterator;

    fn into_iter(self) -> Self::IntoIter {
        AggregationIterator::new(self)
    }
}

#[derive(Debug)]
pub struct AggregationIterator {
    start_at: Option<Value>,
    count: u64,
    aggregations: Aggregations
}

impl AggregationIterator {
    fn new(a: Aggregations) -> AggregationIterator {
        AggregationIterator {
            start_at: None,
            count: 0,
            aggregations: a
        }
    }
}

impl Iterator for AggregationIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        let v = match self.aggregations {
            Aggregations(ref v) => v
        };

//        match self.start_at {
//            None => {
//                self.start_at = Some(*v);
//            },
//            Some(ref mut x) => {
//                self.start_at = Some(*x)
//            }
//        };
////
//        println!("{:#?}", current);

//        println!("{:#?}", v);

        if self.count < 6 {
            Some(Value::U64(0))
        } else {
            None
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
