mod point;
mod format;
mod formats;

pub mod mapping;
pub use self::point::*;
pub use self::format::*;
pub use self::formats::*;

pub type DefaultGeoPointFormat = BasicDateTime;

pub mod prelude {
	//! Includes non-mapping types for the `geo_point` type.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::DefaultGeoPointFormat;
	pub use super::format::*;
	pub use super::date::*;
	pub use super::formats::*;
}
