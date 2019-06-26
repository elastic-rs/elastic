pub(crate) mod date_histogram;
pub(crate) mod stats;
pub(crate) mod terms;

use self::{
    date_histogram::DateHistogramAggregation,
    stats::{
        AvgAggregation,
        MaxAggregation,
        SumAggregation,
    },
    terms::TermAggregation,
};
use std::{
    collections::{
        hash_map::Iter,
        HashMap,
    },
    option::Option,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types)]
pub enum Aggregation {
    date_histogram(DateHistogramAggregation),
    term(TermAggregation),
    avg(AvgAggregation),
    max(MaxAggregation),
    sum(SumAggregation),
}

pub type EsAggregation = HashMap<String, Aggregation>;

impl BucketAggregation for Aggregation {
    fn aggs_mut(&mut self) -> Option<&mut EsAggregation> {
        match *self {
            Aggregation::date_histogram(ref mut d) => d.aggs.as_mut(),
            Aggregation::term(ref mut t) => t.aggs.as_mut(),
            _ => None,
        }
    }

    fn aggs(&self) -> Option<&EsAggregation> {
        match *self {
            Aggregation::date_histogram(ref d) => d.aggs.as_ref(),
            Aggregation::term(ref t) => t.aggs.as_ref(),
            _ => None,
        }
    }

    fn aggs_clear(&mut self) {
        match *self {
            Aggregation::date_histogram(ref mut d) => d.aggs = None,
            Aggregation::term(ref mut t) => t.aggs = None,
            _ => (),
        }
    }

    fn aggs_init(&mut self) {
        match *self {
            Aggregation::date_histogram(ref mut d) => d.aggs = Some(HashMap::new()),
            Aggregation::term(ref mut t) => t.aggs = Some(HashMap::new()),
            _ => (),
        }
    }

    fn set_aggs(&mut self, replacement: Option<EsAggregation>) {
        match *self {
            Aggregation::date_histogram(ref mut d) => d.aggs = replacement,
            Aggregation::term(ref mut t) => t.aggs = replacement,
            _ => (),
        }
    }
}

pub trait BucketAggregation {
    /// Simple addition of child Aggregration
    fn add_child(&mut self, name: &str, agg: Aggregation) {
        if let None = self.aggs() {
            self.aggs_init();
        }

        if let Some(aggs) = self.aggs_mut() {
            aggs.insert(name.into(), agg);
        }
    }

    fn has_child(&self, target: &str) -> bool {
        if let Some(aggs) = self.aggs() {
            return aggs.contains_key(target);
        }
        false
    }

    fn set_aggs(&mut self, Option<EsAggregation>);

    fn add_child_to_target(&mut self, target: &str, name: &str, agg: Aggregation) {
        self.aggs_fn(&|n, a| {
            if n == target {
                a.add_child(name, agg.clone());
            }
        });
    }

    fn aggs(&self) -> Option<&EsAggregation>;

    fn aggs_mut(&mut self) -> Option<&mut EsAggregation>;

    fn aggs_init(&mut self);

    fn aggs_clear(&mut self);

    fn aggs_get_and_clear(&mut self) -> Option<EsAggregation> {
        if let Some(aggs) = self.aggs_mut() {
            let ret = aggs.clone();
            aggs.clear();
            return Some(ret);
        }
        None
    }

    fn aggs_get(&mut self, target: &str) -> Option<&mut Aggregation> {
        use Aggregation::*;
        let mut found = None;

        if let Some(a) = self.aggs_mut() {
            for (name, child) in a {
                match found {
                    None => {
                        if name == target {
                            return Some(child);
                        } else {
                            match *child {
                                date_histogram(ref mut agg) => {
                                    found = agg.aggs_get(target);
                                }
                                term(ref mut agg) => {
                                    found = agg.aggs_get(target);
                                }
                                _ => (),
                            }
                        };
                    }
                    Some(_) => break,
                }
            }
        }
        found
    }

    fn aggs_get_ref(&self, target: &str) -> Option<&Aggregation> {
        use Aggregation::*;
        let mut found = None;

        if let Some(a) = self.aggs() {
            for (name, child) in a {
                match found {
                    None => {
                        if name == target {
                            return Some(child);
                        } else {
                            match *child {
                                date_histogram(ref agg) => {
                                    found = agg.aggs_get_ref(target);
                                }
                                term(ref agg) => {
                                    found = agg.aggs_get_ref(target);
                                }
                                _ => (),
                            }
                        };
                    }
                    Some(_) => break,
                }
            }
        }
        found
    }

    fn aggs_fn<F>(&mut self, f: &F)
    where
        F: Fn(&str, &mut Aggregation),
    {
        use aggregations::BucketAggregation;
        use Aggregation::*;
        if let Some(a) = self.aggs_mut() {
            for (name, child) in a {
                f(name, child);
                match *child {
                    date_histogram(ref mut agg) => {
                        agg.aggs_fn(f);
                    }
                    term(ref mut agg) => {
                        agg.aggs_fn(f);
                    }
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct AggHolder<'i> {
    name: &'i String,
    agg: &'i Aggregation,
}

#[derive(Debug)]
pub struct AggregationIterator<'i> {
    pub iterator_stack: Vec<Iter<'i, String, Aggregation>>,
    pub current: Option<Iter<'i, String, Aggregation>>,
    pub end_of_iter: bool,
}

impl<'i> AggregationIterator<'i> {
    fn new(vi: Iter<'i, String, Aggregation>) -> AggregationIterator<'i> {
        AggregationIterator {
            iterator_stack: vec![vi],
            current: None,
            end_of_iter: false,
        }
    }
}

impl<'i> Iterator for AggregationIterator<'i> {
    type Item = AggHolder<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        use Aggregation::*;

        if !self.end_of_iter {
            //Peek and if None then don't add back
            if let Some(ref mut c) = self.current {
                //FIXME: Is this clone necesarry?
                self.iterator_stack.push(c.clone());
            };
        }

        let next = loop {
            //Set up top of queue
            self.current = self.iterator_stack.pop();

            let next = match self.current {
                Some(ref mut item) => {
                    self.end_of_iter = false;
                    item.next()
                }
                None => {
                    // Pop stack
                    self.end_of_iter = true;
                    None
                }
            };

            match next {
                None => {
                    // Check if the stack is empty and only return None in that case
                    if self.iterator_stack.len() == 0 {
                        break None;
                    };
                }
                Some(x) => break Some(x),
            }
        };

        if let Some(next) = next {
            match next.1 {
                &date_histogram(ref agg) => {
                    let a = agg.aggs();
                    if let Some(ref a) = a {
                        self.iterator_stack.push(a.iter());
                    }
                }
                &term(ref agg) => {
                    let a = agg.aggs();
                    if let Some(ref a) = a {
                        self.iterator_stack.push(a.iter());
                    }
                }
                _ => (),
            };

            return Some(AggHolder {
                name: next.0,
                agg: next.1,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::Query,
        *,
    };
    use serde_json;

    #[test]
    fn api_replace() {
        //Test insert
        let j = include_str!("../../tests/nested.json");
        let mut s1: Query = serde_json::from_str(j).unwrap();

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

        use aggregations::terms::*;

        s1.replace_target_agg("Agg3Terms", "AggNew", Aggregation::term(t));

        assert!(s1
            .aggs_get("Agg2Terms")
            .unwrap()
            .aggs_get("AggNew")
            .unwrap()
            .aggs_get("Agg4Terms")
            .unwrap()
            .aggs()
            .is_some());
    }

    #[test]
    fn api_drop_target() {
        //Test insert
        let j = include_str!("../../tests/nested.json");
        let mut s1: Query = serde_json::from_str(j).unwrap();

        use aggregations::terms::*;

        s1.drop_target_agg("Agg3Terms");

        assert!(s1
            .aggs_get("Agg2Terms")
            .unwrap()
            .aggs_get("Agg3Terms")
            .is_none());

        assert!(s1
            .aggs_get("Agg2Terms")
            .unwrap()
            .aggs_get("Agg4Terms")
            .unwrap()
            .aggs()
            .is_some());
    }

    #[test]
    fn api_find_and_insert() {
        //Test insert
        let j = include_str!("../../tests/nested.json");
        let mut s1: Query = serde_json::from_str(j).unwrap();

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

        use aggregations::terms::*;

        s1.insert_child_after("Agg2Terms", "AggNew", Aggregation::term(t));
        assert!(s1
            .aggs_get("Agg2Terms")
            .unwrap()
            .aggs_get("AggNew")
            .unwrap()
            .aggs_get("Agg3Terms")
            .unwrap()
            .aggs()
            .is_some());
    }

    #[test]
    fn find_and_insert_hard_way() {
        use aggregations::terms::TermsAggFields;
        use Aggregation::*;

        let j = include_str!("../../tests/complex.json");
        let mut s1: Query = serde_json::from_str(j).unwrap();
        let mut s2 = s1.clone();

        let mut a = s1.aggs_get("sourcePort").unwrap();

        //Destructuring FTW
        if let term(TermAggregation {
            terms: TermsAggFields {
                field: ref mut a, ..
            },
            ..
        }) = *a
        {
            assert_eq!(a, "sourcePort");
        }

        assert_eq!(s2.aggs_get("Foo").is_none(), true);

        //Test insert
        let j = include_str!("../../tests/nested.json");
        let mut s1: Query = serde_json::from_str(j).unwrap();

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

        let a = s1.aggs_get("Agg3Terms");
        if let Some(a) = a {
            if let term(TermAggregation {
                terms: TermsAggFields { field: ref a, .. },
                ..
            }) = *a
            {
                assert_eq!(a, "Agg3Terms");
            }

            if let term(ref mut a) = *a {
                assert!(a.aggs_get("Agg4Terms").is_some());

                let mut backup = match a.aggs_get("Agg4Terms").map(|a| a.clone()).unwrap() {
                    term(b) => b,
                    _ => panic!("shouldn't get here"),
                };

                backup.add_child("Agg4Terms", Aggregation::term(t));

                a.aggs_clear();

                assert!(a.aggs_get("Agg4Terms").is_none());

                a.add_child("AggNew", Aggregation::term(backup));

                assert!(a.aggs_get("AggNew").is_some());

                assert!(a.aggs_get("Agg4Terms").is_some());
            }
        }
    }

    #[test]
    fn simple_closure() {
        let j = include_str!("../../tests/complex.json");
        let mut s: Query = serde_json::from_str(j).unwrap();

        s.aggs_fn(&|s, a| ());
    }

    #[test]
    fn iterator_immut() {
        let j = include_str!("../../tests/complex.json");
        let mut s: Query = serde_json::from_str(j).unwrap();
        let mut a = s.aggs();

        let mut a_ref = a.as_ref().unwrap().iter();
        let mut i = AggregationIterator::new(a_ref);
        for item in i {
            ()
        }
    }

    #[test]
    fn iterator_step_by_step() {
        let j = include_str!("../../tests/complex.json");
        let mut s: Query = serde_json::from_str(j).unwrap();

        //Get the aggs of the query
        let mut a = s.aggs_mut();

        // Clear out the children from root
        if let Some(aggs) = a.as_mut() {
            let i = aggs.iter_mut().next();
            let first = i.unwrap();
            let (_, mut val) = first;
            use Aggregation::*;
            match val {
                &mut date_histogram(ref mut agg) => {
                    agg.aggs_clear();
                    assert_eq!(agg.aggs().is_none(), true);
                }
                _ => (),
            }
        } else {
            panic!("no match, shouldn't get here");
        }
    }

    #[test]
    fn iterator_adaptors() {
        let j = include_str!("../../tests/complex.json");
        let mut s: Query = serde_json::from_str(j).unwrap();
        let mut a = s.aggs();

        let mut a_ref = a.as_ref().unwrap().iter();
        let mut i = AggregationIterator::new(a_ref);

        i.for_each(|s| ());
    }

    #[test]
    fn iterator_add() {
        let j = include_str!("../../tests/complex.json");
        let s: Query = serde_json::from_str(j).unwrap();
        let a = s.aggs();

        let a_ref = a.as_ref().unwrap().iter();
        let mut i = AggregationIterator::new(a_ref);

        let a = i.next().unwrap();

        let j = r#"{
                      "terms": {
                        "field": "flowInputInterface",
                        "size": 10000,
                        "order": {
                          "_term": "asc"
                        }
                      }
                    }"#;
        let s: TermAggregation = serde_json::from_str(j).unwrap();

        let mut a = a.agg.clone();

        use Aggregation::*;

        match a {
            date_histogram(ref mut agg) => {
                let mut aggs = agg.aggs_mut();
                let x = aggs.as_mut().unwrap();
                x.insert("foo".into(), Aggregation::term(s));
            }
            _ => {
                panic!("broken...");
            }
        };

        let z = a.clone();

        match a {
            date_histogram(ref mut agg) => {
                let mut aggs = agg.aggs_mut();
                let x = aggs.as_mut().unwrap();
                x.insert("foo2".into(), z);
            }
            _ => {
                panic!("broken...");
            }
        };
    }

    #[test]
    fn aggregation_insert() {
        let j = include_str!("../../tests/complex.json");
        let s: Query = serde_json::from_str(j).unwrap();
        let a = s.aggs();

        let a_ref = a.as_ref().unwrap().iter();
        let mut i = AggregationIterator::new(a_ref);

        let a = i.next().unwrap();

        let j = r#"{
                      "terms": {
                        "field": "flowInputInterface",
                        "size": 10000,
                        "order": {
                          "_term": "asc"
                        }
                      }
                    }"#;
        let s: TermAggregation = serde_json::from_str(j).unwrap();

        let mut a = a.agg.clone();

        use Aggregation::*;

        let z = a.clone();

        match a {
            date_histogram(ref mut agg) => {
                let mut aggs = agg.aggs_mut();
                let x = aggs.as_mut().unwrap();
                x.clear();
                x.insert("foo2".into(), z);
            }
            _ => {
                panic!("broken...");
            }
        };

        use aggregations::BucketAggregation;
        match a {
            Aggregation::date_histogram(ref mut a) => {
                assert!(a.aggs_get("foo2").is_some());
            }
            _ => panic!("broken..."),
        }
    }
}
