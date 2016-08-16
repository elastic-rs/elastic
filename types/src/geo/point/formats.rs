use std::str::FromStr;
use serde;
use serde::{ Error, Serialize, Serializer, Deserialize, Deserializer };
use georust::{ Coordinate, Point };
use geohash;
use super::GeoPointFormat;
use ::geo::mapping::Distance;

/// Geo-point expressed as an object, with `lat` and `lon` keys.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointObject;

#[derive(Serialize, Deserialize)]
struct GeoPointObjectType {
	pub lat: f64,
	pub lon: f64
}

impl GeoPointFormat for GeoPointObject {
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        let point = try!(GeoPointObjectType::deserialize(deserializer));

		Ok(Point(Coordinate {
			x: point.lon,
			y: point.lat
		}))
    }

	fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	M: ::geo::point::mapping::ElasticGeoPointMapping<Self>,
	S: Serializer {
        GeoPointObjectType { lon: point.x(), lat: point.y() }.serialize(serializer)
    }
}

/// Geo-point expressed as a string with the format: `"lat,lon"`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
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
			return Err(D::Error::invalid_value("point must be formatted as `'{y},{x}'`"))
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

	fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	M: ::geo::point::mapping::ElasticGeoPointMapping<Self>,
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
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointHash;
impl GeoPointFormat for GeoPointHash {
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

        let hash = try!(deserializer.deserialize_string(PointVisitor));
		let (coord, _, _) = geohash::decode(&hash);
		Ok(Point(coord))
    }

	fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	M: ::geo::point::mapping::ElasticGeoPointMapping<Self>,
	S: Serializer {
		let len = match M::geohash_precision() {
			Some(Distance(l, _)) => l as usize,
			None => 12usize
		};

		geohash::encode(Coordinate { x: point.x(), y: point.y() }, len).serialize(serializer)
    }
}

/// Geo-point expressed as an array with the format: `[lon, lat]`
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointArray;
impl GeoPointFormat for GeoPointArray {
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer {
        let point = try!(deserializer.deserialize_seq(serde::de::impls::VecVisitor::<f64>::new()));

		if point.len() != 2 {
			return Err(D::Error::invalid_value("point must be formatted as `[{x},{y}]`"))
		}

		Ok(Point(Coordinate { x: point[0], y: point[1] }))
    }

	fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	M: ::geo::point::mapping::ElasticGeoPointMapping<Self>,
	S: Serializer {
        [point.x(), point.y()].serialize(serializer)
    }
}
