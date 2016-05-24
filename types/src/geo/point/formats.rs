use std::str::FromStr;
use serde;
use serde::{ Error, Serializer, Deserializer };
use georust::{ Coordinate, Point };
use super::GeoPointFormat;

//TODO: Fill in formatting

/// Geo-point expressed as an object, with `lat` and `lon` keys.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeoPointObject;

impl GeoPointFormat for GeoPointObject {
	//TODO: Will need a special struct or HashMap to serialize with, could possibly get away with custom visitor
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
		struct PointVisitor;
        impl serde::de::Visitor for PointVisitor {
            type Value = String;

			fn visit_str<E>(&mut self, value: &str) -> Result<String, E> where
			E: serde::de::Error {
                Ok(String::from(value))
            }

            fn visit_string<E>(&mut self, value: String) -> Result<String, E> where
			E: serde::de::Error {
                Ok(value)
            }
        }

        let fmtd = try!(deserializer.deserialize_string(PointVisitor));

		let xy: Vec<&str> = fmtd.split(",").collect();
		if xy.len() != 2 {
			return Err(D::Error::invalid_value("point must be formatted as '{y},{x}'"))
		}

		let x = match f64::from_str(xy[1]) {
			Ok(x) => x,
			Err(_) => return Err(D::Error::custom("`x` value must be a float"))
		};
		let y = match f64::from_str(xy[0]) {
			Ok(y) => y,
			Err(_) => return Err(D::Error::custom("`y` value must be a float"))
		};

		Ok(Point(Coordinate { x: x, y: y }))
    }

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
		let x = point.0.x.to_string();
		let y = point.0.y.to_string();

		let mut fmtd = String::with_capacity(x.len() + y.len() + 1);
		fmtd.push_str(&y);
		fmtd.push(',');
		fmtd.push_str(&x);

		serializer.serialize_str(&fmtd)
    }
}

/// Geo-point expressed as a geohash.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeoPointHash;
impl GeoPointFormat for GeoPointHash {
	//TODO: Need to work out how geohash crate works
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }
}

/// Geo-point expressed as an array with the format: `[lon, lat]`
#[derive(Debug, Default, Clone, Copy)]
pub struct GeoPointArray;
impl GeoPointFormat for GeoPointArray {
	//TODO: Simple array serialisation
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }
}
