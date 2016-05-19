use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use georust::{ Coordinate, Point };
use ::mapping::{ ElasticFieldMapping, ElasticType };
use super::mapping::{ ElasticGeoPointMapping, DefaultGeoPointMapping };
use super::GeoPointFormat;

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

    /// Get `x` coordinate.
    pub fn x(&self) -> f64 {
        self.value.x()
    }

    /// Get `y` coordinate.
    pub fn y(&self) -> f64 {
        self.value.y()
    }

    /// Change the format/mapping of this geo point.
	pub fn into<FInto, TInto>(self) -> ElasticGeoPoint<FInto, TInto> where
	FInto: GeoPointFormat,
	TInto: ElasticFieldMapping<FInto> + ElasticGeoPointMapping<FInto> {
		ElasticGeoPoint::<FInto, TInto>::new(self.value.0)
	}
}

//TODO: impl ToGeo for ElasticGeoPoint
//TODO: impl serialisation
