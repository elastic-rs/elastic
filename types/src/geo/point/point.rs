use std::marker::PhantomData;
use serde::{ Serialize, Deserialize, Serializer, Deserializer };
use georust::{ Coordinate, Point };
use ::mapping::{ ElasticFieldMapping, ElasticType };
use super::mapping::{ ElasticGeoPointMapping, DefaultGeoPointMapping };
use super::GeoPointFormat;

pub struct ElasticGeoPoint<F, T = DefaultGeoPointMapping<F>> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    pub value: Point,
	phantom_f: PhantomData<F>,
	phantom_t: PhantomData<T>
}

impl <F, T> ElasticGeoPoint<F, T> where
F: GeoPointFormat,
T: ElasticFieldMapping<F> + ElasticGeoPointMapping<F> {
    pub fn new(point: Coordinate) -> ElasticGeoPoint<F, T> {
        ElasticGeoPoint {
            value: Point(point),
            phantom_f: PhantomData,
            phantom_t: PhantomData
        }
    }

	pub fn into<FInto, TInto>(self) -> ElasticGeoPoint<FInto, TInto> where
	FInto: GeoPointFormat,
	TInto: ElasticFieldMapping<FInto> + ElasticGeoPointMapping<FInto> {
		ElasticGeoPoint::<FInto, TInto>::new(self.value.0)
	}
}

//TODO: impl ToGeo for ElasticGeoPoint
