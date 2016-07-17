//! Elasticsearch Core Types
//!
//! An implementation of the core types in Elasticsearch documents.
//!
//! Provides `struct`s and `trait`s for defining Elasticsearch type mapping,
//! where correctness is enforced by Rust's type system.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/elastic_types).
//!
//! There are two ways to reference `elastic_types` in your projects, depending on whether you're on
//! the `stable`/`beta` or `nightly` channels.
//!
//! Builds on `nightly` benefit from compile-time codegen for better performance and easier
//! mapping definitions.
//! The story on `stable` will be improved over time so it won't be a second-class citizen forever.
//!
//! ## Nightly
//!
//! To get started, add `elastic_types` and `elastic_types_macros` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! elastic_types = { version = "*", defeault-features = false, features = "nightly" }
//! elastic_types_macros = "*"
//! ```
//!
//! And reference it in your crate root:
//!
//! ```ignore
//! #![feature(plugin)]
//! #![plugin(elastic_types_macros)]
//!
//! extern crate elastic_types;
//! ```
//!
//! ## Stable
//!
//! To get started, add `elastic_types` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! elastic_types = "*"
//! ```
//!
//! And reference it in your crate root:
//!
//! ```ignore
//! extern crate elastic_types;
//! ```
//!
//! Any code examples that aren't compatible with both `nightly` and `stable`, like deriving mappings,
//! have alternatives depending on the channel you're targeting.
//!
//! ## Map Your Types
//!
//! _For mapping on `stable`, see [here](object/index.html#manually)._
//!
//! Derive `ElasticType` on your Elasticsearch-mappable types:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! #[derive(Serialize, Deserialize, ElasticType)]
//! pub struct MyType {
//! 	pub my_date: ElasticDate<DefaultDateFormat>,
//! 	pub my_num: i32
//! }
//!
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! This will generate a mapping type for you called `{TypeName}Mapping`,
//! so in this case our mapping is called `MyTypeMapping`.
//! You can then serialise the mapping as json:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: ElasticDate<DefaultDateFormat>,
//! # 	pub my_string: String,
//! # 	pub my_num: i32
//! # }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! let mapping = TypeMapper::to_string(MyTypeMapping).unwrap();
//! # }
//! ```
//!
//! Which will output the following json:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_types_macros)]
//! # #[macro_use]
//! # extern crate json_str;
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: ElasticDate<DefaultDateFormat>,
//! # 	pub my_num: i32
//! # }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # let mapping = TypeMapper::to_string(MyTypeMapping).unwrap();
//! # let json = json_str!(
//! {
//!     "properties": {
//!         "my_date": {
//!             "type": "date",
//!             "format": "basic_date_time"
//!         },
//!         "my_num": {
//!             "type": "integer"
//!         }
//!     }
//! }
//! # );
//! # assert_eq!(json, mapping);
//! # }
//! ```
//!
//! Of course, structs that derive `ElasticType` can also be used as fields on other Elasticsearch types:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_types_macros)]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: ElasticDate<DefaultDateFormat>,
//! # 	pub my_num: i32
//! # }
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! #[derive(Serialize, Deserialize, ElasticType)]
//! pub struct MyOtherType {
//! 	pub my_type: MyType
//! }
//! # impl serde::Serialize for MyOtherType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyOtherType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! Our mapping for `MyOtherType` then looks like:
//!
//! ```
//! # #![feature(plugin, custom_derive, custom_attribute)]
//! # #![plugin(elastic_types_macros)]
//! # #[macro_use]
//! # extern crate json_str;
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # pub struct MyType {
//! # 	pub my_date: ElasticDate<DefaultDateFormat>,
//! # 	pub my_num: i32
//! # }
//! # impl Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # #[derive(Serialize, Deserialize, ElasticType)]
//! # pub struct MyOtherType {
//! # 	pub my_type: MyType
//! # }
//! # impl Serialize for MyOtherType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl Deserialize for MyOtherType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # let mapping = TypeMapper::to_string(MyOtherTypeMapping).unwrap();
//! # let json = json_str!(
//! {
//!     "properties": {
//!         "my_type": {
//!             "type": "nested",
//!             "properties": {
//!                 "my_date": {
//!                     "type": "date",
//!                     "format": "basic_date_time"
//!                 },
//!                 "my_num": {
//!                     "type": "integer"
//!                 }
//!             }
//!         }
//!     }
//! }
//! # );
//! # assert_eq!(json, mapping);
//! # }
//! ```
//!
//! Elasticsearch doesn't differentiate between nullable types or collections, so it's also possible
//! to derive mapping from `Option` or `Vec` types:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #[macro_use]
//! # extern crate elastic_types;
//! # extern crate serde;
//! # use serde::{ Serialize, Deserialize };
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::date::prelude::*;
//! #[derive(Serialize, Deserialize, ElasticType)]
//! pub struct MyType {
//! 	pub my_date: Option<ElasticDate<DefaultDateFormat>>,
//! 	pub my_num: Vec<i32>
//! }
//!
//! # impl serde::Serialize for MyType {
//! # 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # impl serde::Deserialize for MyType {
//! # 	 fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: serde::Deserializer {
//! # 		unimplemented!()
//! # 	}
//! # }
//! # fn main() {
//! # }
//! ```
//!
//! Which produces the same mapping as before.
//! See the [`object`](object/index.html) mod for more mapping examples.
//!
//! For mapping in [`rs-es`](http://benashford.github.io/rs-es/rs_es/index.html), see the [`RsesMapper`](mappers/struct.RsesMapper.html#examples).
//!
//! # Types
//!
//! Types in Elasticsearch are a combination of _source_ and _mapping_.
//! The source is the data (like `42` or `"my string"`) and the mapping is metadata about how to
//! interpret and use the data (like the format of a date string).
//!
//! The approach `elastic_types` takes to types is to bundle the mapping up as a _Zero Sized Type_.
//! This mapping type is then bound to a field as a generic parameter. For example:
//!
//! ```ignore
//! ElasticString<DefaultStringMapping>
//! ```
//!
//! The source is a `string` and the mapping is `DefaultStringMapping`.
//!
//! All Elasticsearch types implement the base `ElasticType<M: ElasticFieldMapping<F>, F>` trait
//! where `M` is the mapping and `F` is a type-specific format.
//!
//! The following table illustrates the types provided by `elastic_types`
//! (links to the relevant mapping type):
//!
//!  Elasticsearch Type  | Rust Type (Default Mapping) | Crate     | Rust Type (Custom Mapping)                                                       | Format Type
//!  ------------------- | --------------------------- | --------- | -------------------------------------------------------------------------------- | -----------
//!  `integer`           | `i32`                       | `std`     | [`ElasticInteger<M>`](number/mapping/trait.ElasticIntegerMapping.html)           | `()`
//!  `long`              | `i64`                       | `std`     | [`ElasticLong<M>`](number/mapping/trait.ElasticLongMapping.html)                 | `()`
//!  `short`             | `i16`                       | `std`     | [`ElasticShort<M>`](number/mapping/trait.ElasticShortMapping.html)               | `()`
//!  `byte`              | `i8`                        | `std`     | [`ElasticByte<M>`](number/mapping/trait.ElasticByteMapping.html)                 | `()`
//!  `float`             | `f32`                       | `std`     | [`ElasticFloat<M>`](number/mapping/trait.ElasticFloatMapping.html)               | `()`
//!  `double`            | `f64`                       | `std`     | [`ElasticDouble<M>`](number/mapping/trait.ElasticDoubleMapping.html)             | `()`
//!  `string`            | `String`                    | `std`     | [`ElasticString<M>`](string/mapping/trait.ElasticStringMapping.html)             | `()`
//!  `boolean`           | `bool`                      | `std`     | [`ElasticBoolean<M>`](boolean/mapping/trait.ElasticBooleanMapping.html)          | `()`
//!  `ip`                | `Ipv4Addr`                  | `std`     | [`ElasticIp<M>`](ip/mapping/trait.ElasticIpMapping.html)                         | `()`
//!  `date`              | `DateTime<UTC>`             | `chrono`  | [`ElasticDate<F, M>`](date/mapping/trait.ElasticDateMapping.html)                | `DateFormat`
//!  `geo_point`         | `Point`                     | `geo`     | [`ElasticGeoPoint<F, M>`](geo/point/mapping/trait.ElasticGeoPointMapping.html)   | `GeoPointFormat`
//!  `geo_shape`         | -                           | `geojson` | [`ElasticGeoShape<M>`](geo/shape/mapping/trait.ElasticGeoShapeMapping.html)      | `()`
//!
//! The following sections explain this table.
//!
//! ## Mapping
//!
//! Having the mapping available at compile-time makes it easy to write efficient generic methods
//! that use type mapping.
//!
//! Where there's a `std` type that's equivalent to an Elasticsearch type (like `i32` for `integer`),
//! a default mapping is implemented for that type.
//! That means you can use primitives in your structs and have them mapped to the correct type in Elasticsearch.
//! If you want to provide your own mapping for a `std` type, there's also a struct provided by `elastic_types`
//! that wraps the `std` type but also takes an explicit mapping (like `ElasticInteger` for `i32`).
//!
//! Where there isn't a `std` type available (like `date`), an external crate is used and an implementation of
//! that type is provided (like `ElasticDate`, which implements `chrono::DateLike + chrono::TimeLike`).
//!
//! ## Formats
//!
//! For some types (like `ElasticDate`), it's helpful to have an extra generic parameter that describes the
//! `format` the data can take. For most types the format is `()`, because there aren't any alternative formats available.
//!
//! # Links
//!
//! - [Elasticsearch Mapping Concepts](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
//! - [Elasticsearch Type Concepts](https://www.elastic.co/guide/en/elasticsearch/reference/2.3/_basic_concepts.html#_type)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types/")]
#![deny(missing_docs)]

#![cfg_attr(feature = "nightly", feature(custom_derive, plugin, associated_type_defaults))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, elastic_date_macros))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", allow(identity_op))]

#[cfg_attr(not(feature = "nightly"), macro_use)]
#[cfg(not(feature = "nightly"))]
extern crate elastic_date_macros;

pub extern crate chrono;
pub extern crate geo as georust;
extern crate geohash;
pub extern crate geojson;

extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
pub mod mappers;

//Other type dependencies
#[cfg(feature = "serde_macros")]
include!("lib.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
