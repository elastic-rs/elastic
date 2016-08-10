//! Mapping for the Elasticsearch `geo_point` type.

use std::marker::PhantomData;
use serde;
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
macro_rules! geo_point_ser {
    ($t:ident: $f:ident) => (
        impl <$f: $crate::geo::point::GeoPointFormat> serde::Serialize for $t<$f> {
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer {
                let mut state = try!(serializer.serialize_struct("mapping", 7));

                try!(serializer.serialize_struct_elt(&mut state, "type", $t::<$f>::data_type()));

                ser_field!(serializer, &mut state, $t::<$f>::geohash(), "geohash");
                ser_field!(serializer, &mut state, $t::<$f>::geohash_precision(), "geohash_precision");
                ser_field!(serializer, &mut state, $t::<$f>::geohash_prefix(), "geohash_prefix");
                ser_field!(serializer, &mut state, $t::<$f>::ignore_malformed(), "ignore_malformed");
                ser_field!(serializer, &mut state, $t::<$f>::ignore_malformed(), "ignore_malformed");
                ser_field!(serializer, &mut state, $t::<$f>::lat_lon(), "lat_lon");

                serializer.serialize_struct_end(state)
            }
        }
    )
}

/// Define a `date` mapping for all formats.
/// 
/// The easiest way to define a mapping type is to let the macro do it for you:
/// 
/// ```
/// date_mapping!(MyMapping {
///     fn boost() -> Option<f32> { Some(1.03) }
/// });
/// ```
/// 
/// The above example will define a public struct for you and implement
/// `ElasticFieldMapping<F: DateFormat>` and `ElasticDateMapping<F: DateFormat>`, along with a few default traits:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping<F: DateFormat> {
///     _marker: PhantomData<F>
/// }
/// ```
/// 
/// If you want to control the default implementations yourself, you can define your
/// mapping type and just pass it the macro to implement `ElasticFieldMapping<F>`:
/// 
/// ```
/// #[derive(Debug, Default, Clone, Copy)]
/// pub struct MyMapping<F: 'static + DateFormat> {
///     _marker: PhantomData<F>
/// }
/// impl <F: 'static + DateFormat> ElasticDateMapping<F> for MyMapping<F> { }
/// date_mapping_all!(MyMapping: F);
/// ```
macro_rules! geo_point_mapping {
    ($t:ident: $f:ident) => (
        impl <$f: 'static + $crate::geo::point::GeoPointFormat> $crate::mapping::ElasticFieldMapping<F> for $t<$f> {
            fn data_type() -> &'static str { $crate::geo::point::mapping::GEOPOINT_DATATYPE }
        }

        geo_point_ser!($t: $f);
    );
    ($t:ident $b:tt) => (
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $t<F: 'static + $crate::geo::point::GeoPointFormat> {
            _marker: PhantomData<F>
        }

        impl <F: 'static + $crate::geo::point::GeoPointFormat> $crate::mapping::ElasticFieldMapping<F> for $t<F> {
            fn data_type() -> &'static str { $crate::date::mapping::GEOPOINT_DATATYPE }
        }

        impl <F: 'static + $crate::geo::point::GeoPointFormat> $crate::geo::point::mapping::ElasticGeoPointMapping<F> for $t<F> $b

        geo_point_ser!($t: F);
    )
}

/// Default mapping for `geo_point`.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultGeoPointMapping<F> where
F: 'static + GeoPointFormat {
    _marker: PhantomData<F>
}
impl <F: 'static + GeoPointFormat> ElasticGeoPointMapping<F> for DefaultGeoPointMapping<F> { }

geo_point_mapping!(DefaultGeoPointMapping: F);