//! Implementation of the Elasticsearch `date` type.
//! 
//! Dates in Elasticsearch are exposed as a formatted `string` which can contain a `date` and/or a `time` component.
//! All dates are expected to be given in `UTC`, and if no time is supplied, then 12:00am will be used instead.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)

pub mod format;
mod date;

pub use self::format::Format;
pub use self::date::*;