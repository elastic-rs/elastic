//! Implementation of the Elasticsearch `geo_point` type.
//!
//! Geo points are an Elasticsearch specific geospatial type with an `x` (`lon`) and `y` (`lat`)
//! component.

mod point;
mod format;
mod formats;

pub mod mapping;
pub use self::point::*;
pub use self::format::*;
pub use self::formats::*;

/// The default `geo_point` format.
pub type DefaultGeoPointFormat = GeoPointObject;

pub mod prelude {
	//! Includes non-mapping types for the `geo_point` type.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::DefaultGeoPointFormat;
	pub use super::format::*;
	pub use super::point::*;
	pub use super::formats::*;
}
