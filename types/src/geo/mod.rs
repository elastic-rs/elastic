//! Implementation of the Elasticsearch `geo` types.
//!
//! For indexing simple geo points with an `x` and `y` coordinate, use `point::ElasticGeoPoint`.
//! For indexing `geojson`, there are implementations like `shape::ElasticGeometry`.

pub mod point;
pub mod shape;
