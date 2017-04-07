use serde::Deserialize;
use serde_json::{Map, Value};

use parse::MaybeOkResponse;
use super::{HttpResponse, FromResponse, ApiResult};

use std::io::Read;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::slice::Iter;

/// Main `struct` of the crate, provides access to the `hits` and `aggs` iterators.
#[derive(Deserialize, Debug)]
pub struct SearchResponseOf<T: Deserialize> {
    pub took: u64,
    pub timed_out: bool,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub hits: Hits<T>,
    pub aggregations: Option<Aggregations>,
    pub status: Option<u16>,
}

pub type SearchResponse = SearchResponseOf<Hit<Value>>;

impl<T: Deserialize> FromResponse for SearchResponseOf<T> {
    fn from_response<I: Into<HttpResponse<R>>, R: Read>(res: I) -> ApiResult<Self> {
        let res = res.into();

        res.response(|res| {
            match res.status() {
                200...299 => Ok(MaybeOkResponse::ok(res)),
                _ => Ok(MaybeOkResponse::err(res)),
            }
        })
    }
}

impl<T: Deserialize> SearchResponseOf<T> {
    /// Returns an Iterator to the search results or hits of the response.
    pub fn hits(&self) -> &[T] {
        &self.hits.hits()
    }

    /// Returns an Iterator to the search results or aggregations part of the response.
    ///
    /// This Iterator transforms the tree-like JSON object into a row/table
    /// based format for use with standard iterator adaptors.
    pub fn aggs(&self) -> &Aggregations {
        // FIXME: Create empty aggregation, remove unwrap()
        self.aggregations.as_ref().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Shards {
    pub total: u32,
    pub successful: u32,
    pub failed: u32,
}

/// Struct to hold the search's Hits, serializable to type `T` or `serde_json::Value`
#[derive(Deserialize, Debug)]
pub struct Hits<T: Deserialize> {
    pub total: u64,
    pub max_score: Option<u64>,
    pub hits: Vec<T>,
}

impl<T: Deserialize> Hits<T> {
    fn hits(&self) -> &[T] {
        &self.hits
    }
}

#[derive(Deserialize, Debug)]
pub struct Hit<T: Deserialize> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_score")]
    pub score: f32,
    #[serde(rename = "_source")]
    pub source: Option<T>,
    #[serde(rename="_routing")]
    pub routing: Option<String>,
}

/// Type Struct to hold a generic `serde_json::Value` tree of the Aggregation results.
#[derive(Deserialize, Debug)]
pub struct Aggregations(Value);

impl<'a> IntoIterator for &'a Aggregations {
    type Item = RowData<'a>;
    type IntoIter = AggregationIterator<'a>;

    fn into_iter(self) -> AggregationIterator<'a> {
        AggregationIterator::new(self)
    }
}

/// Aggregator that traverses the results from Elasticsearch's Aggregations and returns a result
/// row by row in a table-styled fashion.
#[derive(Debug)]
pub struct AggregationIterator<'a> {
    current_row: Option<RowData<'a>>,
    current_row_finished: bool,
    iter_stack: Vec<(Option<&'a String>, Iter<'a, Value>)>,
    aggregations: &'a Aggregations,
}

impl<'a> AggregationIterator<'a> {
    fn new(a: &'a Aggregations) -> AggregationIterator<'a> {
        let o = a.0
            .as_object()
            .expect("Not implemented, we only cater for bucket objects");
        // FIXME: Bad for lib // JPG: quick-error

        let s = o.into_iter()
            .filter_map(|(key, child)| {
                child.as_object()
                    .and_then(|child| child.get("buckets"))
                    .and_then(Value::as_array)
                    .map(|array| (Some(key), array.iter()))
            })
            .collect();

        AggregationIterator {
            current_row: None,
            current_row_finished: false,
            iter_stack: s,
            aggregations: a,
        }
    }
}

type Object = Map<String, Value>;
type RowData<'a> = BTreeMap<Cow<'a, str>, &'a Value>;

fn insert_value<'a>(fieldname: &str,
                    json_object: &'a Object,
                    keyname: &str,
                    rowdata: &mut RowData<'a>) {
    if let Some(v) = json_object.get(fieldname) {
        let field_name = format!("{}_{}", keyname, fieldname);
        debug!("ITER: Insert value! {} {:?}", field_name, v);
        rowdata.insert(Cow::Owned(field_name), v);
    }
}

impl<'a> Iterator for AggregationIterator<'a> {
    type Item = RowData<'a>;

    fn next(&mut self) -> Option<RowData<'a>> {
        if self.current_row.is_none() {
            // New row
            self.current_row = Some(BTreeMap::new())
        }

        loop {
            if let Some(mut i) = self.iter_stack.pop() {
                let n = i.1.next();

                // FIXME: can this fail?
                let active_name = &i.0.unwrap();

                // Iterate down?
                let mut has_buckets = false;
                // Save
                self.iter_stack.push(i);

                debug!("ITER: Depth {}", self.iter_stack.len());
                // FIXME: Move this, to be able to process first line too
                if let Some(n) = n {
                    if let Some(ref mut row) = self.current_row {
                        debug!("ITER: Row: {:?}", row);

                        for (key, value) in n.as_object().expect("Shouldn't get here!") {
                            if let Some(c) = value.as_object() {
                                // Child Aggregation
                                if let Some(buckets) = c.get("buckets") {
                                    has_buckets = true;
                                    if let Value::Array(ref a) = *buckets {
                                        self.iter_stack.push((Some(key), a.iter()));
                                    }
                                    continue;
                                }
                                // Simple Value Aggregation Name
                                if let Some(v) = c.get("value") {
                                    debug!("ITER: Insert value! {} {:?}", key, v);
                                    row.insert(Cow::Borrowed(key), v);
                                    continue;
                                }
                                // Stats fields
                                insert_value("count", c, key, row);
                                insert_value("min", c, key, row);
                                insert_value("max", c, key, row);
                                insert_value("avg", c, key, row);
                                insert_value("sum", c, key, row);
                                insert_value("sum_of_squares", c, key, row);
                                insert_value("variance", c, key, row);
                                insert_value("std_deviation", c, key, row);

                                if c.contains_key("std_deviation_bounds") {
                                    if let Some(child_values) = c.get("std_deviation_bounds")
                                        .unwrap()
                                        .as_object() {
                                        let u = child_values.get("upper");
                                        let l = child_values.get("lower");
                                        let un = format!("{}_std_deviation_bounds_upper", key);
                                        let ln = format!("{}_std_deviation_bounds_lower", key);
                                        debug!("ITER: Insert std_dev_bounds! {} {} u: {:?} l: \
                                                {:?}",
                                               un,
                                               ln,
                                               u.unwrap(),
                                               l.unwrap());
                                        row.insert(Cow::Owned(un), u.unwrap());
                                        row.insert(Cow::Owned(ln), l.unwrap());
                                    }
                                }
                            }

                            if key == "key" {
                                // Bucket Aggregation Name
                                debug!("ITER: Insert bucket! {} {:?}", active_name, value);
                                row.insert(Cow::Borrowed(active_name), value);
                            } else if key == "doc_count" {
                                // Bucket Aggregation Count
                                debug!("ITER: Insert bucket count! {} {:?}", active_name, value);
                                let field_name = format!("{}_doc_count", active_name);
                                row.insert(Cow::Owned(field_name), value);
                            }
                        }
                    }
                } else {
                    // Was nothing here, exit
                    debug!("ITER: Exit!");
                    self.iter_stack.pop();
                    continue;
                }

                if !has_buckets {
                    debug!("ITER: Bucketless!");
                    break;
                } else {
                    debug!("ITER: Dive!");
                }
            } else {
                debug!("ITER: Done!");
                self.current_row = None;
                break;
            };
        }

        match self.current_row {
            // FIXME: Refactor to avoid this clone()
            Some(ref x) => Some(x.clone()),
            None => None,
        }
    }
}
