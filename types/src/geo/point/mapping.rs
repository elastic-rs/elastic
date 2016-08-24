//! Mapping for the Elasticsearch `geo_point` type.

use std::marker::PhantomData;
use serde::Serialize;
use super::GeoPointFormat;
use ::geo::mapping::Distance;
use ::mapping::ElasticFieldMapping;


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
/// # use std::marker::PhantomData;
/// # use elastic_types::prelude::*;
/// geo_point_mapping!(MyGeoPointMapping {
///     //Overload the mapping functions here
///     fn geohash() -> Option<bool> {
///			Some(true)
///		}
/// });
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
/// # use std::marker::PhantomData;
/// # use elastic_types::prelude::*;
/// # geo_point_mapping!(MyGeoPointMapping {
/// # 	//Overload the mapping functions here
/// # 	fn geohash() -> Option<bool> {
/// # 		Some(true)
/// # 	}
/// # });
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
pub trait ElasticGeoPointMapping<F> where
F: GeoPointFormat,
Self: ElasticFieldMapping<F> + Sized + Serialize {
	/// Should the `geo-point` also be indexed as a geohash in the `.geohash` sub-field? Defaults to `false`,
	/// unless `geohash_prefix` is `true`.
	fn geohash() -> Option<bool> { None }

	/// The maximum length of the geohash to use for the geohash and `geohash_prefix` options.
	fn geohash_precision() -> Option<Distance> { None }

	/// Should the `geo-point` also be indexed as a geohash plus all its prefixes? Defaults to `false`.
	fn geohash_prefix() -> Option<bool> { None }

	/// If `true`, malformed `geo-points` are ignored.
	/// If `false` (default), malformed `geo-points` throw an exception and reject the whole document.
	fn ignore_malformed() -> Option<bool> { None }

	/// Should the `geo-point` also be indexed as `.lat` and `.lon` sub-fields?
	/// Accepts `true` and `false` (default).
	fn lat_lon() -> Option<bool> { None }
}

/// Implement `serde` serialisation for a `date` mapping type.
#[macro_export]
macro_rules! geo_point_ser {
	($t:ident: $f:ident) => (
		impl <$f: $crate::geo::point::GeoPointFormat> ::serde::Serialize for $t<$f> {
			fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where 
			S: ::serde::Serializer {
				let mut state = try!(serializer.serialize_struct("mapping", 7));

				try!(serializer.serialize_struct_elt(&mut state, "type", $t::<$f>::data_type()));

				ser_field!(serializer, &mut state, $t::<$f>::geohash(), "geohash");
				ser_field!(serializer, &mut state, $t::<$f>::geohash_precision(), "geohash_precision");
				ser_field!(serializer, &mut state, $t::<$f>::geohash_prefix(), "geohash_prefix");
				ser_field!(serializer, &mut state, $t::<$f>::ignore_malformed(), "ignore_malformed");
				ser_field!(serializer, &mut state, $t::<$f>::lat_lon(), "lat_lon");

				serializer.serialize_struct_end(state)
			}
		}
	)
}

/// Default mapping for `geo_point`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultGeoPointMapping<F> where
F: 'static + GeoPointFormat {
	_marker: PhantomData<F>
}
impl <F: 'static + GeoPointFormat> ElasticGeoPointMapping<F> for DefaultGeoPointMapping<F> { }

geo_point_mapping!(DefaultGeoPointMapping: F);