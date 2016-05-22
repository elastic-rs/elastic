//! Mapping for the Elasticsearch `geo_point` type.

use std::marker::PhantomData;
use serde;
use serde::{ Serializer, Serialize };
use super::{ GeoPointFormat };
use ::mapping::{ ElasticFieldMapping, ElasticTypeVisitor };


/// Elasticsearch datatype name.
pub const GEOPOINT_TYPE: &'static str = "geo_point";

/// The base requirements for mapping a `geo_point` type.
///
/// # Examples
///
/// Define a custom `ElasticGeoPointMapping`:
///
/// ## Derive Mapping
///
/// ```
/// //TODO: implement
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// //TODO: implement
/// ```
///
/// ## Manually
///
/// ```
/// //TODO: implement
/// ```
pub trait ElasticGeoPointMapping<T> where
T: GeoPointFormat,
Self: ElasticFieldMapping<T> + Sized + Serialize {
    /// Should the `geo-point` also be indexed as a geohash in the `.geohash` sub-field? Defaults to `false`,
    /// unless `geohash_prefix` is `true`.
    fn geohash() -> Option<bool> {
        None
    }

    /// The maximum length of the geohash to use for the geohash and `geohash_prefix` options.
    fn geohash_precision() -> Option<u8> {
        None
    }

    /// Should the `geo-point` also be indexed as a geohash plus all its prefixes? Defaults to `false`.
    fn geohash_prefix() -> Option<bool> {
        None
    }

    /// If `true`, malformed `geo-points` are ignored.
    /// If `false` (default), malformed `geo-points` throw an exception and reject the whole document.
    fn ignore_malformed() -> Option<bool> {
        None
    }

    /// Should the `geo-point` also be indexed as `.lat` and `.lon` sub-fields?
    /// Accepts `true` and `false` (default).
    fn lat_lon() -> Option<bool> {
        None
    }

    /// Controls the number of extra terms that are indexed for each lat/lon point. Defaults to `16`.
    /// Ignored if `lat_lon` is `false`.
    fn precision_step() -> Option<i32> {
        None
    }
}

/// Default mapping for `ElasticGeoPoint`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultGeoPointMapping<T> where
T: GeoPointFormat {
	phantom: PhantomData<T>
}
impl <T> ElasticGeoPointMapping<T> for DefaultGeoPointMapping<T> where
T: GeoPointFormat { }

impl_geo_point_mapping!(DefaultGeoPointMapping<T>);

/// Visitor for a `geo_point` map.
#[derive(Debug, PartialEq)]
pub struct ElasticGeoPointMappingVisitor<F, T> where
F: GeoPointFormat,
T: ElasticGeoPointMapping<F> {
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticTypeVisitor for ElasticGeoPointMappingVisitor<F, T> where
F: GeoPointFormat,
T: ElasticGeoPointMapping<F> {
	fn new() -> Self {
		ElasticGeoPointMappingVisitor {
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}
}
impl <F, T> serde::ser::MapVisitor for ElasticGeoPointMappingVisitor<F, T>  where
F: GeoPointFormat,
T: ElasticGeoPointMapping<F> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", T::data_type()));

        if let Some(geohash) = T::geohash() {
			try!(serializer.serialize_struct_elt("geohash", geohash));
		};

        if let Some(geohash_precision) = T::geohash_precision() {
			try!(serializer.serialize_struct_elt("geohash_precision", geohash_precision));
		};

        if let Some(geohash_prefix) = T::geohash_prefix() {
			try!(serializer.serialize_struct_elt("geohash_prefix", geohash_prefix));
		};

        if let Some(ignore_malformed) = T::ignore_malformed() {
			try!(serializer.serialize_struct_elt("ignore_malformed", ignore_malformed));
		};

        if let Some(lat_lon) = T::lat_lon() {
			try!(serializer.serialize_struct_elt("lat_lon", lat_lon));
		};

        if let Some(precision_step) = T::precision_step() {
			try!(serializer.serialize_struct_elt("precision_step", precision_step));
		};

		Ok(None)
	}
}
