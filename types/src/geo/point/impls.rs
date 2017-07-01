use std::marker::PhantomData;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use georust::{ToGeo, Geometry as GeoEnum};
use super::mapping::{GeoPointFieldType, GeoPointMapping};
use super::{Coordinate, Point, Geometry, GeoPointFormat};

/**
An Elasticsearch `geo_point` type with a format.

The [format](format/index.html) is provided as a generic parameter.
This struct wraps up a `geo::Point` struct, which have an `x` and `y` floating point value.

# Examples

Defining a geo point using the default format:

```
# use elastic_types::prelude::*;
let point: GeoPoint<DefaultGeoPointMapping> = GeoPoint::build(1.0, 1.0);
```

Defining a geo point using a named format:

```
# use elastic_types::prelude::*;
let point: GeoPoint<DefaultGeoPointMapping<GeoPointString>> = GeoPoint::build(1.0, 1.0);
```

Accessing the values of a geo point:

```
# use elastic_types::prelude::*;
let point: GeoPoint<DefaultGeoPointMapping> = GeoPoint::build(1.0, 1.0);

//eg: (1.0,1.0)
println!("({},{})",
        point.x(),
    point.y()
);
```

# Links
- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/geo-point.html)
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GeoPoint<M>  where M: GeoPointMapping {
    value: Point,
    _m: PhantomData<M>,
}

impl<M> GeoPoint<M>
    where M: GeoPointMapping
{
    /**
    Creates a new `GeoPoint` from the given coordinate.
    
    This function will consume the provided `Coordinate`.
    
    # Examples
    
    ```
    # extern crate elastic_types;
    # extern crate geo;
    # fn main() {
    use geo::{ Point, Coordinate };
    use elastic_types::prelude::*;
    
    //Create a geo Coordinate struct
    let coord = Coordinate { x: 1.0, y: 1.0 };
    
    //Give it to the GeoPoint struct
    let point: GeoPoint<DefaultGeoPointMapping> = GeoPoint::new(Point(coord));
    # }
    ```
    */
    pub fn new<I>(point: I) -> Self
        where I: Into<Point>
    {
        GeoPoint {
            value: point.into(),
            _m: PhantomData,
        }
    }

    /**
    Creates an `GeoPoint` from the given `x` and `y` primitives:
    
    ```
    # use elastic_types::prelude::*;
    let point: GeoPoint<DefaultGeoPointMapping> = GeoPoint::build(1.0, 1.0);
    ```
    */
    pub fn build(x: f64, y: f64) -> Self {
        GeoPoint::new(Point::new(x, y))
    }

    /**
    Change the format/mapping of this geo point.
    
    # Examples
    
    ```
    # use elastic_types::prelude::*;
    //Get a point formatted as a string
    let point: GeoPoint<DefaultGeoPointMapping<GeoPointString>> = GeoPoint::build(1.0, 1.0);
    
    //Change the format to an object
    let otherpoint: GeoPoint<DefaultGeoPointMapping<GeoPointObject>> = GeoPoint::remap(point);
    ```
    */
    pub fn remap<MInto>(point: GeoPoint<M>) -> GeoPoint<MInto>
        where MInto: GeoPointMapping
    {
        GeoPoint::new(point.value)
    }
}

impl<M> GeoPointFieldType<M> for GeoPoint<M>
    where M: GeoPointMapping
{
}

impl_mapping_type!(Point, GeoPoint, GeoPointMapping);

impl<M> From<Coordinate> for GeoPoint<M>
    where M: GeoPointMapping
{
    fn from(point: Coordinate) -> GeoPoint<M> {
        GeoPoint::new(Point::new(point.x, point.y))
    }
}

impl<M> ToGeo<f64> for GeoPoint<M>
    where M: GeoPointMapping
{
    fn to_geo(&self) -> Geometry {
        GeoEnum::Point(self.value.clone())
    }
}

impl<M> Serialize for GeoPoint<M>
    where M: GeoPointMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        M::Format::format::<S, M>(&self.value, serializer)
    }
}

impl<'de, M> Deserialize<'de> for GeoPoint<M>
    where M: GeoPointMapping
{
    fn deserialize<D>(deserializer: D) -> Result<GeoPoint<M>, D::Error>
        where D: Deserializer<'de>
    {
        let point = M::Format::parse(deserializer)?;

        Ok(point.into())
    }
}

#[cfg(test)]
mod tests {
    use georust::{Geometry, ToGeo, Point, Coordinate};

    use prelude::*;

    #[test]
    fn can_change_point_mapping() {
        fn takes_custom_mapping(_: GeoPoint<DefaultGeoPointMapping<GeoPointObject>>) -> bool {
            true
        }

        let point: GeoPoint<DefaultGeoPointMapping<GeoPointString>> = GeoPoint::new(Point(Coordinate { x: 1.0, y: 1.0 }));

        assert!(takes_custom_mapping(GeoPoint::remap(point)));
    }

    #[test]
    fn can_build_point_from_geo() {
        let coord = Coordinate { x: 1.0, y: 1.0 };

        let point = GeoPoint::<DefaultGeoPointMapping<DefaultGeoPointFormat>>::new(Point(coord.clone()));

        assert_eq!((coord.x, coord.y), (point.x(), point.y()));
    }

    #[test]
    fn can_convert_point_to_geo() {
        let point = GeoPoint::<DefaultGeoPointMapping<DefaultGeoPointFormat>>::new(Point(Coordinate { x: 1.0, y: 1.0 }));
        let geo = point.to_geo();

        match geo {
            Geometry::Point(point) => assert_eq!((1.0, 1.0), (point.x(), point.y())),
            _ => panic!("expected point"),
        }
    }
}
