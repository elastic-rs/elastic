use std::str::FromStr;
use serde::{Error, Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, MapVisitor};
use serde::de::impls::VecVisitor;
use georust::{Coordinate, Point};
use geohash;
use super::GeoPointFormat;
use super::mapping::GeoPointMapping;
use ::geo::mapping::Distance;

/// Geo-point expressed as an object, with `lat` and `lon` keys.
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointObject;

struct GeoPointObjectType {
    pub lat: f64,
    pub lon: f64,
}

impl Serialize for GeoPointObjectType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("geo_point", 2));

        try!(serializer.serialize_struct_elt(&mut state, "lat", self.lat));
        try!(serializer.serialize_struct_elt(&mut state, "lon", self.lon));

        serializer.serialize_struct_end(state)
    }
}

impl Deserialize for GeoPointObjectType {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        enum Field {
            Lat,
            Lon,
        };

        impl Deserialize for Field {
            fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
                where D: Deserializer
            {
                struct FieldVisitor;

                impl Visitor for FieldVisitor {
                    type Value = Field;

                    fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                        where E: Error
                    {
                        match value {
                            "lat" => Ok(Field::Lat),
                            "lon" => Ok(Field::Lon),
                            _ => Err(Error::unknown_field(value)),
                        }
                    }
                }

                deserializer.deserialize_struct_field(FieldVisitor)
            }
        }

        struct GeoPointObjectTypeVisitor;

        impl Visitor for GeoPointObjectTypeVisitor {
            type Value = GeoPointObjectType;

            fn visit_map<V>(&mut self, mut visitor: V) -> Result<GeoPointObjectType, V::Error>
                where V: MapVisitor
            {
                let mut lat: Option<f64> = None;
                let mut lon: Option<f64> = None;

                while let Some(key) = try!(visitor.visit_key::<Field>()) {
                    match key {
                        Field::Lat => {
                            if lat.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("lat"));
                            }
                            lat = Some(try!(visitor.visit_value()));
                        }
                        Field::Lon => {
                            if lon.is_some() {
                                return Err(<V::Error as Error>::duplicate_field("lon"));
                            }
                            lon = Some(try!(visitor.visit_value()));
                        }
                    }
                }

                try!(visitor.end());

                let lat = match lat {
                    Some(lat) => lat,
                    None => try!(visitor.missing_field("lat")),
                };

                let lon = match lon {
                    Some(lon) => lon,
                    None => try!(visitor.missing_field("lon")),
                };

                Ok(GeoPointObjectType {
                    lat: lat,
                    lon: lon,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["lat", "lon"];
        deserializer.deserialize_struct("GeoPointObjectType", FIELDS, GeoPointObjectTypeVisitor)
    }
}

impl GeoPointFormat for GeoPointObject {
    fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        let point = try!(GeoPointObjectType::deserialize(deserializer));

        Ok(Point(Coordinate {
            x: point.lon,
            y: point.lat,
        }))
    }

    fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error>
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
    fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        struct PointVisitor;

        impl Visitor for PointVisitor {
            type Value = String;

            fn visit_str<E>(&mut self, value: &str) -> Result<String, E>
                where E: Error
            {
                Ok(String::from(value))
            }

            fn visit_string<E>(&mut self, value: String) -> Result<String, E>
                where E: Error
            {
                Ok(value)
            }
        }

        let fmtd = try!(deserializer.deserialize_string(PointVisitor));

        let xy: Vec<&str> = fmtd.split(",").collect();
        if xy.len() != 2 {
            return Err(D::Error::invalid_value("point must be formatted as `'{y},{x}'`"));
        }

        let x = match f64::from_str(xy[1]) {
            Ok(x) => x,
            Err(_) => return Err(D::Error::custom("`x` value must be a float")),
        };
        let y = match f64::from_str(xy[0]) {
            Ok(y) => y,
            Err(_) => return Err(D::Error::custom("`y` value must be a float")),
        };

        Ok(Point(Coordinate { x: x, y: y }))
    }

    fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error>
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
    fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        struct PointVisitor;
        impl Visitor for PointVisitor {
            type Value = String;

            fn visit_str<E>(&mut self, value: &str) -> Result<String, E>
                where E: Error
            {
                Ok(String::from(value))
            }

            fn visit_string<E>(&mut self, value: String) -> Result<String, E>
                where E: Error
            {
                Ok(value)
            }
        }

        let hash = try!(deserializer.deserialize_string(PointVisitor));
        let (coord, _, _) = geohash::decode(&hash);
        Ok(Point(coord))
    }

    fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error>
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
    fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error>
        where D: Deserializer
    {
        let point = try!(deserializer.deserialize_seq(VecVisitor::<f64>::new()));

        if point.len() != 2 {
            return Err(D::Error::invalid_value("point must be formatted as `[{x},{y}]`"));
        }

        Ok(Point(Coordinate {
            x: point[0],
            y: point[1],
        }))
    }

    fn format<S, M>(point: &Point, serializer: &mut S) -> Result<(), S::Error>
        where M: GeoPointMapping<Format = Self>,
              S: Serializer
    {
        [point.x(), point.y()].serialize(serializer)
    }
}
