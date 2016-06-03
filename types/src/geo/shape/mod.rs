//! Implementation of the Elasticsearch `geo_shape` type.
//!

mod shape;

pub use self::shape::*;
pub mod mapping;

pub mod prelude {
	//! Includes non-mapping types for the `geo_shape` types.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::shape::*;
}
