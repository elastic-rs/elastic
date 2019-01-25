extern crate elastic_queries;
extern crate serde_json;

use elastic_queries::prelude::*;

#[test]
fn aggregation_search() {
    let j = include_str!("complex.json");

    let _s: Query = serde_json::from_str(j).unwrap();
}

#[test]
fn simple_aggregation_search() {
    let j = include_str!("simpleagg.json");

    let _s: Query = serde_json::from_str(j).unwrap();
}

#[test]
fn nested_aggregation_search() {
    let j = include_str!("nested.json");

    let _s: Query = serde_json::from_str(j).unwrap();
}
