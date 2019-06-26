use super::mapping::{
    GeoShapeFieldType,
    GeoShapeMapping,
};
use geojson::Geometry;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{
    borrow::Borrow,
    marker::PhantomData,
};

/**
Geo shape type with a given mapping.

Defining a `geo_shape` with a mapping:

```
extern crate geojson;
use geojson::{ Geometry, Value };

# use elastic::types::prelude::*;
# fn main() {
let point: GeoShape<DefaultGeoShapeMapping> = GeoShape::new(
    Geometry::new(
        Value::Point(vec![ 1.0, 1.0 ])
    )
);
# }
```
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GeoShape<TMapping>
where
    TMapping: GeoShapeMapping,
{
    value: Geometry,
    _m: PhantomData<TMapping>,
}

impl<TMapping> GeoShape<TMapping>
where
    TMapping: GeoShapeMapping,
{
    /**
    Creates a new `GeoShape` from the given `Geometry`.

    This function will consume the provided `Geometry`.

    # Examples

    ```
    # extern crate geojson;
    use geojson::{ Geometry, Value };

    # use elastic::types::prelude::*;
    # fn main() {
    let point: GeoShape<DefaultGeoShapeMapping> = GeoShape::new(
        Geometry::new(
            Value::Point(vec![ 1.0, 1.0 ])
        )
    );
    # }
    ```
    */
    pub fn new<I>(geo: I) -> GeoShape<TMapping>
    where
        I: Into<Geometry>,
    {
        GeoShape {
            value: geo.into(),
            _m: PhantomData,
        }
    }

    /** Change the mapping of this geo shape. */
    pub fn remap<TNewMapping>(shape: GeoShape<TMapping>) -> GeoShape<TNewMapping>
    where
        TNewMapping: GeoShapeMapping,
    {
        GeoShape::new(shape.value)
    }
}

impl<TMapping> GeoShapeFieldType<TMapping> for GeoShape<TMapping> where TMapping: GeoShapeMapping {}

impl_mapping_type!(Geometry, GeoShape, GeoShapeMapping);

impl<TMapping> Serialize for GeoShape<TMapping>
where
    TMapping: GeoShapeMapping,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de, TMapping> Deserialize<'de> for GeoShape<TMapping>
where
    TMapping: GeoShapeMapping,
{
    fn deserialize<D>(deserializer: D) -> Result<GeoShape<TMapping>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let t = Geometry::deserialize(deserializer)?;

        Ok(GeoShape::new(t))
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::prelude::*;
    use geojson::{
        Geometry,
        Value,
    };

    #[derive(Default)]
    struct MyGeoShapeMapping;
    impl GeoShapeMapping for MyGeoShapeMapping {}

    #[test]
    fn can_change_geo_shape_mapping() {
        fn takes_custom_mapping(_: GeoShape<MyGeoShapeMapping>) -> bool {
            true
        }

        let point: GeoShape<DefaultGeoShapeMapping> =
            GeoShape::new(Geometry::new(Value::Point(vec![1.0, 1.0])));

        assert!(takes_custom_mapping(GeoShape::remap(point)));
    }

    #[test]
    fn serialise_elastic_geo_shape() {
        let shape =
            GeoShape::<DefaultGeoShapeMapping>::new(Geometry::new(Value::Point(vec![1.0, 1.0])));

        let ser = serde_json::to_value(&shape).unwrap();

        assert_eq!(
            json!({
                "coordinates": [ 1.0, 1.0 ],
                "type": "Point"
            }),
            ser
        );
    }

    #[test]
    fn deserialise_elastic_geo_shape() {
        let shape: GeoShape<DefaultGeoShapeMapping> = serde_json::from_value(json!({
            "coordinates": [ 1, 1 ],
            "type": "Point"
        }))
        .unwrap();

        assert_eq!(Geometry::new(Value::Point(vec![1.0, 1.0])), *shape);
    }

}
