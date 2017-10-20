/*!
Error types from Elasticsearch
*/

use serde::{Deserialize, Deserializer};
use serde_json::{Error as JsonError, Map, Value};
use std::io::Error as IoError;

quick_error! {
    /** An error parsing a response stream. */
    #[derive(Debug)]
    pub enum ParseResponseError {
        /** The response contains invalid json. */
        Json(err: JsonError) {
            from()
        }
        /** The response caused an io error while buffering. */
        Io(err: IoError) {
            from()
        }
    }
}

quick_error! {
    /** An error parsing a REST API response to a success value. */
    #[derive(Debug)]
    pub enum ResponseError {
        /** A REST API error from Elasticsearch. */
        Api(err: ApiError) {
            from()
        }
        /** An error parsing a response body. */
        Parse(err: ParseResponseError) {
            from()
        }
    }
}

quick_error! {
    /** A REST API error response. */
    #[derive(Debug, PartialEq)]
    pub enum ApiError {
        /**
        An index wasn't found.

        Some endpoints, like search, will return an `IndexNotFound` error if a request is made to a missing index.
        Other endpoints will return a successful response even if the index is missing but include some error property in the response body.
        */
        IndexNotFound { index: String } {
            description("index not found")
            display("index not found: '{}'", index)
        }
        /**
        A document wasn't found.

        This error can occur when attempting to update a document that doesn't already exist.
        */
        DocumentMissing { index: String } {
            description("document missing")
            display("document in index is missing: '{}'", index)
        }
        /**
        An index already exists but was expected to.

        Attempting to create an index with a name that's already in use will result in an `IndexAlreadyExists` error.
        */
        IndexAlreadyExists { index: String } {
            description("index already exists")
            display("index already exists: '{}'", index)
        }
        /**
        The request body contains invalid data.

        If a Query DSL query contains invalid JSON or unrecognised properties then Elasticsearch will return a `Parsing` error.
        */
        Parsing { line: u64, col: u64, reason: String } {
            description("request parse error")
            display("request parse error: '{}' on line: {}, col: {}", reason, line, col)
        }
        /**
        The document mapping contains invalid data.

        If a put mapping request contains invalid JSON or unrecognised properties then Elasticsearch will return a `MapperParsing` error.
        */
        MapperParsing { reason: String } {
            description("mapper parse error")
            display("mapper parse error: '{}'", reason)
        }
        /**
        The request body can't be processed.

        Some endpoints that expect certain constraints of a request to hold will return an `ActionRequestValidation` error if those constraints aren't met.
        */
        ActionRequestValidation { reason: String } {
            description("action request failed validation")
            display("action request failed validation: '{}'", reason)
        }
        /**
        A currently unrecognised error occurred.
        
        **WARNING:** Don't rely on this variant to capture an error.
        As new variants are introduced they will no longer be matched by `ApiError::Other`.
        For this reason, this variant will disappear in the future.
        */
        Other(v: Map<String, Value>) {
            description("error response from Elasticsearch")
            display("error response from Elasticsearch: {:?}", v)
        }
        #[doc(hidden)]
        __NonExhaustive {}
    }
}

macro_rules! error_key {
    ($obj:ident [ $key:ident ] : |$cast:ident| $cast_expr:expr) => ({
            let key = $obj.get(stringify!($key))
                          .and_then(|$cast| $cast_expr)
                          .map(|v| v.to_owned());

            match key {
                Some(v) => v,
                _ => return ApiError::Other($obj).into()
            }
        }
    )
}

impl<'de> Deserialize<'de> for ApiError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Map::deserialize(deserializer)?;

        Ok(value.into())
    }
}

impl From<Map<String, Value>> for ApiError {
    fn from(mut value: Map<String, Value>) -> Self {
        let obj = {
            match value.remove("error") {
                Some(Value::Object(value)) => value,
                _ => return ApiError::Other(value),
            }
        };

        let ty = {
            let ty = obj.get("type")
                .and_then(|v| v.as_str())
                .map(|v| v.to_owned());

            match ty {
                Some(ty) => ty,
                _ => return ApiError::Other(obj),
            }
        };

        match ty.as_ref() {
            "index_not_found_exception" => {
                let index = error_key!(obj[index]: |v| v.as_str());

                ApiError::IndexNotFound {
                    index: index.into(),
                }
            }
            "index_already_exists_exception" => {
                let index = error_key!(obj[index]: |v| v.as_str());

                ApiError::IndexAlreadyExists {
                    index: index.into(),
                }
            }
            "document_missing_exception" => {
                let index = error_key!(obj[index]: |v| v.as_str());

                ApiError::DocumentMissing {
                    index: index.into(),
                }
            }
            "parsing_exception" => {
                let line = error_key!(obj[line]: |v| v.as_u64());
                let col = error_key!(obj[col]: |v| v.as_u64());
                let reason = error_key!(obj[reason]: |v| v.as_str());

                ApiError::Parsing {
                    line: line,
                    col: col,
                    reason: reason.into(),
                }
            }
            "mapper_parsing_exception" => {
                let reason = error_key!(obj[reason]: |v| v.as_str());

                ApiError::MapperParsing {
                    reason: reason.into(),
                }
            }
            "action_request_validation_exception" => {
                let reason = error_key!(obj[reason]: |v| v.as_str());

                ApiError::ActionRequestValidation {
                    reason: reason.into(),
                }
            }
            _ => ApiError::Other(obj),
        }
    }
}
