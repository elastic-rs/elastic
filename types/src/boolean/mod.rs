//! Implementation of the Elasticsearch `boolean` types.

mod boolean;

pub mod mapping;
pub use self::boolean::*;

pub mod prelude {
	//! Includes non-mapping types for the `boolean` type.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::boolean::*;
}
