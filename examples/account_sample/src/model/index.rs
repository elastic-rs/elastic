//! Commands for managing the bank index.
//!
//! The public API for this module just gives us a few REST API requests
//! for checking whether the bank index exists and creating it along with
//! analysers, filters and mapping for `Account`s.

use super::account::Account;
use elastic::prelude::*;
use serde_json::Value;

/// Get the name of the bank index.
pub fn name() -> Index<'static> {
    "bank-sample".into()
}

/// Get the settings and mappings for the index.
pub fn body() -> Value {
    json!({
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
            Account::static_ty(): Account::index_mapping()
        }
    })
}
