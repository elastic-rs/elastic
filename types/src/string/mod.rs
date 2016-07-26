//! Implementation of the Elasticsearch `string` type.
//!
//! Strings are stored as a sequence of tokens, constructed based on the given `analyzer`.
//!
//! # Examples
//!
//! For defining your own string mapping, see [mapping details](mapping/trait.ElasticStringMapping.html#derive-mapping).
//!
//! Map with a default `string`:
//!
//! ```
//! struct MyType {
//! 	pub field: String
//! }
//! ```
//!
//! Map with a custom `string`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # extern crate serde;
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::string::prelude::*;
//! # #[derive(Clone, Default, ElasticStringMapping)]
//! # pub struct MyStringMapping;
//! # impl ElasticStringMapping for MyStringMapping { }
//! struct MyType {
//! 	pub field: ElasticString<MyStringMapping>
//! }
//! # }
//! ```
//! 
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/string.html)

mod keyword;
mod text;

pub mod prelude {
	//! Includes non-mapping types for the `string` type.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::keyword::ElasticKeyword;
	pub use super::text::ElasticText;
}

pub mod mapping;