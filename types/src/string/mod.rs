//! Implementation of the Elasticsearch `string` type.
//! 
//! Strings are stored as a sequence of tokens, constructed based on the given `analyzer`.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

mod mapping;
mod string;

pub use self::mapping::*;
pub use self::string::*;