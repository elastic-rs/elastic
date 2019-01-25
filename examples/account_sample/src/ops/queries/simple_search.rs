use ops::Client;
use elastic::prelude::*;
use elastic::error::Result;

use model;
use model::account::Account;

pub trait SimpleSearchQuery {
    fn simple_search_query(&self, qry: &str) -> Result<SearchResponse<Account>>;
}

impl SimpleSearchQuery for Client {
    fn simple_search_query(&self, qry: &str) -> Result<SearchResponse<Account>> {
        let query = json!({
          "query": {
            "function_score": {
              "query": {
                "bool": {
                  "should": [
                    {
                      "query_string": {
                        "query": qry
                      }
                    },
                    {
                      "term": {
                        "employer": { 
                          "value": qry,
                          "boost": 1.3
                        }
                      }
                    }
                  ]
                }
              },
              "functions": [
                {
                  "gauss": {
                    "balance": {
                      "origin": 30000,
                      "scale": 10000
                    }
                  }
                },
                {
                  "gauss": {
                    "age": {
                      "origin": 35,
                      "scale": 5
                    }
                  }
                }
              ]
            }
          }
        });

        self.io
            .search()
            .index(model::index::name())
            .ty(Some(model::account::name()))
            .body(query.to_string())
            .send()
    }
}
