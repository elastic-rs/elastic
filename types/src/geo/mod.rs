//! Implementation of the Elasticsearch `geo` types.
//!
//! For indexing simple geo points with an `x` and `y` coordinate, use `point::ElasticGeoPoint`.
//! For indexing `geojson`, there are implementations like `shape::ElasticGeometry`.

pub mod point;
pub mod shape;

pub mod prelude {
    //! Includes non-mapping types for the `geo_point` and `geo_shape` types.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

    pub use super::point::prelude::*;
    pub use super::shape::prelude::*;
}
