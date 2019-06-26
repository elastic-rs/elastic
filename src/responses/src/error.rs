/*!
Error types from Elasticsearch.
*/

use serde::{
    Deserialize,
    Deserializer,
};
use serde_json::{
    Error as JsonError,
    Map,
    Value,
};
use std::{
    error::Error as StdError,
    fmt,
    io::Error as IoError,
};

mod inner {
    use serde_json::{
        Map,
        Value,
    };
    use std::{
        error::Error as StdError,
        fmt,
    };

    use super::ApiError;

    #[derive(Debug)]
    pub struct UnknownApiError(pub Map<String, Value>);

    impl fmt::Display for UnknownApiError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            (&self.0 as &fmt::Debug).fmt(f)
        }
    }

    impl StdError for UnknownApiError {
        fn description(&self) -> &str {
            "an unknown API error"
        }

        fn cause(&self) -> Option<&StdError> {
            None
        }
    }

    pub enum ParsedApiError {
        Known(ApiError),
        Unknown(Map<String, Value>),
    }
}

pub(crate) use self::inner::*;

/**
A generic error parsing an API response.
*/
#[derive(Debug)]
pub struct ParseError {
    inner: Box<StdError + Send + Sync>,
}

impl ParseError {
    pub fn new<E>(err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        ParseError {
            inner: Box::new(err),
        }
    }
}

impl From<IoError> for ParseError {
    fn from(err: IoError) -> Self {
        ParseError::new(err)
    }
}

impl From<JsonError> for ParseError {
    fn from(err: JsonError) -> Self {
        ParseError::new(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl StdError for ParseError {
    fn description(&self) -> &str {
        self.inner.description()
    }

    fn cause(&self) -> Option<&StdError> {
        Some(&*self.inner)
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
        Parse(err: ParseError) {
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
        The request body can't be processed.

        Some endpoints that expect certain constraints of a request to hold will return an `ActionRequestValidation` error if those constraints aren't met.
        */
        ActionRequestValidation { reason: String } {
            description("action request failed validation")
            display("action request failed validation: '{}'", reason)
        }
        /** The request body can't be parsed.  */
        Parsing { reason: String } {
            description("parsing failed")
            display("parsing failed: '{}'", reason)
        }
        /** There was an illegal argument in the request.  */
        IllegalArgument { reason: String } {
            description("illegal argument")
            display("illegal argument: '{}'", reason)
        }
        /** There was a problem with the SQL query. */
        Verification { reason: String} {
            description("verification exception")
            display("verification error: '{}", reason)
        }
        #[doc(hidden)]
        __NonExhaustive {}
    }
}

macro_rules! error_key {
    ($obj:ident [ $key:ident ] : |$cast:ident| $cast_expr:expr) => {{
        let key = $obj
            .get(stringify!($key))
            .and_then(|$cast| $cast_expr)
            .map(|v| v.to_owned());

        match key {
            Some(v) => v,
            _ => return ParsedApiError::Unknown($obj),
        }
    }};
}

impl<'de> Deserialize<'de> for ParsedApiError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Map::deserialize(deserializer)?;

        Ok(value.into())
    }
}

impl From<Map<String, Value>> for ParsedApiError {
    fn from(mut value: Map<String, Value>) -> Self {
        let obj = {
            match value.remove("error") {
                Some(Value::Object(value)) => value,
                _ => return ParsedApiError::Unknown(value),
            }
        };

        let ty = {
            let ty = obj
                .get("type")
                .and_then(|v| v.as_str())
                .map(|v| v.to_owned());

            match ty {
                Some(ty) => ty,
                _ => return ParsedApiError::Unknown(obj),
            }
        };

        match ty.as_ref() {
            "index_not_found_exception" => {
                let index = error_key!(obj[index]: |v| v.as_str());

                ParsedApiError::Known(ApiError::IndexNotFound {
                    index: index.into(),
                })
            }
            "index_already_exists_exception" => {
                let index = error_key!(obj[index]: |v| v.as_str());

                ParsedApiError::Known(ApiError::IndexAlreadyExists {
                    index: index.into(),
                })
            }
            "document_missing_exception" => {
                let index = error_key!(obj[index]: |v| v.as_str());

                ParsedApiError::Known(ApiError::DocumentMissing {
                    index: index.into(),
                })
            }
            "action_request_validation_exception" => {
                let reason = error_key!(obj[reason]: |v| v.as_str());

                ParsedApiError::Known(ApiError::ActionRequestValidation {
                    reason: reason.into(),
                })
            }
            "parsing_exception" => {
                let reason = error_key!(obj[reason]: |v| v.as_str());

                ParsedApiError::Known(ApiError::Parsing {
                    reason: reason.into(),
                })
            }
            "illegal_argument_exception" => {
                let reason = error_key!(obj[reason]: |v| v.as_str());

                ParsedApiError::Known(ApiError::IllegalArgument {
                    reason: reason.into(),
                })
            }
            "verification_exception" => {
                let reason = error_key!(obj[reason]: |v| v.as_str());

                ParsedApiError::Known(ApiError::Verification {
                    reason: reason.into(),
                })
            }
            _ => ParsedApiError::Unknown(obj),
        }
    }
}
