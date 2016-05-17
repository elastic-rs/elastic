use serde::{ Serializer, Deserializer };
use georust::Point;
use super::GeoPointFormat;

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

	fn name() -> &'static str {
        panic!("implement")
    }
}

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

	fn name() -> &'static str {
        panic!("implement")
    }
}

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

	fn name() -> &'static str {
        panic!("implement")
    }
}

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

	fn name() -> &'static str {
        panic!("implement")
    }
}
