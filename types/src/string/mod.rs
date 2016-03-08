//! Implementation of the Elasticsearch `string` type.
//! 
//! Strings are stored as a sequence of tokens, constructed based on the given `analyzer`.
//! 
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

mod string;

pub mod mapping;
pub use self::string::*;

pub mod prelude {
	//! Includes non-mapping types for the `string` type.
    //! 
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::string::*;
}