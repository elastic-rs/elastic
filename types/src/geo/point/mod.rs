//! Implementation of the Elasticsearch `geo_point` type.
//!
//! Geo points are an Elasticsearch specific geospatial type with an `x` (`lon`) and `y` (`lat`)
//! component.
//! `ElasticGeoPoint` is a good choice for storing and analysing geospatial points where geojson
//! compatibility isn't needed.
//!
//! # Examples
//!
//! For defining your own geo point mapping, see [mapping details](mapping/trait.ElasticGeoPointMapping.html#derive-mapping).
//!
//! Map with a default `geo_point`:
//!
//! ```
//! # use elastic_types::geo::point::prelude::*;
//! struct MyType {
//! 	pub field: ElasticGeoPoint<DefaultGeoPointFormat>
//! }
//! ```
//!
//! Map with a custom `geo_point`:
//!
//! ```
//! # #![feature(plugin, custom_derive)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # extern crate serde;
//! # extern crate elastic_types;
//! # use std::marker::PhantomData;
//! # fn main() {
//! # use elastic_types::mapping::prelude::*;
//! # use elastic_types::geo::point::prelude::*;
//! # #[derive(Default, Clone, ElasticGeoPointMapping)]
//! # pub struct MyGeoPointMapping<T: GeoPointFormat = GeoPointString> {
//! #	phantom: PhantomData<T>
//! # }
//! # impl <T: GeoPointFormat> ElasticGeoPointMapping<T> for MyGeoPointMapping<T> { }
//! struct MyType {
//! 	pub field: ElasticGeoPoint<GeoPointString, MyGeoPointMapping>
//! }
//! # }
//! ```
//!
//! # Links
//! - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)

mod point;
mod format;
mod formats;

pub mod mapping;
pub use self::point::*;
pub use self::format::*;
pub use self::formats::*;

/// The default `geo_point` format (`GeoPointArray`).
pub type DefaultGeoPointFormat = GeoPointArray;

pub mod prelude {
	//! Includes non-mapping types for the `geo_point` type.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::DefaultGeoPointFormat;
	pub use super::format::*;
	pub use super::point::*;
	pub use super::formats::*;
}
