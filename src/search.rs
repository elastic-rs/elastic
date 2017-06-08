use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use common::{DefaultAllocatedField, Shards};
use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::slice::Iter;

/// Response for a [search request](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html).
/// 
/// This is the main `struct` of the crate, provides access to the `hits` and `aggs` iterators.
/// 
/// # Examples
/// 
/// ```no_run
/// # extern crate elastic_responses;
/// # use elastic_responses::SearchResponse;
/// # fn do_request() -> SearchResponse { unimplemented!() }
/// # fn main() {
/// // Send a request (omitted, see `samples/basic`), and read the response.
/// // Parse body to JSON as an elastic_responses::SearchResponse object
/// let body_as_json: SearchResponse = do_request();
///
/// // Use hits() or aggs() iterators
/// // Hits
/// for i in body_as_json.hits() {
///   println!("{:?}",i);
/// }
///
/// // Agregations
/// for i in body_as_json.aggs() {
///   println!("{:?}",i);
/// }
/// # }
/// ```
#[derive(Deserialize, Debug)]
pub struct SearchResponse<T> {
    pub took: u64,
    pub timed_out: bool,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub hits: Hits<Hit<T>>,
    pub aggregations: Option<Aggregations>,
    pub status: Option<u16>,
}

impl<T: DeserializeOwned> IsOk for SearchResponse<T> {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}

impl<T> SearchResponse<T> {
    /// Returns an Iterator to the search results or hits of the response.
    pub fn hits(&self) -> &[Hit<T>] {
        self.hits.hits()
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

/// Struct to hold the search's Hits, serializable to type `T` or `serde_json::Value`
#[derive(Deserialize, Debug)]
pub struct Hits<T> {
    pub total: u64,
    pub max_score: Option<f32>,
    pub hits: Vec<T>,
}

impl<T> Hits<T> {
    fn hits(&self) -> &[T] {
        &self.hits
    }
}

/// Full metadata and source for a single hit.
#[derive(Deserialize, Debug)]
pub struct Hit<T> {
    #[serde(rename = "_index")]
    pub index: DefaultAllocatedField,
    #[serde(rename = "_type")]
    pub ty: DefaultAllocatedField,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    #[serde(rename = "_score")]
    pub score: Option<f32>,
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
