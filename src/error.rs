use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value, Error as JsonError};

quick_error! {
    /// An error parsing a REST API response to a success value.
    #[derive(Debug)]
    pub enum ResponseError {
        Api(err: ApiError) {
            from()
        }
        Json(err: JsonError) {
            from()
        }
    }
}

quick_error! {
    /// A REST API error response.
    #[derive(Debug, PartialEq)]
    pub enum ApiError {
        IndexNotFound { index: String } {
            description("index not found")
            display("index not found: '{}'", index)
        }
        Parsing { line: u64, col: u64, reason: String } {
            description("request parse error")
            display("request parse error: '{}' on line: {}, col: {}", reason, line, col)
        }
        ActionRequestValidation { reason: String }
        Other(v: Map<String, Value>) {
            description("error response from Elasticsearch")
            display("error response from Elasticsearch: {:?}", v)
        }
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

impl Deserialize for ApiError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
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

                ApiError::IndexNotFound { index: index.into() }
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
