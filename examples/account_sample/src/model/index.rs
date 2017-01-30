use elastic::client::requests::IndicesCreateRequest;
use elastic::types::prelude::{Document, DocumentType, FieldType};
use serde_json;
use super::account::Account;

pub fn name() -> &'static str {
    INDEX
}

pub fn request() -> IndicesCreateRequest<'static> {
    IndicesCreateRequest::for_index(name(), bank_index())
}

const INDEX: &'static str = "bank-sample";

fn bank_index() -> String {
    let account_name = format!("\"{}\"", Account::name());
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
    fn request_url() {
        let req = request();

        assert_eq!("/bank-sample", req.url.as_ref());
    }

    #[test]
    fn request_body() {
        let req = request();

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