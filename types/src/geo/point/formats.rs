use std::str::FromStr;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Error, Unexpected, Visitor};
use serde::de::impls::VecVisitor;
use geohash;
use super::{Coordinate, Point, GeoPointFormat};
use super::mapping::GeoPointMapping;
use ::geo::mapping::Distance;

/// Geo-point expressed as an object, with `lat` and `lon` keys.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointObject;

#[derive(Serialize, Deserialize)]
struct GeoPointObjectType {
    pub lat: f64,
    pub lon: f64,
}

impl GeoPointFormat for GeoPointObject {
    fn parse<D>(deserializer: D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        let point = try!(GeoPointObjectType::deserialize(deserializer));

        Ok(Point::new(point.lon, point.lat))
    }

    fn format<S, M>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
        where M: GeoPointMapping<Format = Self>,
              S: Serializer
    {
        GeoPointObjectType {
                lon: point.x(),
                lat: point.y(),
            }
            .serialize(serializer)
    }
}

/// Geo-point expressed as a string with the format: `"lat,lon"`.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointString;
impl GeoPointFormat for GeoPointString {
    fn parse<D>(deserializer: D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        struct PointVisitor;

        impl Visitor for PointVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json string formatted as 'y,x'")
            }

            fn visit_str<E>(self, value: &str) -> Result<String, E>
                where E: Error
            {
                Ok(String::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<String, E>
                where E: Error
            {
                Ok(value)
            }
        }

        let fmtd = try!(deserializer.deserialize_string(PointVisitor));

        let xy: Vec<&str> = fmtd.split(",").collect();
        if xy.len() != 2 {
            return Err(D::Error::invalid_value(Unexpected::Str(&fmtd),
                                               &"point must be formatted as `'y,x'`"));
        }

        let x = match f64::from_str(xy[1]) {
            Ok(x) => x,
            Err(_) => return Err(D::Error::custom("`x` value must be a float")),
        };
        let y = match f64::from_str(xy[0]) {
            Ok(y) => y,
            Err(_) => return Err(D::Error::custom("`y` value must be a float")),
        };

        Ok(Point::new(x, y))
    }

    fn format<S, M>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
        where M: GeoPointMapping<Format = Self>,
              S: Serializer
    {
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
    fn parse<D>(deserializer: D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        struct PointVisitor;
        impl Visitor for PointVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json string containing a geohash")
            }

            fn visit_str<E>(self, value: &str) -> Result<String, E>
                where E: Error
            {
                Ok(String::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<String, E>
                where E: Error
            {
                Ok(value)
            }
        }

        let hash = try!(deserializer.deserialize_string(PointVisitor));
        let (coord, _, _) = geohash::decode(&hash);
        Ok(Point::new(coord.x, coord.y))
    }

    fn format<S, M>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
        where M: GeoPointMapping<Format = Self>,
              S: Serializer
    {
        let len = match M::geohash_precision() {
            Some(Distance(l, _)) => l as usize,
            None => 12usize,
        };

        geohash::encode(Coordinate {
                            x: point.x(),
                            y: point.y(),
                        },
                        len)
            .serialize(serializer)
    }
}

/// Geo-point expressed as an array with the format: `[lon, lat]`
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointArray;
impl GeoPointFormat for GeoPointArray {
    fn parse<D>(deserializer: D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        let point = try!(deserializer.deserialize_seq(VecVisitor::<f64>::new()));

        if point.len() != 2 {
            return Err(D::Error::invalid_value(Unexpected::Seq, &"a json array with 2 values"));
        }

        Ok(Point::new(point[0], point[1]))
    }

    fn format<S, M>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
        where M: GeoPointMapping<Format = Self>,
              S: Serializer
    {
        [point.x(), point.y()].serialize(serializer)
    }
}
