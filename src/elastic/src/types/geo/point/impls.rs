use super::{
    mapping::{
        GeoPointFieldType,
        GeoPointMapping,
    },
    Coordinate,
    GeoPointFormat,
    Geometry,
    Point,
};
use geo::{
    Geometry as GeoEnum,
    ToGeo,
};
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
An Elasticsearch `geo_point` type with a format.

The [format](format/index.html) is provided as a generic parameter.
This struct wraps up a `geo::Point` struct, which have an `x` and `y` floating point value.

# Examples

Defining a geo point using the default format:

```
# use elastic::types::prelude::*;
let point: GeoPoint<DefaultGeoPointMapping> = GeoPoint::build(1.0, 1.0);
```

Defining a geo point using a named format:

```
# use elastic::types::prelude::*;
let point: GeoPoint<DefaultGeoPointMapping<GeoPointString>> = GeoPoint::build(1.0, 1.0);
```

Accessing the values of a geo point:

```
# use elastic::types::prelude::*;
let point: GeoPoint<DefaultGeoPointMapping> = GeoPoint::build(1.0, 1.0);

//eg: (1.0,1.0)
println!("({},{})",
    point.x(),
    point.y()
);
```

# Links
- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/master/geo-point.html)
*/
#[derive(Debug, Clone, PartialEq)]
pub struct GeoPoint<TMapping>
where
    TMapping: GeoPointMapping,
{
    value: Point,
    _m: PhantomData<TMapping>,
}

impl<TMapping> GeoPoint<TMapping>
where
    TMapping: GeoPointMapping,
{
    /**
    Creates a new `GeoPoint` from the given coordinate.

    This function will consume the provided `Coordinate`.

    # Examples

    ```
    # extern crate geo;
    # fn main() {
    use geo::{ Point, Coordinate };
    use elastic::types::prelude::*;

    //Create a geo Coordinate struct
    let coord = Coordinate { x: 1.0, y: 1.0 };

    //Give it to the GeoPoint struct
    let point: GeoPoint<DefaultGeoPointMapping> = GeoPoint::new(Point(coord));
    # }
    ```
    */
    pub fn new<I>(point: I) -> Self
    where
        I: Into<Point>,
    {
        GeoPoint {
            value: point.into(),
            _m: PhantomData,
        }
    }

    /**
    Creates an `GeoPoint` from the given `x` and `y` primitives:

    ```
    # use elastic::types::prelude::*;
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
    # use elastic::types::prelude::*;
    //Get a point formatted as a string
    let point: GeoPoint<DefaultGeoPointMapping<GeoPointString>> = GeoPoint::build(1.0, 1.0);

    //Change the format to an object
    let otherpoint: GeoPoint<DefaultGeoPointMapping<GeoPointObject>> = GeoPoint::remap(point);
    ```
    */
    pub fn remap<TNewMapping>(point: GeoPoint<TMapping>) -> GeoPoint<TNewMapping>
    where
        TNewMapping: GeoPointMapping,
    {
        GeoPoint::new(point.value)
    }
}

impl<TMapping> GeoPointFieldType<TMapping> for GeoPoint<TMapping> where TMapping: GeoPointMapping {}

impl_mapping_type!(Point, GeoPoint, GeoPointMapping);

impl<TMapping> From<Coordinate> for GeoPoint<TMapping>
where
    TMapping: GeoPointMapping,
{
    fn from(point: Coordinate) -> GeoPoint<TMapping> {
        GeoPoint::new(Point::new(point.x, point.y))
    }
}

impl<TMapping> ToGeo<f64> for GeoPoint<TMapping>
where
    TMapping: GeoPointMapping,
{
    fn to_geo(&self) -> Geometry {
        GeoEnum::Point(self.value.clone())
    }
}

impl<TMapping> Serialize for GeoPoint<TMapping>
where
    TMapping: GeoPointMapping,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TMapping::Format::format::<S, TMapping>(&self.value, serializer)
    }
}

impl<'de, TMapping> Deserialize<'de> for GeoPoint<TMapping>
where
    TMapping: GeoPointMapping,
{
    fn deserialize<D>(deserializer: D) -> Result<GeoPoint<TMapping>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let point = TMapping::Format::parse(deserializer)?;

        Ok(point.into())
    }
}

#[cfg(test)]
mod tests {
    use geo::{
        Coordinate,
        Geometry,
        Point,
        ToGeo,
    };

    use crate::types::prelude::*;

    #[test]
    fn can_change_point_mapping() {
        fn takes_custom_mapping(_: GeoPoint<DefaultGeoPointMapping<GeoPointObject>>) -> bool {
            true
        }

        let point: GeoPoint<DefaultGeoPointMapping<GeoPointString>> =
            GeoPoint::new(Point(Coordinate { x: 1.0, y: 1.0 }));

        assert!(takes_custom_mapping(GeoPoint::remap(point)));
    }

    #[test]
    fn can_build_point_from_geo() {
        let coord = Coordinate { x: 1.0, y: 1.0 };

        let point =
            GeoPoint::<DefaultGeoPointMapping<DefaultGeoPointFormat>>::new(Point(coord.clone()));

        assert_eq!((coord.x, coord.y), (point.x(), point.y()));
    }

    #[test]
    fn can_convert_point_to_geo() {
        let point =
            GeoPoint::<DefaultGeoPointMapping<DefaultGeoPointFormat>>::new(Point(Coordinate {
                x: 1.0,
                y: 1.0,
            }));
        let geo = point.to_geo();

        match geo {
            Geometry::Point(point) => assert_eq!((1.0, 1.0), (point.x(), point.y())),
            _ => panic!("expected point"),
        }
    }
}
