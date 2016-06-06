//! Mapping for the Elasticsearch `geo_point` type.

use std::marker::PhantomData;
use serde;
use serde::{ Serializer, Serialize };
use super::{ GeoPointFormat };
use ::mapping::{ ElasticFieldMapping, ElasticTypeVisitor };


/// Elasticsearch datatype name.
pub const GEOPOINT_DATATYPE: &'static str = "geo_point";

/// The base requirements for mapping a `geo_point` type.
///
/// # Examples
///
/// Define a custom `ElasticGeoPointMapping`:
///
/// ## Derive Mapping
///
/// Currently, deriving mapping only works for structs that take a generic `GeoPointFormat` parameter.
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_macros)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// use std::marker::PhantomData;
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::geo::point::prelude::*;
///
/// #[derive(Default, Clone, Copy, ElasticGeoPointMapping)]
/// pub struct MyGeoPointMapping<F: GeoPointFormat> {
/// 	phantom: PhantomData<F>
/// }
/// impl <F: GeoPointFormat> ElasticGeoPointMapping<F> for MyGeoPointMapping<F> {
/// 	//Overload the mapping functions here
/// 	fn geohash() -> Option<bool> {
///			Some(true)
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
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use std::marker::PhantomData;
/// # use elastic_types::mapping::prelude::*;
/// # use elastic_types::geo::point::prelude::*;
/// #
/// # #[derive(Default, Clone, Copy, ElasticGeoPointMapping)]
/// # pub struct MyGeoPointMapping<F: GeoPointFormat> {
/// # 	phantom: PhantomData<F>
/// # }
/// # impl <F: GeoPointFormat> ElasticGeoPointMapping<F> for MyGeoPointMapping<F> {
/// # 	//Overload the mapping functions here
/// # 	fn geohash() -> Option<bool> {
/// #			Some(true)
/// #		}
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&MyGeoPointMapping::<DefaultGeoPointFormat>::default()).unwrap();
/// # let json = json_str!(
/// {
///     "type": "geo_point",
///     "geohash": true
/// }
/// # );
/// # assert_eq!(json, mapping);
/// # }
/// ```
///
/// ## Limitations
///
/// Automatically deriving mapping has the following limitations:
///
/// - Non-generic mappings aren't supported by auto deriving.
/// So your date mapping must take generic parameter `<F: GeoPointFormat>`.
///
/// The above limitation can be worked around by implementing the mapping manually.
///
/// ## Manually
///
/// Define a geo point mapping that's only valid for the `GeoPointString` format:
///
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # use std::marker::PhantomData;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::geo::point::prelude::*;
///
/// #[derive(Default, Clone)]
/// pub struct MyGeoPointMapping;
///
/// impl ElasticFieldMapping<GeoPointString> for MyGeoPointMapping {
/// 	type Visitor = ElasticGeoPointMappingVisitor<GeoPointString, MyGeoPointMapping>;
///
/// 	fn data_type() -> &'static str {
/// 		GEOPOINT_DATATYPE
/// 	}
/// }
///
/// impl ElasticGeoPointMapping<GeoPointString> for MyGeoPointMapping {
/// 	//Overload the mapping functions here
/// 	fn geohash() -> Option<bool> {
///			Some(true)
///		}
/// }
///
/// impl serde::Serialize for MyGeoPointMapping {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
///
/// Define a date mapping that's valid for any `GeoPointFormat` (equivalent to the auto derive example):
///
/// ```
/// # extern crate serde;
/// # extern crate elastic_types;
/// # use std::marker::PhantomData;
/// # fn main() {
/// use elastic_types::mapping::prelude::*;
/// use elastic_types::geo::point::prelude::*;
///
/// #[derive(Default, Clone)]
/// pub struct MyGeoPointMapping<F: GeoPointFormat> {
///     phantom: PhantomData<F>
/// }
///
/// impl <F: GeoPointFormat> ElasticFieldMapping<F> for MyGeoPointMapping<F> {
/// 	type Visitor = ElasticGeoPointMappingVisitor<F, MyGeoPointMapping<F>>;
///
/// 	fn data_type() -> &'static str {
/// 		GEOPOINT_DATATYPE
/// 	}
/// }
///
/// impl <F: GeoPointFormat> ElasticGeoPointMapping<F> for MyGeoPointMapping<F> {
/// 	//Overload the mapping functions here
/// 	fn geohash() -> Option<bool> {
///			Some(true)
///		}
/// }
///
/// impl <F: GeoPointFormat> serde::Serialize for MyGeoPointMapping<F> {
/// 	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
/// 	where S: serde::Serializer {
/// 		serializer.serialize_struct("mapping", Self::get_visitor())
/// 	}
/// }
/// # }
/// ```
pub trait ElasticGeoPointMapping<F> where
F: GeoPointFormat,
Self: ElasticFieldMapping<F> + Sized + Serialize {
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
pub struct DefaultGeoPointMapping<F> where
F: GeoPointFormat {
	phantom: PhantomData<F>
}
impl <F> ElasticGeoPointMapping<F> for DefaultGeoPointMapping<F> where
F: GeoPointFormat { }

impl_geo_point_mapping!(DefaultGeoPointMapping<F>);

/// Visitor for a `geo_point` map.
#[derive(Debug, PartialEq)]
pub struct ElasticGeoPointMappingVisitor<F, M> where
F: GeoPointFormat,
M: ElasticGeoPointMapping<F> {
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<M>
}

impl <F, M> ElasticTypeVisitor for ElasticGeoPointMappingVisitor<F, M> where
F: GeoPointFormat,
M: ElasticGeoPointMapping<F> {
	fn new() -> Self {
		ElasticGeoPointMappingVisitor {
			phantom_f: PhantomData,
			phantom_t: PhantomData
		}
	}
}
impl <F, M> serde::ser::MapVisitor for ElasticGeoPointMappingVisitor<F, M>  where
F: GeoPointFormat,
M: ElasticGeoPointMapping<F> {
	fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
	where S: Serializer {
		try!(serializer.serialize_struct_elt("type", M::data_type()));

        if let Some(geohash) = M::geohash() {
			try!(serializer.serialize_struct_elt("geohash", geohash));
		};

        if let Some(geohash_precision) = M::geohash_precision() {
			try!(serializer.serialize_struct_elt("geohash_precision", geohash_precision));
		};

        if let Some(geohash_prefix) = M::geohash_prefix() {
			try!(serializer.serialize_struct_elt("geohash_prefix", geohash_prefix));
		};

        if let Some(ignore_malformed) = M::ignore_malformed() {
			try!(serializer.serialize_struct_elt("ignore_malformed", ignore_malformed));
		};

        if let Some(lat_lon) = M::lat_lon() {
			try!(serializer.serialize_struct_elt("lat_lon", lat_lon));
		};

        if let Some(precision_step) = M::precision_step() {
			try!(serializer.serialize_struct_elt("precision_step", precision_step));
		};

		Ok(None)
	}
}
