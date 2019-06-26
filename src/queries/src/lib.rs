#![feature(custom_attribute)]
#![feature(iterator_for_each)]
#![recursion_limit = "256"]

#[macro_use]
extern crate derive_builder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[marco_use]
extern crate serde_json;
//extern crate error_chain;

mod aggregations;
mod filters;
pub mod prelude;

use aggregations::{
    Aggregation,
    BucketAggregation,
    EsAggregation,
};
use filters::Filters;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
pub enum Values {
    String(String),
    Number(i64),
    Bool(bool),
}

impl From<i64> for Values {
    fn from(int: i64) -> Self {
        Values::Number(int)
    }
}

pub enum BoolQuerySections {
    Must,
    Should,
    Filter,
    MustNot,
}

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct Bool {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub must: Option<Vec<Filters>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub should: Option<Vec<Filters>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub filter: Option<Vec<Filters>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub must_not: Option<Vec<Filters>>,
}

impl Bool {
    fn add_filter(&mut self, section: BoolQuerySections, f: Filters) {
        use BoolQuerySections::*;

        match section {
            Must => {
                if let Some(ref mut ve) = self.must {
                    if let Err(i) = ve.binary_search(&f) {
                        ve.insert(i, f);
                    }
                } else {
                    self.must = Some(vec![f]);
                }
            }
            Should => {
                if let Some(ref mut ve) = self.should {
                    if let Err(i) = ve.binary_search(&f) {
                        ve.insert(i, f);
                    }
                } else {
                    self.should = Some(vec![f]);
                }
            }
            Filter => {
                if let Some(ref mut ve) = self.filter {
                    if let Err(i) = ve.binary_search(&f) {
                        ve.insert(i, f);
                    }
                } else {
                    self.filter = Some(vec![f]);
                }
            }
            MustNot => {
                if let Some(ref mut ve) = self.must_not {
                    if let Err(i) = ve.binary_search(&f) {
                        ve.insert(i, f);
                    }
                } else {
                    self.must_not = Some(vec![f]);
                }
            }
        }
    }

    fn remove_filter(&mut self, section: BoolQuerySections, f: Filters) {
        use BoolQuerySections::*;

        match section {
            Must => {
                if let Some(ref mut ve) = self.must {
                    if let Ok(i) = ve.binary_search(&f) {
                        ve.remove(i);
                    }
                } else {
                    self.must = Some(vec![f]);
                }
            }
            Should => {
                if let Some(ref mut ve) = self.should {
                    if let Ok(i) = ve.binary_search(&f) {
                        ve.remove(i);
                    }
                } else {
                    self.should = Some(vec![f]);
                }
            }
            Filter => {
                if let Some(ref mut ve) = self.filter {
                    if let Ok(i) = ve.binary_search(&f) {
                        ve.remove(i);
                    }
                } else {
                    self.filter = Some(vec![f]);
                }
            }
            MustNot => {
                if let Some(ref mut ve) = self.must_not {
                    if let Ok(i) = ve.binary_search(&f) {
                        ve.remove(i);
                    }
                } else {
                    self.must_not = Some(vec![f]);
                }
            }
        }
    }
}

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct QueryField {
    #[builder(default = "self.default_bool()?")]
    pub bool: Bool,
}

impl QueryFieldBuilder {
    fn default_bool(&self) -> Result<Bool, String> {
        BoolBuilder::default().build()
    }
}

#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct Query {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "self.default_query()?")]
    pub query: Option<QueryField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    pub aggs: Option<HashMap<String, Aggregation>>,
}

impl Query {
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn add_filter(&mut self, section: BoolQuerySections, f: Filters) {
        if let Some(ref mut query) = self.query {
            query.bool.add_filter(section, f);
        }
    }

    fn remove_filter(&mut self, section: BoolQuerySections, f: Filters) {
        if let Some(ref mut query) = self.query {
            query.bool.remove_filter(section, f);
        }
    }

    fn aggs_up_to_target(&mut self, target: &str) -> Option<(&str, Aggregation)> {
        if let Some(root_aggs) = self.aggs_mut() {
            for (rootname, aggregations) in root_aggs.iter_mut() {
                aggregations.aggs_fn(&|n, a| {
                    if n == target {
                        a.aggs_clear();
                    }
                });
                return Some((rootname, aggregations.clone()));
            }
        }

        None
    }

    fn aggs_target<'a>(&mut self, target: &'a str) -> Option<(&'a str, Aggregation)> {
        if let Some(root_aggs) = self.aggs_mut() {
            let mut found = None;

            for (_, aggregations) in root_aggs.iter_mut() {
                match found {
                    None => match aggregations.aggs_get(target).map(|a| a) {
                        Some(out) => {
                            found = Some((target, out.clone()));
                        }
                        None => {
                            found = None;
                        }
                    },
                    Some(_) => break,
                }
            }
            return found;
        }

        None
    }

    fn insert_child_after(&mut self, target: &str, child: &str, agg: Aggregation) {
        self.aggs = self.insert_child_after_internal(target, child, agg);
    }

    /// Add new Aggregation shifting current Aggregations down.
    fn insert_child_after_internal(
        &mut self,
        target: &str,
        child: &str,
        agg: Aggregation,
    ) -> Option<aggregations::EsAggregation> {
        if let Some(root_aggs) = self.aggs_mut() {
            let mut returnable = HashMap::new();

            for (rootname, aggregations) in root_aggs.iter_mut() {
                // Copy the root and clear it from the target onwards
                let mut root = aggregations.clone();
                root.aggs_fn(&|n, a| {
                    if n == target {
                        a.aggs_clear();
                    }
                });

                //Find the target and store it
                let out = match aggregations.aggs_get(target).map(|a| a.aggs_mut()) {
                    Some(a) => a,
                    None => None,
                };

                //Add the new element
                if let Some(target) = root.aggs_get(target) {
                    target.add_child(child, agg.clone());
                }

                //Add the stored sub-children back
                if let Some(newchild) = root.aggs_get(child) {
                    match out {
                        Some(a) => {
                            for (name, key) in a {
                                newchild.add_child(name, key.clone());
                            }
                        }
                        None => (),
                    };
                }
                returnable.insert(rootname.clone(), root);
            }

            return Some(returnable);
        }
        None
    }

    fn replace_target_agg(&mut self, target: &str, child: &str, agg: Aggregation) {
        self.insert_child_after(target, child, agg);
        self.drop_target_agg(target);
    }

    /// Add new Aggregation shifting current Aggregations down.
    fn drop_target_agg(&mut self, target: &str) {
        if let Some(root_aggs) = self.aggs_mut() {
            for (rootname, aggregations) in root_aggs.iter_mut() {
                aggregations.aggs_fn(&|name, agg| {
                    if agg.has_child(target) {
                        let mut backup = agg.clone();
                        let grand_c = backup.aggs_get(target).unwrap().aggs();
                        agg.aggs_clear();
                        if let Some(c) = grand_c {
                            agg.set_aggs(Some(c.clone()));
                        }
                    }
                });
            }
        }
    }
}

impl BucketAggregation for Query {
    fn aggs(&self) -> Option<&aggregations::EsAggregation> {
        self.aggs.as_ref()
    }

    fn aggs_mut(&mut self) -> Option<&mut aggregations::EsAggregation> {
        self.aggs.as_mut()
    }

    fn aggs_clear(&mut self) {
        self.aggs = None;
    }

    fn aggs_init(&mut self) {
        self.aggs = Some(HashMap::new());
    }

    fn set_aggs(&mut self, replacement: Option<EsAggregation>) {
        self.aggs = replacement;
    }
}

impl QueryBuilder {
    fn default_query(&self) -> Result<Option<QueryField>, String> {
        Ok(Some(QueryFieldBuilder::default().build()?))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        filters::*,
        *,
    };

    #[test]
    fn root_to_target() {
        //Test insert
        let j = include_str!("../tests/nested.json");
        let mut s1: Query = serde_json::from_str(j).unwrap();

        let mut agg = s1.aggs_up_to_target("Agg2Terms").unwrap();

        use aggregations::terms::*;

        let j = r#"{
                      "terms": {
                        "field": "AggNew",
                        "size": 10000,
                        "order": {
                          "_term": "asc"
                        }
                      }
                    }"#;
        let t: TermAggregation = serde_json::from_str(j).unwrap();

        agg.1
            .add_child_to_target("Agg2Terms", "AggNew", Aggregation::term(t.clone()));
        assert_eq!(agg.1.aggs_get("AggNew").is_some(), true);

        agg.1
            .add_child_to_target("AggNew", "AggNew2", Aggregation::term(t.clone()));

        assert!(agg.1.aggs_get("AggNew2").is_some());
        agg.1
            .add_child_to_target("AggNew2", "AggNew3", Aggregation::term(t));

        assert!(agg.1.aggs_get("AggNew3").is_some());
    }

    #[test]
    fn target() {
        //Test insert
        let j = include_str!("../tests/nested.json");
        let mut s1: Query = serde_json::from_str(j).unwrap();

        let _agg = s1.aggs_target("Agg2Terms").unwrap();
    }

    #[test]
    fn builder() {
        let bo = BoolBuilder::default()
            .build()
            .expect("could not build bool");
        let qb = QueryFieldBuilder::default()
            .build()
            .expect("could not build queryfield");
        let mut q = QueryBuilder::default()
            .build()
            .expect("could not build query");

        q.add_filter(
            BoolQuerySections::Must,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(1))),
        );
        q.add_filter(
            BoolQuerySections::MustNot,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(1))),
        );
        q.add_filter(
            BoolQuerySections::Must,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(1))),
        );
        q.add_filter(
            BoolQuerySections::MustNot,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(1))),
        );
        q.add_filter(
            BoolQuerySections::MustNot,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(2))),
        );
        q.remove_filter(
            BoolQuerySections::MustNot,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(2))),
        );

        let j = serde_json::to_string(&q).unwrap();
        let expected =
            r#"{"query":{"bool":{"must":[{"term":{"foo":1}}],"must_not":[{"term":{"foo":1}}]}}}"#;
        assert_eq!(expected, j);
    }

    #[test]
    fn filter() {
        let j = r#"
            {
              "query": {
                "bool": {
                  "must": [
                    { "match": { "title":   "Search"        }},
                    { "match": { "content": "Elasticsearch" }}
                  ],
                  "filter": [
                    { "term":  { "status": "published" }},
                    { "range": { "publish_date": { "gte": "2015-01-01" }}}
                  ]
                }
              }
            }
            "#;

        let mut s: Query = super::serde_json::from_str(j).unwrap();
        s.add_filter(
            BoolQuerySections::Must,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(1))),
        );
        s.add_filter(
            BoolQuerySections::MustNot,
            Filters::term(TermFilter::new(String::from("foo"), Values::Number(1))),
        );
    }

    #[test]
    fn simple_queries_parse() {
        let j = r#"
            {
              "query": {
                "bool": {
                  "must": [
                    { "match": { "title":   "Search"        }},
                    { "match": { "content": "Elasticsearch" }}
                  ],
                  "filter": [
                    { "term":  { "status": "published" }},
                    { "range": { "publish_date": { "gte": "2015-01-01" }}}
                  ]
                }
              }
            }
            "#;
        let _s: Query = super::serde_json::from_str(j).unwrap();

        let simple = r#"
            {
              "size": 0,
              "query": {
                "bool": {
                  "must": [
                    { "wildcard" : { "user" : "ki*y" } },
                    { "term":  { "status": "published" }}
                  ],
                  "filter": [
                    { "range": { "publish_date": { "gte": "2015-01-01" }}}
                  ],
                  "should": [
                    { "exists": { "field": "sourceAddress" } }
                  ]
                }
              }
            }
            "#;
        let _s: Query = super::serde_json::from_str(simple).unwrap();
    }
}
