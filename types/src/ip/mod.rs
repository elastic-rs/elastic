//! Implementation of the Elasticsearch `boolean` type.
//!
//! # Examples
//!
//! For defining your own ip mapping, see [mapping details](mapping/trait.ElasticIpMapping.html#derive-mapping).
//!
//! Map with a default `ip`:
//!
//! ```
//! # use std::net::Ipv4Addr;
//! struct MyType {
//! 	pub field: std::net::Ipv4Addr
//! }
//! ```
//!
//! Map with a custom `ip`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # extern crate serde;
//! #[macro_use]
//! # extern crate elastic_types;
//! # fn main() {
//! # use elastic_types::prelude::*;
//! # ip_mapping!(MyIpMapping {});
//! struct MyType {
//! 	pub field: ElasticIp<MyIpMapping>
//! }
//! # }
//! ```
//!
//! # Links
//!
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/ip.html)

mod ip;

pub mod mapping;
pub use self::ip::*;

pub mod prelude {
	//! Includes non-mapping types for the `ip` type.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::ip::*;
}
