use serde::{ Serializer, Deserializer };
use georust::Point;
use super::GeoPointFormat;

/// Geo-point expressed as an object, with `lat` and `lon` keys.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeoPointObject;

impl GeoPointFormat for GeoPointObject {
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }
}

/// Geo-point expressed as a string with the format: `"lat,lon"`.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeoPointString;
impl GeoPointFormat for GeoPointString {
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }
}

/// Geo-point expressed as a geohash.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeoPointHash;
impl GeoPointFormat for GeoPointHash {
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }
}

/// Geo-point expressed as an array with the format: [ lon, lat]
#[derive(Debug, Default, Clone, Copy)]
pub struct GeoPointArray;
impl GeoPointFormat for GeoPointArray {
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }
}
