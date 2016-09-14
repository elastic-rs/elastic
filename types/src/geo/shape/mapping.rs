//! Mapping for Elasticsearch `geo_shape` types.

use serde::{ Serialize, Serializer };
use ::mapping::{ ElasticFieldMapping, ElasticFieldMappingWrapper };
use ::geo::mapping::Distance;

/// Elasticsearch datatype name.
pub const GEOSHAPE_DATATYPE: &'static str = "geo_shape";

#[doc(hidden)]
#[derive(Default)]
pub struct GeoShapeFormat;

/// The base requirements for mapping a `geo_shape` type.
///
/// Custom mappings can be defined by implementing `GeoShapeMapping`.
///
/// # Examples
///
/// Define a custom `GeoShapeMapping`:
///
/// ## Derive Mapping
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use elastic_types::prelude::*;
/// #[derive(Default)]
/// struct MyGeoShapeMapping;
/// impl GeoShapeMapping for MyGeoShapeMapping {
/// 	//Overload the mapping functions here
/// 	fn tree_levels() -> Option<i32> {
///			Some(2)
///		}
/// }
/// # fn main() {}
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(elastic_types_macros)]
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use elastic_types::prelude::*;
/// #[derive(Default)]
/// # struct MyGeoShapeMapping;
/// # impl GeoShapeMapping for MyGeoShapeMapping {
/// # 	//Overload the mapping functions here
/// # 	fn tree_levels() -> Option<i32> {
///	# 		Some(2)
///	# 	}
/// # }
/// # fn main() {
/// # let mapping = FieldMapper::to_string(MyGeoShapeMapping).unwrap();
/// # let json = json_str!(
/// {
///     "type": "geo_shape",
/// 	"tree_levels": 2
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
pub trait GeoShapeMapping where
Self: Default {
	/// Name of the PrefixTree implementation to be used:
	/// `geohash` for `GeohashPrefixTree` and `quadtree` for `QuadPrefixTree`.
	fn tree() -> Option<Tree> { None }

	/// This parameter may be used instead of `tree_levels` to set an appropriate value
	/// for the `tree_levels` parameter.
	/// The value specifies the desired precision and Elasticsearch will calculate the best
	/// `tree_levels` value to honor this precision.
	/// The value should be a number followed by an optional distance unit.
	fn precision() -> Option<Distance> { None }

	/// Maximum number of layers to be used by the `PrefixTree`.
	/// This can be used to control the precision of shape representations and therefore
	/// how many terms are indexed.
	/// Defaults to the default value of the chosen `PrefixTree` implementation.
	/// Since this parameter requires a certain level of understanding of the underlying implementation,
	/// users may use the `precision` parameter instead.
	/// However, Elasticsearch only uses the `tree_levels` parameter internally and this is
	/// what is returned via the mapping API even if you use the `precision` parameter.
	fn tree_levels() -> Option<i32> { None }

	/// The `strategy` parameter defines the approach for how to represent shapes at indexing and search time.
	/// It also influences the capabilities available so it is recommended to let Elasticsearch
	/// set this parameter automatically.
	/// There are two strategies available: `recursive` and `term`.
	/// Term strategy supports point types only (the `points_only` parameter will be automatically set to `true`)
	/// while `Recursive` strategy supports all shape types.
	fn strategy() -> Option<Strategy> { None }

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
	fn distance_error_pct() -> Option<f32> { None }

	/// Setting this parameter in the `geo_shape` mapping explicitly sets vertex order for
	/// the coordinate list of a `geo_shape` field but can be overridden in each individual
	/// GeoJSON document.
	fn orientation() -> Option<Orientation> { None }

	/// Setting this option to `true` (defaults to `false`) configures the `geo_shape` field
	/// type for point shapes only (NOTE: Multi-Points are not yet supported).
	/// This optimizes index and search performance for the geohash and quadtree when it is
	/// known that only points will be indexed.
	/// At present `geo_shape` queries can not be executed on geo_point field types.
	/// This option bridges the gap by improving point performance on a `geo_shape` field
	/// so that geo_shape queries are optimal on a point only field.
	fn points_only() -> Option<bool> { None }
}


impl <T> ElasticFieldMapping<GeoShapeFormat> for T where
T: GeoShapeMapping { 
	type SerType = ElasticFieldMappingWrapper<T, GeoShapeFormat>;

	fn data_type() -> &'static str { GEOSHAPE_DATATYPE }
}

impl <T> Serialize for ElasticFieldMappingWrapper<T, GeoShapeFormat> where
T: ElasticFieldMapping<GeoShapeFormat> + GeoShapeMapping {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		let mut state = try!(serializer.serialize_struct("mapping", 8));

		try!(serializer.serialize_struct_elt(&mut state, "type", T::data_type()));

		ser_field!(serializer, &mut state, T::tree(), "tree");
		ser_field!(serializer, &mut state, T::precision(), "precision");
		ser_field!(serializer, &mut state, T::tree_levels(), "tree_levels");
		ser_field!(serializer, &mut state, T::strategy(), "strategy");
		ser_field!(serializer, &mut state, T::distance_error_pct(), "distance_error_pct");
		ser_field!(serializer, &mut state, T::orientation(), "orientation");
		ser_field!(serializer, &mut state, T::points_only(), "points_only");

		serializer.serialize_struct_end(state)
	}
}

/// Default mapping for `geo_shape`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultGeoShapeMapping;
impl GeoShapeMapping for DefaultGeoShapeMapping { }

/// Name of the `PrefixTree` implementation to be used.
pub enum Tree {
	/// For `GeohashPrefixTree`.
	Geohash,
	/// For `QuadPrefixTree`.
	QuadPrefix
}

impl Serialize for Tree {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
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
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
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
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
	S: Serializer {
		serializer.serialize_str(match *self {
			Orientation::Clockwise => "cw",
			Orientation::CounterClockwise => "ccw"
		})
	}
}
