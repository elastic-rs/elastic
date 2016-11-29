//! Implementation of the Elasticsearch `keyword` type.
//!
//! Keyword fields are stored as a raw string of tokens, and aren't analysed when querying.
//! They're useful for data that only has meaning when considered as a whole, like an id
//! or single word.

#[macro_use]
pub mod mapping;
mod keyword;

pub use self::keyword::*;
