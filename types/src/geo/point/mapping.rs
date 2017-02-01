//! Mapping for the Elasticsearch `geo_point` type.

use std::marker::PhantomData;
use serde::{Serialize, Serializer};
use super::GeoPointFormat;
use ::geo::mapping::Distance;
use ::field::{FieldMapping, SerializeField, Field};


/// Elasticsearch datatype name.
pub const GEOPOINT_DATATYPE: &'static str = "geo_point";

#[doc(hidden)]
#[derive(Default)]
pub struct GeoPointFormatWrapper<F>
    where F: GeoPointFormat
{
    _f: PhantomData<F>,
}

/// The base requirements for mapping a `geo_point` type.
///
/// # Examples
///
/// Define a custom `GeoPointMapping`:
///
/// ## Derive Mapping
///
/// Currently, deriving mapping only works for structs that take a `GeoPointFormat` as an associated type.
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_derive)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use std::marker::PhantomData;
/// # use elastic_types::prelude::*;
/// #[derive(Default)]
/// struct MyGeoPointMapping;
/// impl GeoPointMapping for MyGeoPointMapping {
///     type Format = GeoPointArray;
///
///     //Overload the mapping functions here
///     fn geohash() -> Option<bool> {
///         Some(true)
///     }
/// }
/// # fn main() {}
/// ```
///
/// This will produce the following mapping:
///
/// ```
/// # #[macro_use]
/// # extern crate elastic_types_derive;
/// # #[macro_use]
/// # extern crate json_str;
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # extern crate serde_json;
/// # use std::marker::PhantomData;
/// # use elastic_types::prelude::*;
/// # #[derive(Default)]
/// # struct MyGeoPointMapping;
/// # impl GeoPointMapping for MyGeoPointMapping {
/// #     type Format = GeoPointArray;
/// #     fn geohash() -> Option<bool> {
/// #         Some(true)
/// #     }
/// # }
/// # fn main() {
/// # let mapping = serde_json::to_string(&Field::from(MyGeoPointMapping)).unwrap();
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
/// ## Map with a generic Format
///
/// You can use a generic input parameter to make your `GeoPointMapping` work for any kind of
/// `GeoPointFormat`:
///
/// ```
/// # #![feature(plugin, custom_derive, custom_attribute)]
/// # #![plugin(json_str, elastic_types_derive)]
/// # #[macro_use]
/// # extern crate elastic_types;
/// # extern crate serde;
/// # use std::marker::PhantomData;
/// # use elastic_types::prelude::*;
/// #[derive(Default)]
/// struct MyGeoPointMapping<F> {
///     _marker: PhantomData<F>
/// }
///
/// impl <F: GeoPointFormat> GeoPointMapping for MyGeoPointMapping<F> {
///     type Format = F;
/// }
/// # fn main() {}
/// ```
pub trait GeoPointMapping
    where Self: Default
{
    /// The format used to serialise and deserialise the geo point.
    ///
    /// The format isn't actually a part of the Elasticsearch mapping for a `geo_point`,
    /// but is included on the mapping to keep things consistent.
    type Format: GeoPointFormat;

    /// Should the `geo-point` also be indexed as a geohash in the `.geohash` sub-field? Defaults to `false`,
    /// unless `geohash_prefix` is `true`.
    fn geohash() -> Option<bool> {
        None
    }

    /// The maximum length of the geohash to use for the geohash and `geohash_prefix` options.
    fn geohash_precision() -> Option<Distance> {
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
}

impl<T, F> FieldMapping<GeoPointFormatWrapper<F>> for T
    where T: GeoPointMapping<Format = F>,
          F: GeoPointFormat
{
    fn data_type() -> &'static str {
        GEOPOINT_DATATYPE
    }
}

impl<T, F> SerializeField<GeoPointFormatWrapper<F>> for T
    where T: GeoPointMapping<Format = F>,
          F: GeoPointFormat
{
    type Field = Field<T, GeoPointFormatWrapper<F>>;
}

impl<T, F> Serialize for Field<T, GeoPointFormatWrapper<F>>
    where T: FieldMapping<GeoPointFormatWrapper<F>> + GeoPointMapping<Format = F>,
          F: GeoPointFormat
{
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 6));

        try!(serializer.serialize_struct_elt(&mut state, "type", T::data_type()));

        ser_field!(serializer, &mut state, "geohash", T::geohash());
        ser_field!(serializer,
                   &mut state,
                   "geohash_precision",
                   T::geohash_precision());
        ser_field!(serializer,
                   &mut state,
                   "geohash_prefix",
                   T::geohash_prefix());
        ser_field!(serializer,
                   &mut state,
                   "ignore_malformed",
                   T::ignore_malformed());
        ser_field!(serializer, &mut state, "lat_lon", T::lat_lon());

        serializer.serialize_struct_end(state)
    }
}

/// Default mapping for `geo_point`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultGeoPointMapping<F>
    where F: GeoPointFormat
{
    _f: PhantomData<F>,
}

impl<F> GeoPointMapping for DefaultGeoPointMapping<F>
    where F: GeoPointFormat
{
    type Format = F;
}
