/*!
Response types for a [search request](https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html).
*/

use serde::de::DeserializeOwned;
use serde_json::{
    Map,
    Value,
};

use common::Shards;
use parsing::IsOkOnSuccess;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    slice::Iter,
    vec::IntoIter,
};

/**
Response for a [search request][search-req].

This is the main `struct` of the crate, provides access to the `hits` and `aggs` iterators.

# Aggregations

Aggregations currently have the following limitations:

- Only metric aggregations nested in buckets are supported
- Only [Simple Metric Aggregations][metric-aggs] like `avg`, `min`, `max`, `sum` and [Stats Aggregations][stats-aggs] are supported

# Examples

Iterate over the hits in a search response:

```no_run
# extern crate elastic_responses;
# use elastic_responses::{SearchResponse, Value};
# fn do_request() -> SearchResponse<Value> { unimplemented!() }
# fn main() {
let response: SearchResponse<Value> = do_request();

// Iterate over hits. Could also use `documents`
for hit in response.hits() {
    let score = hit.score().unwrap_or(f32::default());
    let doc = hit.document();

    println!("score: {}", score);
    println!("doc: {:?}", doc);
}
# }
```

[search-req]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html
[metric-aggs]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics.html
[stats-aggs]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-aggregations-metrics-stats-aggregation.html
*/
#[derive(Deserialize, Debug)]
pub struct SearchResponse<T> {
    took: u64,
    timed_out: bool,
    #[serde(rename = "_shards")]
    shards: Shards,
    hits: HitsWrapper<T>,
    aggregations: Option<AggsWrapper>,
    status: Option<u16>,
}

/** Struct to hold the search's Hits, serializable to type `T` or `serde_json::Value`. */
#[derive(Deserialize, Debug)]
struct HitsWrapper<T> {
    total: u64,
    max_score: Option<f32>,
    #[serde(rename = "hits")]
    inner: Vec<Hit<T>>,
}

impl<T> SearchResponse<T> {
    /** Time in milliseconds it took for Elasticsearch to process the request. */
    pub fn took(&self) -> u64 {
        self.took
    }

    /** Whether or not the request timed out before completing. */
    pub fn timed_out(&self) -> bool {
        self.timed_out
    }

    /** Shards metadata for the request. */
    pub fn shards(&self) -> &Shards {
        &self.shards
    }

    /** A http status associated with the response. */
    pub fn status(&self) -> Option<u16> {
        self.status.clone()
    }

    /** The total number of documents that matched the search query. */
    pub fn total(&self) -> u64 {
        self.hits.total
    }

    /** The max score for documents that matched the search query. */
    pub fn max_score(&self) -> Option<f32> {
        self.hits.max_score.clone()
    }

    /** Iterate over the hits matched by the search query. */
    pub fn hits(&self) -> Hits<T> {
        Hits::new(&self.hits)
    }

    /** Convert the response into an iterator that consumes the hits. */
    pub fn into_hits(self) -> IntoHits<T> {
        IntoHits::new(self.hits)
    }

    /**
    Iterate over the documents matched by the search query.

    This iterator emits just the `_source` field for the returned hits.
    */
    pub fn documents(&self) -> Documents<T> {
        Documents::new(&self.hits)
    }

    /** Convert the response into an iterator that consumes the documents. */
    pub fn into_documents(self) -> IntoDocuments<T> {
        IntoDocuments::new(self.hits)
    }

    /**
    Iterate over the aggregations in the response.

    This Iterator transforms the tree-like JSON object into a row/table based format for use with standard iterator adaptors.
    */
    pub fn aggs(&self) -> Aggs {
        Aggs::new(self.aggregations.as_ref())
    }

    /**
    Get a reference to the raw aggregation value.
    */
    pub fn aggs_raw(&self) -> Option<&Value> {
        self.aggregations.as_ref().map(|wrapper| &wrapper.0)
    }
}

impl<T: DeserializeOwned> IsOkOnSuccess for SearchResponse<T> {}

/** A borrowing iterator over search query hits. */
pub struct Hits<'a, T: 'a> {
    inner: Iter<'a, Hit<T>>,
}

impl<'a, T: 'a> Hits<'a, T> {
    fn new(hits: &'a HitsWrapper<T>) -> Self {
        Hits {
            inner: hits.inner.iter(),
        }
    }
}

impl<'a, T: 'a> Iterator for Hits<'a, T> {
    type Item = &'a Hit<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/** A consuminig iterator over search query hits. */
pub struct IntoHits<T> {
    inner: IntoIter<Hit<T>>,
}

impl<T> IntoHits<T> {
    fn new(hits: HitsWrapper<T>) -> Self {
        IntoHits {
            inner: hits.inner.into_iter(),
        }
    }
}

impl<T> Iterator for IntoHits<T> {
    type Item = Hit<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/** A borrowing iterator over the source documents in search query hits. */
pub struct Documents<'a, T: 'a> {
    inner: Iter<'a, Hit<T>>,
}

impl<'a, T: 'a> Documents<'a, T> {
    fn new(hits: &'a HitsWrapper<T>) -> Self {
        Documents {
            inner: hits.inner.iter(),
        }
    }
}

impl<'a, T: 'a> Iterator for Documents<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|hit| hit.source.as_ref())
    }
}

/** A consuming iterator over the source documents in search query hits. */
pub struct IntoDocuments<T> {
    inner: IntoIter<Hit<T>>,
}

impl<T> IntoDocuments<T> {
    fn new(hits: HitsWrapper<T>) -> Self {
        IntoDocuments {
            inner: hits.inner.into_iter(),
        }
    }
}

impl<T> Iterator for IntoDocuments<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().and_then(|hit| hit.source)
    }
}

/** Full metadata and source for a single hit. */
#[derive(Deserialize, Debug)]
pub struct Hit<T> {
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_type")]
    ty: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_version")]
    version: Option<u32>,
    #[serde(rename = "_score")]
    score: Option<f32>,
    #[serde(rename = "_source")]
    source: Option<T>,
    #[serde(rename = "_routing")]
    routing: Option<String>,
    highlight: Option<Value>,
}

impl<T> Hit<T> {
    /** Get a reference to the source document. */
    pub fn document(&self) -> Option<&T> {
        self.source.as_ref()
    }

    /** Convert the hit into the source document. */
    pub fn into_document(self) -> Option<T> {
        self.source
    }

    /** The index for the hit. */
    pub fn index(&self) -> &str {
        &self.index
    }

    /** The type of the hit. */
    pub fn ty(&self) -> &str {
        &self.ty
    }

    /** The id of the hit. */
    pub fn id(&self) -> &str {
        &self.id
    }

    /** The version of the hit. */
    pub fn version(&self) -> Option<u32> {
        self.version.clone()
    }

    /** The score of the hit. */
    pub fn score(&self) -> Option<f32> {
        self.score.clone()
    }

    /**
    A reference to the [highlighted] snippets of the part(s) of the field(s)
    matching the search query.

    [highlighted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    */
    pub fn highlight(&self) -> Option<&Value> {
        self.highlight.as_ref()
    }

    /**
    A clone of the [highlighted] snippets of the part(s) of the field(s)
    matching the search query.

    [highlighted]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-request-highlighting.html
    */
    pub fn into_highlight(&self) -> Option<Value> {
        self.highlight.clone()
    }
}

/** Type Struct to hold a generic `serde_json::Value` tree of the aggregation results. */
#[derive(Deserialize, Debug)]
struct AggsWrapper(Value);

/**
Aggregator that traverses the results from Elasticsearch's aggregations and returns a result row by row in a table-styled fashion.
*/
#[derive(Debug)]
pub struct Aggs<'a> {
    current_row: Option<RowData<'a>>,
    current_row_finished: bool,
    iter_stack: Vec<(&'a str, Iter<'a, Value>)>,
}

impl<'a> Aggs<'a> {
    fn new(aggregations: Option<&'a AggsWrapper>) -> Aggs<'a> {
        let iter_stack = {
            match aggregations.and_then(|aggs| aggs.0.as_object()) {
                Some(o) => o
                    .into_iter()
                    .filter_map(|(key, child)| {
                        child
                            .as_object()
                            .and_then(|child| child.get("buckets"))
                            .and_then(Value::as_array)
                            .map(|array| (key.as_ref(), array.iter()))
                    })
                    .collect(),
                None => Vec::new(),
            }
        };

        Aggs {
            current_row: None,
            current_row_finished: false,
            iter_stack: iter_stack,
        }
    }
}

type Object = Map<String, Value>;
type RowData<'a> = BTreeMap<Cow<'a, str>, &'a Value>;

fn insert_value<'a>(
    fieldname: &str,
    json_object: &'a Object,
    keyname: &str,
    rowdata: &mut RowData<'a>,
) {
    if let Some(v) = json_object.get(fieldname) {
        let field_name = format!("{}_{}", keyname, fieldname);
        rowdata.insert(Cow::Owned(field_name), v);
    }
}

impl<'a> Iterator for Aggs<'a> {
    type Item = RowData<'a>;

    fn next(&mut self) -> Option<RowData<'a>> {
        if self.current_row.is_none() {
            // New row
            self.current_row = Some(BTreeMap::new())
        }

        loop {
            if let Some((active_name, mut array)) = self.iter_stack.pop() {
                let n = array.next();

                // Iterate down?
                let mut has_buckets = false;
                // Save
                self.iter_stack.push((active_name, array));

                // FIXME: Move this, to be able to process first line too
                if let Some(n) = n {
                    if let Some(ref mut row) = self.current_row {
                        for (key, value) in n.as_object().expect("Shouldn't get here!") {
                            if let Some(c) = value.as_object() {
                                // Child Aggregation
                                if let Some(buckets) = c.get("buckets") {
                                    has_buckets = true;
                                    if let Value::Array(ref a) = *buckets {
                                        self.iter_stack.push((key, a.iter()));
                                    }
                                    continue;
                                }
                                // Simple Value Aggregation Name
                                if let Some(v) = c.get("value") {
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
                                    if let Some(child_values) =
                                        c.get("std_deviation_bounds").unwrap().as_object()
                                    {
                                        let u = child_values.get("upper");
                                        let l = child_values.get("lower");
                                        let un = format!("{}_std_deviation_bounds_upper", key);
                                        let ln = format!("{}_std_deviation_bounds_lower", key);
                                        row.insert(Cow::Owned(un), u.unwrap());
                                        row.insert(Cow::Owned(ln), l.unwrap());
                                    }
                                }
                            }

                            if key == "key" {
                                // Bucket Aggregation Name
                                row.insert(Cow::Borrowed(active_name), value);
                            } else if key == "doc_count" {
                                // Bucket Aggregation Count
                                let field_name = format!("{}_doc_count", active_name);
                                row.insert(Cow::Owned(field_name), value);
                            }
                        }
                    }
                } else {
                    // Was nothing here, exit
                    self.iter_stack.pop();
                    continue;
                }

                if !has_buckets {
                    break;
                }
            } else {
                self.current_row = None;
                break;
            };
        }

        self.current_row.take()
    }
}
