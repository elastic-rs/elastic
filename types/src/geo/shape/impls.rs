use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use geojson::Geometry;
use super::mapping::{GeoShapeFieldType, GeoShapeMapping, DefaultGeoShapeMapping};

/**
Geo shape type with a given mapping.

Defining a `geo_shape` with a mapping:

```
# extern crate elastic_types;
extern crate geojson;
use geojson::{ Geometry, Value };

# use elastic_types::prelude::*;
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
pub struct GeoShape<M = DefaultGeoShapeMapping>
    where M: GeoShapeMapping
{
    value: Geometry,
    _m: PhantomData<M>,
}

impl<M> GeoShape<M>
    where M: GeoShapeMapping
{
    /**
    Creates a new `GeoShape` from the given `Geometry`.
    
    This function will consume the provided `Geometry`.
    
    # Examples
    
    ```
    # extern crate elastic_types;
    # extern crate geojson;
    use geojson::{ Geometry, Value };
    
    # use elastic_types::prelude::*;
    # fn main() {
    let point: GeoShape = GeoShape::new(
        Geometry::new(
            Value::Point(vec![ 1.0, 1.0 ])
        )
    );
    # }
    ```
    */
    pub fn new<I>(geo: I) -> GeoShape<M>
        where I: Into<Geometry>
    {
        GeoShape {
            value: geo.into(),
            _m: PhantomData,
        }
    }

    /** Change the mapping of this geo shape. */
    pub fn remap<MInto: GeoShapeMapping>(self) -> GeoShape<MInto> {
        GeoShape::<MInto>::new(self.value)
    }
}

impl<M> GeoShapeFieldType<M> for GeoShape<M> where M: GeoShapeMapping {}

impl_mapping_type!(Geometry, GeoShape, GeoShapeMapping);

impl<M> Serialize for GeoShape<M>
    where M: GeoShapeMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        self.value.serialize(serializer)
    }
}

impl<'de, M> Deserialize<'de> for GeoShape<M>
    where M: GeoShapeMapping
{
    fn deserialize<D>(deserializer: D) -> Result<GeoShape<M>, D::Error>
        where D: Deserializer<'de>
    {
        let t = try!(Geometry::deserialize(deserializer));

        Ok(GeoShape::<M>::new(t))
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use geojson::{Geometry, Value};
    use prelude::*;

    #[derive(Default)]
    struct MyGeoShapeMapping;
    impl GeoShapeMapping for MyGeoShapeMapping {}

    #[test]
    fn can_change_geo_shape_mapping() {
        fn takes_custom_mapping(_: GeoShape<MyGeoShapeMapping>) -> bool {
            true
        }

        let point: GeoShape<DefaultGeoShapeMapping> = GeoShape::new(Geometry::new(Value::Point(vec![ 1.0, 1.0 ])));

        assert!(takes_custom_mapping(point.remap()));
    }

    #[test]
    fn serialise_elastic_geo_shape() {
        let shape = GeoShape::<DefaultGeoShapeMapping>::new(Geometry::new(Value::Point(vec![ 1.0, 1.0 ])));

        let ser = serde_json::to_string(&shape).unwrap();

        assert_eq!(json_str!({
            "coordinates": [ 1.0, 1.0 ],
            "type": "Point"
        }), ser);
    }

    #[test]
    fn deserialise_elastic_geo_shape() {
        let shape: GeoShape<DefaultGeoShapeMapping> = serde_json::from_str(&json_str!({
            "coordinates": [ 1, 1 ],
            "type": "Point"
        }))
            .unwrap();

        assert_eq!(
            Geometry::new(
                Value::Point(vec![ 1.0, 1.0 ])),
            *shape
        );
    }

}
