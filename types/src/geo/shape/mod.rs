//! Implementation of the Elasticsearch `geo_shape` types.
//!
//! Each geojson type has an equivalent Elasticsearch geo shape type:
//!
//!  GeoJSON Type         | Rust Type
//!  -------------------- | -------------------------
//!  `Point`              | `ElasticPoint`
//!  `LineString`         | `ElasticLineString`
//!  `Polygon`            | `ElasticPolygon`
//!  `MultiPoint`         | `Vec<ElasticPoint>`
//!  `MultiLineString`    | `Vec<LineString>`
//!  `MultiPolygon`       | `Vec<Polygon>`

mod shape;

pub use self::shape::*;
pub mod mapping;

pub mod prelude {
	//! Includes non-mapping types for the `geo_shape` types.
	//!
	//! This is a convenience module to make it easy to build mappings for multiple types without too many `use` statements.

	pub use super::shape::*;
}
