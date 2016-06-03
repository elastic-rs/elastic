//! Mapping for Elasticsearch `geo_shape` types.
//!
//! Custom mappings can be defined by implementing the right number mapping for some Rust primitive number type.
//! The implementation is the same for all number types, the only difference is the return type of `null_value`.
//!
//! # Examples
//!
//! Define a custom `ElasticGeoShapeMapping`:
//!
//! ## Derive Mapping
//!
//! This will produce the following mapping:
//!
//! ## Manually
//!

use std::marker::PhantomData;
use serde;
use serde::{ Serialize, Serializer };
use geojson::{ PointType };
use ::mapping::{ ElasticType, ElasticFieldMapping, ElasticTypeVisitor };

/// Elasticsearch datatype name.
pub const GEOSHAPE_DATATYPE: &'static str = "geo_shape";

/// Base `geo_shape` mapping.
pub trait ElasticGeoShapeMapping where
Self: ElasticFieldMapping<()> + Sized + Serialize {
    /// Name of the PrefixTree implementation to be used:
    /// `geohash` for `GeohashPrefixTree` and `quadtree` for `QuadPrefixTree`.
    fn tree() -> Option<Tree> {
        None
    }

    /// This parameter may be used instead of `tree_levels` to set an appropriate value
    /// for the `tree_levels` parameter.
    /// The value specifies the desired precision and Elasticsearch will calculate the best
    /// `tree_levels` value to honor this precision.
    /// The value should be a number followed by an optional distance unit.
    fn precision() -> Option<Distance> {
        None
    }

    /// Maximum number of layers to be used by the `PrefixTree`.
    /// This can be used to control the precision of shape representations and therefore
    /// how many terms are indexed.
    /// Defaults to the default value of the chosen `PrefixTree` implementation.
    /// Since this parameter requires a certain level of understanding of the underlying implementation,
    /// users may use the `precision` parameter instead.
    /// However, Elasticsearch only uses the `tree_levels` parameter internally and this is
    /// what is returned via the mapping API even if you use the `precision` parameter.
    fn tree_levels() -> Option<i32> {
        None
    }

    /// The `strategy` parameter defines the approach for how to represent shapes at indexing and search time.
    /// It also influences the capabilities available so it is recommended to let Elasticsearch
    /// set this parameter automatically.
    /// There are two strategies available: `recursive` and `term`.
    /// Term strategy supports point types only (the `points_only` parameter will be automatically set to `true`)
    /// while `Recursive` strategy supports all shape types.
    fn strategy() -> Option<Strategy> {
        None
    }

    /// Used as a hint to the `PrefixTree` about how precise it should be.
    /// Defaults to `0.025` (2.5%) with `0.5` as the maximum supported value.
    ///
    /// > PERFORMANCE NOTE: This value will default to `0` if a `precision` or `tree_level` definition is explicitly defined.
    /// This guarantees spatial precision at the level defined in the mapping.
    /// This can lead to significant memory usage for high resolution shapes with low error
    /// (e.g., large shapes at `1m` with < `0.001` error).
    /// To improve indexing performance (at the cost of query accuracy) explicitly define `tree_level`
    /// or `precision` along with a reasonable `distance_error_pct`,
    /// noting that large shapes will have greater false positives.
    fn distance_error_pct() -> Option<f32> {
        None
    }

    /// Setting this parameter in the `geo_shape` mapping explicitly sets vertex order for
    /// the coordinate list of a `geo_shape` field but can be overridden in each individual
    /// GeoJSON document.
    fn orientation() -> Option<Orientation> {
        None
    }

    /// Setting this option to `true` (defaults to `false`) configures the `geo_shape` field
    /// type for point shapes only (NOTE: Multi-Points are not yet supported).
    /// This optimizes index and search performance for the geohash and quadtree when it is
    /// known that only points will be indexed.
    /// At present `geo_shape` queries can not be executed on geo_point field types.
    /// This option bridges the gap by improving point performance on a `geo_shape` field
    /// so that geo_shape queries are optimal on a point only field.
    fn points_only() -> Option<bool> {
        None
    }
}

/// Default mapping for `String`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultGeoShapeMapping;
impl ElasticGeoShapeMapping for DefaultGeoShapeMapping { }

impl_geo_shape_mapping!(DefaultGeoShapeMapping);

/// Visitor for a `geo_shape` field mapping.
#[derive(Debug, PartialEq)]
pub struct ElasticGeoShapeMappingVisitor<T> where
T: ElasticGeoShapeMapping {
    phantom: PhantomData<T>
}

impl <T> ElasticTypeVisitor for ElasticGeoShapeMappingVisitor<T> where
T: ElasticGeoShapeMapping {
    fn new() -> Self {
        ElasticGeoShapeMappingVisitor {
            phantom: PhantomData
        }
    }
}
impl <T> serde::ser::MapVisitor for ElasticGeoShapeMappingVisitor<T> where
T: ElasticGeoShapeMapping {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
    where S: Serializer {
        try!(serializer.serialize_struct_elt("type", T::data_type()));

        if let Some(tree) = T::tree() {
            try!(serializer.serialize_struct_elt("tree", tree));
        }

        if let Some(precision) = T::precision() {
            try!(serializer.serialize_struct_elt("precision", precision));
        }

        if let Some(tree_levels) = T::tree_levels() {
            try!(serializer.serialize_struct_elt("tree_levels", tree_levels));
        }

        if let Some(strategy) = T::strategy() {
            try!(serializer.serialize_struct_elt("strategy", strategy));
        }

        if let Some(distance_error_pct) = T::distance_error_pct() {
            try!(serializer.serialize_struct_elt("distance_error_pct", distance_error_pct));
        }

        if let Some(orientation) = T::orientation() {
            try!(serializer.serialize_struct_elt("orientation", orientation));
        }

        if let Some(points_only) = T::points_only() {
            try!(serializer.serialize_struct_elt("points_only", points_only));
        }

        Ok(None)
    }
}

/// A unit of measure for distance.
pub enum DistanceUnit {
    /// For `in`.
    Inches,
    /// For `yd`.
    Yards,
    /// For `mi`.
    Miles,
    /// For `km`.
    Kilometers,
    /// For `m`.
    Meters,
    /// For `cm`.
    Centimeters,
    /// For `mm`.
    Millimeters
}

/// A distance value paired with a unit of measure.
pub struct Distance(pub f32, pub DistanceUnit);

impl ToString for Distance {
    fn to_string(&self) -> String {
        let value = self.0.to_string();
        let unit = match self.1 {
            DistanceUnit::Inches => "in",
            DistanceUnit::Yards => "yd",
            DistanceUnit::Miles => "mi",
            DistanceUnit::Kilometers => "km",
            DistanceUnit::Meters => "m",
            DistanceUnit::Centimeters => "cm",
            DistanceUnit::Millimeters => "mm"
        };

        let mut s = String::with_capacity(value.len() + unit.len());
        s.push_str(&value);
        s.push_str(unit);

        s
    }
}

impl Serialize for Distance {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

/// Name of the `PrefixTree` implementation to be used.
pub enum Tree {
    /// For `GeohashPrefixTree`.
    Geohash,
    /// For `QuadPrefixTree`.
    QuadPrefix
}

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match *self {
            Tree::Geohash => "geohash",
            Tree::QuadPrefix => "quadtree"
        })
    }
}

/// The strategy defines the approach for how to represent shapes at indexing and search time.
pub enum Strategy {
    /// Recursive strategy supports all shape types.
    Recursive,
    /// Term strategy supports point types only.
    Term
}

impl Serialize for Strategy {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match *self {
            Strategy::Recursive => "recursive",
            Strategy::Term => "term"
        })
    }
}

/// This parameter defines one of two coordinate system rules (Right-hand or Left-hand)
/// each of which can be specified in a few different ways.
/// - Right-hand rule: right, ccw, counterclockwise,
/// - Left-hand rule: left, cw, clockwise.
/// The default orientation (counterclockwise) complies with the OGC standard which defines outer
/// ring vertices in counterclockwise order with inner ring(s) vertices (holes) in clockwise order.
pub enum Orientation {
    /// For `cw`.
    Clockwise,
    /// For `ccw`.
    CounterClockwise
}

impl Serialize for Orientation {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match *self {
            Orientation::Clockwise => "cw",
            Orientation::CounterClockwise => "ccw"
        })
    }
}
