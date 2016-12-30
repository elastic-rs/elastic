//! Implementation of the Elasticsearch `number` types.
//!
//! Numeric types come in a number of flavours that correspond to primitives in Rust:
//!
//! Rust | Elasticsearch
//! ------ | ------------------
//! `i64` |  `long`
//! `i32` | `integer`
//! `i16` | `short`
//! `i8` | `byte`
//! `f64` | `double`
//! `f32` | `float`
//!
//! For mapping a number with the default mapping, you can use the Rust primitive.
//! If you need to use a custom mapping, then there is an `Elastic*` type for each number.
//!
//! # Examples
//!
//! For defining your own number mapping, see [mapping details](mapping/index.html#derive-mapping).
//!
//! Map with a default `number` (`i32` in this case):
//!
//! ```
//! struct MyType {
//! 	pub field: i32
//! }
//! ```
//!
//! Map with a custom `number` (`i32` in this case):
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_derive)]
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate elastic_types;
//! # use elastic_types::prelude::*;
//! # fn main() {
//! # #[derive(Default)]
//! # struct MyIntegerMapping;
//! # impl IntegerMapping for MyIntegerMapping { }
//! struct MyType {
//! 	pub field: Integer<MyIntegerMapping>
//! }
//! # }
//! ```
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/number.html)

#[macro_use]
pub mod mapping;

mod number;
pub use self::number::*;

pub mod prelude {
    //! Includes all types for the `number` type.
    //!
    //! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use super::number::*;
    pub use super::mapping::*;
}
