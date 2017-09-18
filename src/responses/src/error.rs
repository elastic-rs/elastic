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
        IndexNotFound { index: String } {
            description("index not found")
            display("index not found: '{}'", index)
        }
        IndexAlreadyExists { index: String } {
            description("index already exists")
            display("index already exists: '{}'", index)
        }
        Parsing { line: u64, col: u64, reason: String } {
            description("request parse error")
            display("request parse error: '{}' on line: {}, col: {}", reason, line, col)
        }
        MapperParsing { reason: String } {
            description("mapper parse error")
            display("mapper parse error: '{}'", reason)
        }
        ActionRequestValidation { reason: String }
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
