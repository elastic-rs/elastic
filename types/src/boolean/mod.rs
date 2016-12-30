//! Implementation of the Elasticsearch `boolean` types.
//!
//! # Examples
//!
//! For defining your own boolean mapping, see [mapping details](mapping/trait.BooleanMapping.html#derive-mapping).
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
//! # #![plugin(json_str, elastic_types_derive)]
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::prelude::*;
//! # #[derive(Default)]
//! # struct MyBooleanMapping;
//! # impl BooleanMapping for MyBooleanMapping { }
//! struct MyType {
//! 	pub field: Boolean<MyBooleanMapping>
//! }
//! # }
//! ```
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/boolean.html)

#[macro_use]
pub mod mapping;

mod boolean;
pub use self::boolean::*;

pub mod prelude {
    //! Includes all types for the `boolean` type.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use super::boolean::*;
    pub use super::mapping::*;
}
