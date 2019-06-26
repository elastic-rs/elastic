/*! Mapping for Elasticsearch `geo_shape` types. */

use crate::types::geo::mapping::Distance;
use serde::{
    Serialize,
    Serializer,
};

/** A field that will be mapped as a `geo_shape`. */
pub trait GeoShapeFieldType<TMapping> {}

/**
The base requirements for mapping a `geo_shape` type.

Custom mappings can be defined by implementing `GeoShapeMapping`.

# Examples

Define a custom `GeoShapeMapping`:

```
# #[macro_use] use elastic::types::prelude::*;
#[derive(Default)]
struct MyGeoShapeMapping;
impl GeoShapeMapping for MyGeoShapeMapping {
    //Overload the mapping functions here
    fn tree_levels() -> Option<i32> {
        Some(2)
    }
}
# fn main() {}
```

This will produce the following mapping:

```
# #[macro_use] extern crate serde_json;
# use elastic::types::prelude::*;
#[derive(Default)]
# struct MyGeoShapeMapping;
# impl GeoShapeMapping for MyGeoShapeMapping {
#     //Overload the mapping functions here
#     fn tree_levels() -> Option<i32> {
#         Some(2)
#     }
# }
# fn main() {
# let mapping = elastic::types::__derive::standalone_field_ser(MyGeoShapeMapping).unwrap();
# let json = json!(
{
    "type": "geo_shape",
    "tree_levels": 2
}
# );
# assert_eq!(json, mapping);
# }
```
*/
pub trait GeoShapeMapping {
    /**
    Name of the PrefixTree implementation to be used:
    `geohash` for `GeohashPrefixTree` and `quadtree` for `QuadPrefixTree`.
    */
    fn tree() -> Option<Tree> {
        None
    }

    /**
    This parameter may be used instead of `tree_levels` to set an appropriate value
    for the `tree_levels` parameter.
    The value specifies the desired precision and Elasticsearch will calculate the best
    `tree_levels` value to honor this precision.
    The value should be a number followed by an optional distance unit.
    */
    fn precision() -> Option<Distance> {
        None
    }

    /**
    Maximum number of layers to be used by the `PrefixTree`.
    This can be used to control the precision of shape representations and therefore
    how many terms are indexed.
    Defaults to the default value of the chosen `PrefixTree` implementation.
    Since this parameter requires a certain level of understanding of the underlying implementation,
    users may use the `precision` parameter instead.
    However, Elasticsearch only uses the `tree_levels` parameter internally and this is
    what is returned via the mapping API even if you use the `precision` parameter.
    */
    fn tree_levels() -> Option<i32> {
        None
    }

    /**
    The `strategy` parameter defines the approach for how to represent shapes at indexing and search time.
    It also influences the capabilities available so it is recommended to let Elasticsearch
    set this parameter automatically.
    There are two strategies available: `recursive` and `term`.
    Term strategy supports point types only (the `points_only` parameter will be automatically set to `true`)
    while `Recursive` strategy supports all shape types.
    */
    fn strategy() -> Option<Strategy> {
        None
    }

    /**
    Used as a hint to the `PrefixTree` about how precise it should be.
    Defaults to `0.025` (2.5%) with `0.5` as the maximum supported value.

    > PERFORMANCE NOTE: This value will default to `0` if a `precision` or `tree_level` definition is explicitly defined.
    This guarantees spatial precision at the level defined in the mapping.
    This can lead to significant memory usage for high resolution shapes with low error
    (e.g., large shapes at `1m` with < `0.001` error).
    To improve indexing performance (at the cost of query accuracy) explicitly define `tree_level`
    or `precision` along with a reasonable `distance_error_pct`,
    noting that large shapes will have greater false positives.
    */
    fn distance_error_pct() -> Option<f32> {
        None
    }

    /**
    Setting this parameter in the `geo_shape` mapping explicitly sets vertex order for
    the coordinate list of a `geo_shape` field but can be overridden in each individual
    GeoJSON document.
    */
    fn orientation() -> Option<Orientation> {
        None
    }

    /**
    Setting this option to `true` (defaults to `false`) configures the `geo_shape` field
    type for point shapes only (NOTE: Multi-Points are not yet supported).
    This optimizes index and search performance for the geohash and quadtree when it is
    known that only points will be indexed.
    At present `geo_shape` queries can not be executed on geo_point field types.
    This option bridges the gap by improving point performance on a `geo_shape` field
    so that geo_shape queries are optimal on a point only field.
    */
    fn points_only() -> Option<bool> {
        None
    }
}

/** Default mapping for `geo_shape`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultGeoShapeMapping;
impl GeoShapeMapping for DefaultGeoShapeMapping {}

/** Name of the `PrefixTree` implementation to be used. */
pub enum Tree {
    /** For `GeohashPrefixTree`. */
    Geohash,
    /** For `QuadPrefixTree`. */
    QuadPrefix,
}

impl Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Tree::Geohash => "geohash",
            Tree::QuadPrefix => "quadtree",
        })
    }
}

/** The strategy defines the approach for how to represent shapes at indexing and search time. */
pub enum Strategy {
    /** Recursive strategy supports all shape types. */
    Recursive,
    /** Term strategy supports point types only. */
    Term,
}

impl Serialize for Strategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Strategy::Recursive => "recursive",
            Strategy::Term => "term",
        })
    }
}

/**
This parameter defines one of two coordinate system rules (Right-hand or Left-hand)
each of which can be specified in a few different ways.
- Right-hand rule: right, ccw, counterclockwise,
- Left-hand rule: left, cw, clockwise.
The default orientation (counterclockwise) complies with the OGC standard which defines outer
ring vertices in counterclockwise order with inner ring(s) vertices (holes) in clockwise order.
*/
pub enum Orientation {
    /** For `cw`. */
    Clockwise,
    /** For `ccw`. */
    CounterClockwise,
}

impl Serialize for Orientation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Orientation::Clockwise => "cw",
            Orientation::CounterClockwise => "ccw",
        })
    }
}

mod private {
    use super::{
        GeoShapeFieldType,
        GeoShapeMapping,
    };
    use crate::types::private::field::{
        FieldMapping,
        FieldType,
        SerializeFieldMapping,
        StaticSerialize,
    };
    use serde::{
        ser::SerializeStruct,
        Serialize,
        Serializer,
    };

    impl<TField, TMapping> FieldType<TMapping, GeoShapePivot> for TField
    where
        TField: GeoShapeFieldType<TMapping> + Serialize,
        TMapping: GeoShapeMapping,
    {
    }

    #[derive(Default)]
    pub struct GeoShapePivot;

    impl<TMapping> FieldMapping<GeoShapePivot> for TMapping
    where
        TMapping: GeoShapeMapping,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, GeoShapePivot>;

        fn data_type() -> &'static str {
            "geo_shape"
        }
    }

    impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, GeoShapePivot>
    where
        TMapping: FieldMapping<GeoShapePivot> + GeoShapeMapping,
    {
        fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("mapping", 8)?;

            state.serialize_field("type", TMapping::data_type())?;

            ser_field!(state, "tree", TMapping::tree());
            ser_field!(state, "precision", TMapping::precision());
            ser_field!(state, "tree_levels", TMapping::tree_levels());
            ser_field!(state, "strategy", TMapping::strategy());
            ser_field!(state, "distance_error_pct", TMapping::distance_error_pct());
            ser_field!(state, "orientation", TMapping::orientation());
            ser_field!(state, "points_only", TMapping::points_only());

            state.end()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::{
        prelude::*,
        private::field,
    };

    #[derive(Default, Clone)]
    pub struct MyGeoShapeMapping;
    impl GeoShapeMapping for MyGeoShapeMapping {
        fn tree() -> Option<Tree> {
            Some(Tree::Geohash)
        }

        fn precision() -> Option<Distance> {
            Some(Distance(50.0, DistanceUnit::Meters))
        }

        fn tree_levels() -> Option<i32> {
            Some(8)
        }

        fn strategy() -> Option<Strategy> {
            Some(Strategy::Recursive)
        }

        fn distance_error_pct() -> Option<f32> {
            Some(0.5)
        }

        fn orientation() -> Option<Orientation> {
            Some(Orientation::Clockwise)
        }

        fn points_only() -> Option<bool> {
            Some(false)
        }
    }

    #[test]
    fn serialise_mapping_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultGeoShapeMapping)).unwrap();

        let expected = json!({
            "type": "geo_shape"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_custom() {
        let ser = serde_json::to_value(&field::serialize(MyGeoShapeMapping)).unwrap();

        let expected = json!({
            "type": "geo_shape",
            "tree": "geohash",
            "precision": "50m",
            "tree_levels": 8,
            "strategy": "recursive",
            "distance_error_pct": 0.5,
            "orientation": "cw",
            "points_only": false
        });

        assert_eq!(expected, ser);
    }

}
