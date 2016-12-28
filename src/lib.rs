#![feature(custom_derive)]
#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod definition;

#[derive(Deserialize, Debug)]
pub struct Response {
    took: u64,
    timed_out: bool,
    _shards: Shards,
    hits: Hits,
//    aggregations: Option<serde_json::Value>,
    aggregations: Aggregations,
    status: Option<u16>
}

#[derive(Deserialize, Debug)]
struct Aggregations(Option<serde_json::Value>);

// and we'll implement IntoIterator
impl IntoIterator for Aggregations {
    type Item = serde_json::Value;
    type IntoIter = AggregationIterator;

    fn into_iter(self) -> Self::IntoIter {
//        self.0.into_iter()
        AggregationIterator::new(self)
    }
}

struct AggregationIterator {
    count: usize,
    aggregations: Aggregations
}

impl AggregationIterator {
    //FIXME: right name?
    fn new(a: Aggregations) -> AggregationIterator {
        AggregationIterator {
            count: 0,
            aggregations: a
        }
    }
}

impl Iterator for AggregationIterator {
    // we will be counting with usize
    type Item = serde_json::Value;

    // next() is the only required method
    fn next(&mut self) -> Option<serde_json::Value> {
        // increment our count. This is why we started at zero.
//        self.count += 1;

        // check to see if we've finished counting or not.
//        if self.count < 6 {
//            Some(self.count)
//        } else {
//            None
//        }
        None
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
    hits: Vec<serde_json::Value>
}

impl Response {
    pub fn hits(&self) -> &Vec<serde_json::Value> {
        &self.hits.hits()
    }
}

impl Hits {
    pub fn hits(&self) -> &Vec<serde_json::Value> {
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
    fn it_works() {
    }
}
