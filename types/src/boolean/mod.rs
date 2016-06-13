//! Implementation of the Elasticsearch `boolean` types.
//!
//! # Examples
//!
//! For defining your own boolean mapping, see [mapping details](mapping/trait.ElasticBooleanMapping.html#derive-mapping).
//!
//! Map with a default `boolean`:
//!
//! ```
//! struct MyType {
//! 	pub field: bool
//! }
//! ```
//!
//! Map with a custom `boolean`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # extern crate serde;
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::boolean::prelude::*;
//! # #[derive(Debug, Clone, Default, ElasticBooleanMapping)]
//! # pub struct MyBooleanMapping;
//! # impl ElasticBooleanMapping for MyBooleanMapping { }
//! struct MyType {
//! 	pub field: ElasticBoolean<MyBooleanMapping>
//! }
//! # }
//! ```
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/boolean.html)

mod boolean;

pub mod mapping;
pub use self::boolean::*;

pub mod prelude {
	//! Includes non-mapping types for the `boolean` type.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::boolean::*;
}
