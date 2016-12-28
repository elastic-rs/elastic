extern crate tera;
extern crate serde_json;
extern crate dataflow;

use self::serde_json::Value;
use std::collections::BTreeMap;
use self::dataflow::HotrodPCollectionMap;
use self::dataflow::HotrodPCollectionArray;
use self::dataflow::table_from_string;
//use std::iter::IntoIterator::into_iter;

#[derive(Debug)]
enum EsAggregationNodes<'a> {
    Container(&'a Value),
    Buckets(&'a BTreeMap<String, Value>)
    //    ValueNode(&'a Value),
    //    StatsNode(&'a Value)
}

//enum EsRowValues<'a> {
//    None,
//    Values(&'a BTreeMap<&'a String, &'a Value>)
//}

#[derive(Debug)]
pub struct EsResultStore {
    result: Value
}

impl EsResultStore {
    pub fn new(rawresponse: String) -> EsResultStore {
        let data: Value = serde_json::from_str(&rawresponse).unwrap();

        EsResultStore {
            result: data
        }
    }

    pub fn pretty_results(&self) -> String {
        let serialized = serde_json::to_string_pretty(&self.result).unwrap();
        serialized
    }

    fn aggregations(&self) -> &serde_json::Value {

        let obj = &self.result.as_object().unwrap();

        //        let obj = match self.result {
        //            Value::Object(o) => o,
        //            _ => {
        //                panic!("Couldn't find root object");
        //            }
        //        };

        let aggs = obj.get("aggregations").unwrap();

        aggs
    }

    fn hits(&self) -> &serde_json::Value {
        let obj = &self.result.as_object().unwrap();
        let hits = obj.get("hits").unwrap();

        let obj = hits.as_object().unwrap();
        let hits = obj.get("hits").unwrap();

        hits
    }

    fn walk_tree<'a>(&'a self, table: &mut HotrodPCollectionArray, node: &EsAggregationNodes, parent_name: &str) {
        match *node {
            EsAggregationNodes::Container(node) => {
                let mut row_values = match table.pop() {
                    Some(x) => {
                        x
                    },
                    None => {
                        HotrodPCollectionMap::new()
                    }
                };

                let as_btreemap = match node.as_object() {
                    Some(o) => o,
                    None => panic!("Wasn't an object")
                };

                let node_name: &str;
                let has_buckets = false;

                if as_btreemap.contains_key("key") {
                    let node_name_v = as_btreemap.get("key").unwrap();
                    if node_name_v.is_number() {
                        let node_name_n = &node_name_v.as_i64().unwrap();
                        let v = Value::String(node_name_n.to_string());
                        let k = String::from(parent_name);
                        row_values.insert(k, v);
                    } else if node_name_v.is_string() {
                        node_name = node_name_v.as_str().unwrap();
                        let v = Value::String(String::from(node_name));
                        let k = String::from(parent_name);
                        row_values.insert(k, v);
                    }
                }

                for (key, value) in as_btreemap.iter() {
                    match *value {
                        Value::Object(ref o) => {
                            if o.contains_key("value") {
                                row_values.insert(key.clone(), o["value"].clone());
                            }
                            if o.contains_key("avg") && o.contains_key("count") &&
                                o.contains_key("max") && o.contains_key("min") &&
                                o.contains_key("sum") {
                                row_values.insert(format!("{}_avg", key.clone()), o["avg"].clone());
                                row_values.insert(format!("{}_count", key.clone()), o["count"].clone());
                                row_values.insert(format!("{}_max", key.clone()), o["max"].clone());
                                row_values.insert(format!("{}_min", key.clone()), o["min"].clone());
                                row_values.insert(format!("{}_sum", key.clone()), o["sum"].clone());
                            }
                            if o.contains_key("buckets") {
                                let x = EsAggregationNodes::Buckets(o);
                                table.push(row_values);
                                self.walk_tree(table, &x, key);
                                return;
                            }
                        },
                        _ => {}
                    };
                }
                if !has_buckets {
                    table.push(row_values);
                } else {
                    println!("Done, pushing row");
                }
            },
            EsAggregationNodes::Buckets(node) => {
                let row_values = match table.pop() {
                    Some(last_row) => {
                        last_row
                    },
                    None => {
                        HotrodPCollectionMap::new()
                    }
                };

                for b in node.get("buckets").unwrap().as_array().unwrap() {
                    let x = EsAggregationNodes::Container(b);

                    let fork_values = row_values.clone();
                    table.push(fork_values);

                    self.walk_tree(table, &x, parent_name);
                }
            },
            //            ref x => {
            //                panic!("Not implemented yet: {:?}", x);
            //            }
        }
    }

    pub fn get_table_from_aggs(&self) -> HotrodPCollectionArray {
        let mut table = HotrodPCollectionArray::new();

        let rootnode = EsAggregationNodes::Container(&self.aggregations());
        let s = "";

        self.walk_tree(&mut table, &rootnode, &s);
        table
    }

    pub fn get_table_from_hits(self) -> HotrodPCollectionArray {
        //        let data_array = match *self.hits() {
        //            Value::Array(a) => a,
        //            _ => Vec::new()
        //        };
        let mut table = HotrodPCollectionArray::new();
        let data_array = self.hits().as_array().unwrap();

        //        let return_data : HotrodPCollectionArray = data_array.into_iter().map(
        //            move |x| {
        //                match x {
        //                    Value::Object(o) => o,
        //                    _ => BTreeMap::new()
        //                }
        //            }
        //        ).collect();

        for e in data_array {
            table.push(e.as_object().unwrap().clone());
        }

        table
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use resultparser::EsResultStore;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn interface_test() {
        let mut f = File::open("tests/aggregation.json").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let r = EsResultStore::new(s);
        assert!(r.result.is_object());
    }

    #[test]
    fn hits_test() {
        let mut f = File::open("tests/hits.json").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let r = EsResultStore::new(s);
        assert!(r.result.is_object());
    }

    #[test]
    fn hits_to_table_test() {
        let mut f = File::open("tests/hits.json").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let r = EsResultStore::new(s);
        let t = r.get_table_from_hits();
        assert!(t.len() > 1);
    }

    #[test]
    fn aggs_test() {
        let mut f = File::open("tests/aggregation.json").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let r = EsResultStore::new(s);
        let t = r.get_table_from_aggs();
        assert!(t.len() > 1);
    }

    #[test]
    fn large_aggs_test() {
        let mut f = File::open("tests/largeagg.json").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let r = EsResultStore::new(s);

        let t = r.get_table_from_aggs();
        let serialized = serde_json::to_string_pretty(&t).unwrap();

        let mut f = File::create("tests/largeresult.json").unwrap();
        f.write_all(serialized.as_bytes()).unwrap();
        f.sync_all().unwrap();
    }
}

