/*! Mapping for the Elasticsearch `geo_point` type. */

use std::marker::PhantomData;
use super::{DefaultGeoPointFormat, GeoPointFormat};
use geo::mapping::Distance;

/** A field that will be mapped as a `geo_point`. */
pub trait GeoPointFieldType<M> {}

/**
The base requirements for mapping a `geo_point` type.

# Examples

Define a custom `GeoPointMapping`:

```
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use std::marker::PhantomData;
# use elastic_types::prelude::*;
#[derive(Default)]
struct MyGeoPointMapping;
impl GeoPointMapping for MyGeoPointMapping {
    type Format = GeoPointArray;

    //Overload the mapping functions here
    fn geohash() -> Option<bool> {
        Some(true)
    }
}
# fn main() {}
```

This will produce the following mapping:

```
# #[macro_use]
# extern crate elastic_types_derive;
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# extern crate serde_json;
# use std::marker::PhantomData;
# use elastic_types::prelude::*;
# #[derive(Default)]
# struct MyGeoPointMapping;
# impl GeoPointMapping for MyGeoPointMapping {
#     type Format = GeoPointArray;
#     fn geohash() -> Option<bool> {
#         Some(true)
#     }
# }
# fn main() {
# let mapping = elastic_types::derive::standalone_field_ser(MyGeoPointMapping).unwrap();
# let json = json_str!(
{
    "type": "geo_point",
    "geohash": true
}
# );
# assert_eq!(json, mapping);
# }
```

## Map with a generic Format

You can use a generic input parameter to make your `GeoPointMapping` work for any kind of
`GeoPointFormat`:

```
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use std::marker::PhantomData;
# use elastic_types::prelude::*;
#[derive(Default)]
struct MyGeoPointMapping<F> {
    _marker: PhantomData<F>
}

impl <F: GeoPointFormat> GeoPointMapping for MyGeoPointMapping<F> {
    type Format = F;
}
# fn main() {}
```
*/
pub trait GeoPointMapping {
    /**
    The format used to serialise and deserialise the geo point.

    The format isn't actually a part of the Elasticsearch mapping for a `geo_point`,
    but is included on the mapping to keep things consistent.
    */
    type Format: GeoPointFormat;

    /**
    Should the `geo-point` also be indexed as a geohash in the `.geohash` sub-field? Defaults to `false`,
    unless `geohash_prefix` is `true`.
    */
    fn geohash() -> Option<bool> {
        None
    }

    /** The maximum length of the geohash to use for the geohash and `geohash_prefix` options. */
    fn geohash_precision() -> Option<Distance> {
        None
    }

    /** Should the `geo-point` also be indexed as a geohash plus all its prefixes? Defaults to `false`. */
    fn geohash_prefix() -> Option<bool> {
        None
    }

    /**
    If `true`, malformed `geo-points` are ignored.
    If `false` (default), malformed `geo-points` throw an exception and reject the whole document.
    */
    fn ignore_malformed() -> Option<bool> {
        None
    }

    /**
    Should the `geo-point` also be indexed as `.lat` and `.lon` sub-fields?
    Accepts `true` and `false` (default).
    */
    fn lat_lon() -> Option<bool> {
        None
    }
}

/** Default mapping for `geo_point`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultGeoPointMapping<TFormat = DefaultGeoPointFormat>
where
    TFormat: GeoPointFormat,
{
    _f: PhantomData<TFormat>,
}

impl<TFormat> GeoPointMapping for DefaultGeoPointMapping<TFormat>
where
    TFormat: GeoPointFormat,
{
    type Format = TFormat;
}

mod private {
    use serde::{Serialize, Serializer};
    use serde::ser::SerializeStruct;
    use private::field::{StaticSerialize, SerializeFieldMapping, FieldMapping, FieldType};
    use super::{GeoPointFieldType, GeoPointMapping};

    #[derive(Default)]
    pub struct GeoPointPivot;

    impl<TField, TMapping> FieldType<TMapping, GeoPointPivot> for TField
    where
        TField: GeoPointFieldType<TMapping> + Serialize,
        TMapping: GeoPointMapping,
    {
    }

    impl<TMapping> FieldMapping<GeoPointPivot> for TMapping
    where
        TMapping: GeoPointMapping,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, GeoPointPivot>;

        fn data_type() -> &'static str {
            "geo_point"
        }
    }

    impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, GeoPointPivot>
    where
        TMapping: FieldMapping<GeoPointPivot> + GeoPointMapping,
    {
        fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = try!(serializer.serialize_struct("mapping", 6));

            try!(state.serialize_field("type", TMapping::data_type()));

            ser_field!(state, "geohash", TMapping::geohash());
            ser_field!(state, "geohash_precision", TMapping::geohash_precision());
            ser_field!(state, "geohash_prefix", TMapping::geohash_prefix());
            ser_field!(state, "ignore_malformed", TMapping::ignore_malformed());
            ser_field!(state, "lat_lon", TMapping::lat_lon());

            state.end()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use prelude::*;
    use private::field;

    #[derive(Default, Clone)]
    pub struct MyGeoPointMapping;
    impl GeoPointMapping for MyGeoPointMapping {
        type Format = GeoPointArray;

        fn geohash() -> Option<bool> {
            Some(false)
        }

        fn geohash_precision() -> Option<Distance> {
            Some(Distance(50.0, DistanceUnit::Meters))
        }

        fn geohash_prefix() -> Option<bool> {
            Some(true)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }

        fn lat_lon() -> Option<bool> {
            Some(true)
        }
    }

    #[test]
    fn serialise_mapping_default() {
        let ser = serde_json::to_string(&field::serialize(DefaultGeoPointMapping::<
            DefaultGeoPointFormat,
        >::default()))
            .unwrap();

        let expected = json_str!({
            "type": "geo_point"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_custom() {
        let ser = serde_json::to_string(&field::serialize(MyGeoPointMapping)).unwrap();

        let expected = json_str!({
            "type": "geo_point",
            "geohash": false,
            "geohash_precision": "50m",
            "geohash_prefix": true,
            "ignore_malformed": true,
            "lat_lon": true
        });

        assert_eq!(expected, ser);
    }

}
