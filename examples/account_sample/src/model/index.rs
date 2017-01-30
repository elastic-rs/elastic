//! Commands for managing the bank index.
//! 
//! The public API for this module just gives us a few REST API requests
//! for checking whether the bank index exists and creating it along with
//! analysers, filters and mapping for `Account`s.

use elastic::client::requests::{Index, IndicesExistsRequest, IndicesCreateRequest};
use elastic::types::prelude::{Document, FieldType};
use serde_json;
use super::account::{self, Account};

/// Get the name of the bank index.
pub fn name() -> Index<'static> {
    "bank-sample".into()
}

/// Get a request to check if the bank index exists.
pub fn exists() -> IndicesExistsRequest<'static> {
    IndicesExistsRequest::for_index(name())
}

/// Get a request to create the bank index.
pub fn put() -> IndicesCreateRequest<'static> {
    IndicesCreateRequest::for_index(name(), bank_index())
}

fn bank_index() -> String {
    let account_name = format!("\"{}\"", account::name());
    let account_mapping = serde_json::to_string(&Document::from(Account::mapping())).expect("get Account mapping");
    let filters = bank_filters();
    let analysers = bank_analysers();

    let get_index = json_fn!(|filters, analysers, account_name, account_mapping| {
       "settings" : {
          "analysis" : {
                "filter" : $filters,
                "analyzer" : $analysers
          },
          "mappings": {
            $account_name: $account_mapping
          }
       }
    });
    
    get_index(&filters, &analysers, &account_name, &account_mapping)
}

fn bank_filters() -> String {
    json_str!({
        "email": {
            "type": "pattern_capture",
            "preserve_original": 1,
            "patterns": [
                "([^@]+)",
                "(\\p{L}+)",
                "(\\d+)",
                "@(.+)"
            ]
        }
    })
}

fn bank_analysers() -> String {
    json_str!({
        "email": {
            "tokenizer": "uax_url_email",
            "filter": [
                "email",
                "lowercase",
                "unique"
            ]
        }
    })
}

#[cfg(test)]
mod tests {
    use std::str;
    use elastic::client::requests::RawBody;
    use super::*;

    #[test]
    fn put_request_url() {
        let req = put();

        assert_eq!("/bank-sample", req.url.as_ref());
    }

    #[test]
    fn put_request_body() {
        let req = put();

        let body = req.body.into_raw();
        let body = str::from_utf8(&body).unwrap();

        let expected = json_str!({
            "settings":{
                "analysis":{
                    "filter":{
                        "email":{
                            "type":"pattern_capture",
                            "preserve_original":1,
                            "patterns":[
                                "([^@]+)",
                                "(\\p{L}+)",
                                "(\\d+)",
                                "@(.+)"
                            ]
                        }
                    },
                    "analyzer":{
                        "email":{
                            "tokenizer":"uax_url_email",
                            "filter":[
                                "email",
                                "lowercase",
                                "unique"
                            ]
                        }
                    }
                },
                "mappings":{
                    "account":{
                        "properties":{
                            "account_number":{
                                "type":"integer"
                            },
                            "balance":{
                                "type":"integer"
                            },
                            "firstname":{
                                "type":"keyword"
                            },
                            "lastname":{
                                "type":"keyword"
                            },
                            "age":{
                                "type":"byte"
                            },
                            "gender":{
                                "type":"keyword"
                            },
                            "address":{
                                "type":"text"
                            },
                            "employer":{
                                "type":"keyword"
                            },
                            "email":{
                                "type":"text",
                                "analyzer":"email"
                            },
                            "city":{
                                "type":"keyword"
                            },
                            "state":{
                                "type":"keyword"
                            }
                        }
                    }
                }
            }
        });

        assert_eq!(expected, body);
    }
}