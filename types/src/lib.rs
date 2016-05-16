//! Elasticsearch Core Types
//!
//! A high-level implementation of the core types in Elasticsearch documents.
//!
//! Types within this crate are self-contained and handle their own serialisation/deserialisation requirements.
//! Each type also supplies a `struct` for its [Put Mapping API](https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-put-mapping.html) properties.
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
//! elastic_types = { version = "*", defeault-features = false, features = "nightly-default" }
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
//! ## Map Your Types
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
//! The following table illustrates the types provided by `elastic_types`:
//!
//!  Elasticsearch Type  | Rust Type (Default Mapping) | Crate     | Rust Type (Custom Mapping)    | Format Type
//!  ------------------- | --------------------------- | --------- | ----------------------------- | -----------
//!  `integer`           | `i32`                       | `std`     | `ElasticInteger<M>`           | `()`
//!  `long`              | `i64`                       | `std`     | `ElasticLong<M>`              | `()`
//!  `short`             | `i16`                       | `std`     | `ElasticShort<M>`             | `()`
//!  `byte`              | `i8`                        | `std`     | `ElasticByte<M>`              | `()`
//!  `float`             | `f32`                       | `std`     | `ElasticFloat<M>`             | `()`
//!  `double`            | `f64`                       | `std`     | `ElasticDouble<M>`            | `()`
//!  `string`            | `String`                    | `std`     | `ElasticString<M>`            | `()`
//!  `boolean`           | `bool`                      | `std`     | `ElasticBoolean<M>`           | `()`
//!  `date`              | `DateTime<UTC>`             | `chrono`  | `ElasticDate<F, M>`           | `DateFormat`
//!  `geo_point`         | `Point`                     | `geo`     | `ElasticGeoPoint<F, M>`       | `GeoPointFormat`
//!  `geo_shape`         | `Geometry`                  | `geojson` | `ElasticGeoShape<M>`          | `()`
//!  `point`             | `PointType`                 | `geojson` | `ElasticPoint<M>`             | `()`
//!  `linestring`        | `LineStringType`            | `geojson` | `ElasticLineString<M>`        | `()`
//!  `polygon`           | `PolygonType`               | `geojson` | `ElasticPolygon<M>`           | `()`
//!  `multipoint`        | `Vec<PointType>`            | `geojson` | `ElasticMultiPoint<M>`        | `()`
//!  `multipolygon`      | `Vec<PolygonType`           | `geojson` | `ElasticMultiPolygon<M>`      | `()`
//!  `geometrycollection`| `Vec<Geometry>`             | `geojson` | `ElasticGeoCollection<M>`     | `()`
//!  `envelope`          | -                           | -         | `ElasticEnvelope<M>`          | `()`
//!  `circle`            | -                           | -         | `ElasticCircle<M>`            | `()`
//!  `object`            | -                           | -         | user-defined `struct`         | `()`
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
//! # Excluding Dependencies
//!
//! Any Elasticsearch type that requires an external dependency (like `chrono` for `ElasticDate`)
//! is actually feature-gated, and that type can be excluded from builds if they aren't needed
//! to reduce the dependency footprint.
//!
//! To exclude types, use the appropriate base features in your `Cargo.toml` as defined below.
//! The base feature will exclude all possible types, so you can go and include just the ones you need.
//!
//! On `nightly`:
//!
//! ```ignore
//! elastic_types = { version = "*", default-features = false, features = [ "nightly", {type features here} ] }
//! ```
//!
//! On `stable`:
//!
//! ```ignore
//! elastic_types = { version = "*", default-features = false, features = [ "stable", {type features here} ]}
//! ```
//!
//! ## Possible Features
//!
//! The following types are provided as features (enabled by default):
//!
//!  Rust Type      | Stable Feature | Nightly Feature      | Excluded Crates
//!  -------------- | -------------- | -------------------- | ----------------------------
//!  `ElasticDate`  | `date-ty`      | `date-ty-nightly`    | `chrono`
//!  `ElasticGeo*`  | `geo-ty`       | `geo-ty-nightly`     | `geo`, `geohash`, `geojson`
//!
//! ## Examples
//!
//! Exclude all types with dependent crates:
//!
//! On `nightly`:
//!
//! ```ignore
//! elastic_types = {
//!     version = "*",
//!     default-features = false,
//!     features = [ "nightly" ]
//! }
//! ```
//!
//! On `stable`:
//!
//! ```ignore
//! elastic_types = {
//!     version = "*",
//!     default-features = false,
//!     features = [ "stable" ]
//! }
//! ```
//!
//! Include just `ElasticDate`:
//!
//! On `nightly`:
//!
//! ```ignore
//! elastic_types = {
//!     version = "*",
//!     default-features = false,
//!     features = [ "nightly", "date-ty-nightly" ]
//! }
//! ```
//!
//! On `stable`:
//!
//! ```ignore
//! elastic_types = {
//!     version = "*",
//!     default-features = false,
//!     features = [ "stable", "date-ty" ]
//! }
//! ```
//!
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/guide/current/mapping.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

#![doc(html_root_url = "http://kodraus.github.io/rustdoc/elastic_types/")]
#![deny(missing_docs)]

#![cfg_attr(feature = "nightly-testing", plugin(clippy))]

#![cfg_attr(feature = "nightly", feature(custom_derive, plugin, associated_type_defaults))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]
#![cfg_attr(all(feature = "elastic_date_macros", feature = "nightly"), plugin(elastic_date_macros))]

#[macro_use]
mod macros;
pub mod mappers;

//Date type dependencies
#[cfg_attr(all(feature = "elastic_date_macros", not(feature = "nightly")), macro_use)]
#[cfg(all(feature = "elastic_date_macros", not(feature = "nightly")))]
extern crate elastic_date_macros;
#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(any(feature = "date-ty", feature = "date-ty-nightly"))]
pub mod date;

//Geo type dependencies
#[cfg(feature = "geo")]
extern crate geo;
#[cfg(feature = "geohash")]
extern crate geohash;
#[cfg(feature = "geojson")]
extern crate geojson;
#[cfg(any(feature = "geo-ty", feature = "geo-ty-nightly"))]
pub mod geo;

extern crate serde;
extern crate serde_json;

//Other type dependencies
#[cfg(feature = "serde_macros")]
include!("lib.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
