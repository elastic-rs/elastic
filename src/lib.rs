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


impl<'a> IntoIterator for &'a Aggregations {
    type Item = &'a Value;
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

#[derive(Debug)]
pub struct AggregationIterator<'a> {
    start_at: Option<&'a Value>,
    count: u64,
    aggregations: &'a Aggregations
}

impl<'a> AggregationIterator<'a> {
    fn new(a: &'a Aggregations) -> AggregationIterator<'a> {

        let v = match a {
            &Aggregations(ref v) => v
        };

//        println!("{:#?}", v);

        AggregationIterator {
            start_at: Some(v),
            count: 0,
            aggregations: a
        }
    }
}

enum IterationState {
    AtNameNode { node: &Value },
    AtBuckets,
    AtValue,
    AtStats,
    AtExStats,
    Rowdone
}

impl<'a> Iterator for AggregationIterator<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<&'a Value> {

        match self.start_at {
            Some(x) => Some(x),
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
