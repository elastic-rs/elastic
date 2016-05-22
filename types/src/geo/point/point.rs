use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use georust::{ Coordinate, Point, ToGeo, Geometry };
use ::mapping::{ ElasticFieldMapping, ElasticType };
use super::mapping::{ ElasticGeoPointMapping, DefaultGeoPointMapping };
use super::GeoPointFormat;

//TODO: Proper docs

/// An Elasticsearch `geo_point` type with a format.
pub struct ElasticGeoPoint<F, T = DefaultGeoPointMapping<F>> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    /// The `x` and `y` coordinate for the point.
    pub value: Point,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    /// Creates a new `ElasticGeoPoint` from the given coordinate.
    pub fn new(point: Coordinate) -> ElasticGeoPoint<F, T> {
        ElasticGeoPoint {
            value: Point(point),
            phantom_f: PhantomData,
            phantom_t: PhantomData
        }
    }

    /// Get the `x` part of the coordinate.
    pub fn x(&self) -> f64 {
        self.value.x()
    }

    /// Get the `y` part of the coordinate.
    pub fn y(&self) -> f64 {
        self.value.y()
    }

    /// Change the format/mapping of this geo point.
	pub fn remap<FInto, TInto>(self) -> ElasticGeoPoint<FInto, TInto> where
	FInto: GeoPointFormat,
	TInto: ElasticFieldMapping<FInto> + ElasticGeoPointMapping<FInto> {
		ElasticGeoPoint::<FInto, TInto>::new(self.value.0)
	}
}

impl <F, T> ElasticType<T, F> for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {

}

impl <F, T> From<Point> for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn from(point: Point) -> ElasticGeoPoint<F, T> {
		ElasticGeoPoint::<F, T>::new(point.0)
	}
}

impl <F, T> ToGeo for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    fn to_geo(&self) -> Geometry {
        Geometry::Point(self.value.clone())
    }
}

//Serialize geo_point
impl <F, T> Serialize for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		F::format(&self.value, serializer)
	}
}

//Deserialize geo_point
impl <F, T> Deserialize for ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
	fn deserialize<D>(deserializer: &mut D) -> Result<ElasticGeoPoint<F, T>, D::Error> where
	D: Deserializer {
        let point = try!(F::parse(deserializer));

        Ok(point.into())
    }
}
