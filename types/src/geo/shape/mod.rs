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

pub mod mapping;
