//! Commands for managing the bank index.
//!
//! The public API for this module just gives us a few REST API requests
//! for checking whether the bank index exists and creating it along with
//! analysers, filters and mapping for `Account`s.

use elastic::client::requests::Index;
use elastic::types::prelude::{IndexDocumentMapping, FieldType};
use super::account::{self, Account};

/// Get the name of the bank index.
pub fn name() -> Index<'static> {
    "bank-sample".into()
}

/// Get the settings and mappings for the index.
pub fn body() -> String {
    let body = json!({
        "settings" : {
            "analysis" : {
                "filter" : {
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
                },
                "analyzer" : {
                    "email": {
                        "tokenizer": "uax_url_email",
                        "filter": [
                            "email",
                            "lowercase",
                            "unique"
                        ]
                    }
                }
            }
        },
        "mappings": {
            account::name(): IndexDocumentMapping::from(Account::mapping())
        }
    });

    body.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_settings() {
        let body = body().unwrap();

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
        });

        assert_eq!(expected, body);
    }
}
