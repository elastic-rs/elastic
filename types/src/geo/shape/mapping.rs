//! Mapping for Elasticsearch `geo_shape` types.
//!
//! Custom mappings can be defined by implementing the right number mapping for some Rust primitive number type.
//! The implementation is the same for all number types, the only difference is the return type of `null_value`.
//!
//! # Examples
//!
//! Define a custom `ElasticPointMapping`:
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

/// Elasticsearch datatype name.
pub const POINT_DATATYPE: &'static str = "point";
/// Elasticsearch datatype name.
pub const MULTIPOINT_DATATYPE: &'static str = "multipoint";

macro_rules! geo_shape_ser {
    ($m:ident) => (
        //TODO: map $m::*
    )
}

macro_rules! geo_shape_mapping {
    ($m:ident, $v:ident, $n:ty) => (
    	/// Base `geo_shape` mapping.
    	pub trait $m where
        Self: ElasticFieldMapping<()> + Sized + Serialize {
			//TODO: Fill in field mapping
		}

		/// Visitor for a `geo_shape` field mapping.
		#[derive(Debug, PartialEq)]
		pub struct $v<T> where T: $m {
			phantom: PhantomData<T>
		}

        impl <T> ElasticTypeVisitor for $v<T> where T: $m {
            fn new() -> Self {
        		$v {
                    phantom: PhantomData
                }
        	}
        }
		impl <T> serde::ser::MapVisitor for $v<T> where T: $m {
			fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
			where S: Serializer {
				try!(serializer.serialize_struct_elt("type", T::data_type()));

				geo_shape_ser!($m);

				Ok(None)
			}
		}
    )
}

macro_rules! geo_multi_shape_mapping {
    ($m:ident, $mm:ident, $dt:ident, $v:ident) => (
        /// Multi mapping for a `geo_shape` type.
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $m<T> where
        T: $mm {
            phantom: PhantomData<T>
        }

        impl <T> ElasticFieldMapping<()> for $m<T> where
        T: $mm {
            type Visitor = $v<T>;
            type MultiFieldMapping = Self;

            fn data_type() -> &'static str {
                $dt
            }
        }

        /// Visitor for a `geo_shape` multi field mapping.
        #[derive(Debug, PartialEq)]
        pub struct $v<T> where
        T: ElasticPointMapping {
            phantom: PhantomData<T>
        }

        impl <T> ElasticTypeVisitor for $v<T> where
        T: $mm {
            fn new() -> Self {
                ElasticMultiPointMappingVisitor {
                    phantom: PhantomData
                }
            }
        }
        impl <T> serde::ser::MapVisitor for $v<T> where
        T: $mm {
            fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
            where S: Serializer {
                try!(serializer.serialize_struct_elt("type", $m::<T>::data_type()));

                geo_shape_ser!($mm);

                Ok(None)
            }
        }

        impl <T> Serialize for $m<T> where
        T: $mm {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer {
                serializer.serialize_struct("mapping", Self::get_visitor())
            }
        }
    )
}

/// A unit of measure for distance.
pub enum DistanceUnit {
    /// `in`.
    Inches,
    /// `yd`.
    Yards,
    /// `mi`.
    Miles,
    /// `km`.
    Kilometers,
    /// `m`.
    Meters,
    /// `cm`.
    Centimeters,
    /// `mm`.
    Millimeters
}

/// A distance value paired with a unit of measure.
pub struct Distance(f32, DistanceUnit);

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

geo_shape_mapping!(
    ElasticPointMapping,
    ElasticPointMappingVisitor,
    PointType
);
geo_multi_shape_mapping!(
    ElasticMultiPointMapping,
    ElasticPointMapping,
    MULTIPOINT_DATATYPE,
    ElasticMultiPointMappingVisitor
);

/// Default mapping for a `point` type.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultPointMapping;
impl ElasticPointMapping for DefaultPointMapping { }
impl_point_mapping!(DefaultPointMapping);
impl ElasticType<DefaultPointMapping, ()> for PointType { }
