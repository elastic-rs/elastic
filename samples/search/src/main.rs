//! Elasticsearch Reqwest Client Integration Example
//!
//! This sample assumes you have a node running on `localhost`.

#[macro_use]
extern crate json_str;
extern crate elastic_reqwest;
extern crate elastic_responses;

use elastic_reqwest::{ElasticClient};
use elastic_reqwest::req::SearchRequest;
use elastic_responses::{parse, SearchResponse};

fn main() {

    let (client, params) = elastic_reqwest::default().unwrap();

    let body = json_str!({
      "size": 10,
      "query": {
        "bool": {
          "must": [
            {
              "query_string": {
                "analyze_wildcard": true,
                "query": "*"
              }
            },
            {
              "range": {
                "@timestamp": {
                  "gte": "now-15m",
                  "lte": "now",
                  "format": "epoch_millis"
                }
              }
            }
          ],
          "must_not": []
        }
      },
      "_source": {
        "excludes": []
      },
      "aggs": {
        "type": {
          "terms": {
            "field": "_type",
            "size": 5,
            "order": {
              "_count": "desc"
            }
          },
          "aggs": {
            "index": {
              "terms": {
                "field": "_index",
                "size": 5,
                "order": {
                  "_count": "desc"
                }
              }
            }
          }
        }
      }
    });

    // Send the request and read the response.
    let res = client.elastic_req(&params, SearchRequest::for_index("_all", body)).unwrap();

    //Parse body to JSON
    let body_as_json: SearchResponse = parse().from_reader(res.status().to_u16(), res).unwrap();

    //Use hits() or aggs() iterators
    //Hits
    for i in body_as_json.hits() {
        println!("{:?}",i);
    }

    //Agregations
    for i in body_as_json.aggs() {
        println!("{:?}",i);
    }
}
