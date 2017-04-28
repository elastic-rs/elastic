use ops::Client;
use elastic::client::into_response;
use elastic::client::requests::{SearchRequest, IntoBody};
use elastic::client::responses::SearchResponse;
use elastic::error::Result;

use model;
use model::account::Account;

pub trait SimpleSearchQuery {
    fn simple_search_query(&self, qry: &str) -> Result<SearchResponse<Account>>;
}

impl SimpleSearchQuery for Client {
    fn simple_search_query(&self, qry: &str) -> Result<SearchResponse<Account>> {
        let qry = format!("\"{}\"", qry);
        let req = search(search_body(&qry));
        
        self.io.search().request(req).send()
    }
}

fn search<B>(body: B) -> SearchRequest<'static, B>
    where B: IntoBody
{
    SearchRequest::for_index_ty(model::index::name(), model::account::name(), body)
}

fn search_body(qry: &str) -> String {
    let get_body = json_fn!(|qry| {
      "query": {
        "function_score": {
          "query": {
            "bool": {
              "should": [
                {
                  "query_string": {
                    "query": $qry
                  }
                },
                {
                  "term": {
                    "employer": { 
                      "value": $qry,
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

    get_body(qry)
}