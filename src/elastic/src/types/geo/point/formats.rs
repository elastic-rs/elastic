use super::{
    mapping::GeoPointMapping,
    Coordinate,
    GeoPointFormat,
    Point,
};
use crate::types::geo::mapping::Distance;
use geohash;
use serde::{
    de::{
        Error as DeError,
        SeqAccess,
        Unexpected,
        Visitor,
    },
    ser::Error as SerError,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::{
    fmt::Write,
    str::FromStr,
};

/** The default `geo_point` format (`GeoPointArray`). */
pub type DefaultGeoPointFormat = GeoPointArray;

/** Geo-point expressed as an object, with `lat` and `lon` keys. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointObject;

#[derive(Serialize, Deserialize)]
struct GeoPointObjectType {
    pub lat: f64,
    pub lon: f64,
}

impl GeoPointFormat for GeoPointObject {
    fn parse<'de, D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>,
    {
        let point = GeoPointObjectType::deserialize(deserializer)?;

        Ok(Point::new(point.lon, point.lat))
    }

    fn format<S, TMapping>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
    where
        TMapping: GeoPointMapping<Format = Self>,
        S: Serializer,
    {
        GeoPointObjectType {
            lon: point.x(),
            lat: point.y(),
        }
        .serialize(serializer)
    }
}

/** Geo-point expressed as a string with the format: `"lat,lon"`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointString;
impl GeoPointFormat for GeoPointString {
    fn parse<'de, D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointVisitor;

        impl<'de> Visitor<'de> for PointVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json string formatted as 'y,x'")
            }

            fn visit_str<E>(self, value: &str) -> Result<String, E>
            where
                E: DeError,
            {
                Ok(String::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<String, E>
            where
                E: DeError,
            {
                Ok(value)
            }
        }

        let fmtd = deserializer.deserialize_string(PointVisitor)?;

        let xy: Vec<&str> = fmtd.split(",").collect();
        if xy.len() != 2 {
            return Err(D::Error::invalid_value(
                Unexpected::Str(&fmtd),
                &"point must be formatted as `'y,x'`",
            ));
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

    fn format<S, TMapping>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
    where
        TMapping: GeoPointMapping<Format = Self>,
        S: Serializer,
    {
        let mut fmtd = String::new();

        write!(&mut fmtd, "{},{}", point.0.y, point.0.x).expect("Display for a point failed");

        serializer.serialize_str(&fmtd)
    }
}

/** Geo-point expressed as a geohash. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointHash;
impl GeoPointFormat for GeoPointHash {
    fn parse<'de, D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointVisitor;
        impl<'de> Visitor<'de> for PointVisitor {
            type Value = String;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "a json string containing a geohash")
            }

            fn visit_str<E>(self, value: &str) -> Result<String, E>
            where
                E: DeError,
            {
                Ok(String::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<String, E>
            where
                E: DeError,
            {
                Ok(value)
            }
        }

        let hash = deserializer.deserialize_string(PointVisitor)?;
        let (coord, _, _) = geohash::decode(&hash).map_err(|e| D::Error::custom(e))?;
        Ok(Point::new(coord.x, coord.y))
    }

    fn format<S, TMapping>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
    where
        TMapping: GeoPointMapping<Format = Self>,
        S: Serializer,
    {
        let len = match TMapping::geohash_precision() {
            Some(Distance(l, _)) => l as usize,
            None => 12usize,
        };

        geohash::encode(
            Coordinate {
                x: point.x(),
                y: point.y(),
            },
            len,
        )
        .map_err(|e| S::Error::custom(e))?
        .serialize(serializer)
    }
}

/** Geo-point expressed as an array with the format: `[lon, lat]` */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct GeoPointArray;
impl GeoPointFormat for GeoPointArray {
    fn parse<'de, D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PointVisitor;
        impl<'de> Visitor<'de> for PointVisitor {
            type Value = Point;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "an array with 2 numbers")
            }

            fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
            where
                S: SeqAccess<'de>,
            {
                let mut values = Vec::with_capacity(2);

                while let Some(value) = visitor.next_element()? {
                    if values.len() == 2 {
                        Err(S::Error::invalid_value(
                            Unexpected::Seq,
                            &"a json array with 2 values",
                        ))?;
                    }

                    values.push(value);
                }

                if values.len() != 2 {
                    Err(S::Error::invalid_value(
                        Unexpected::Seq,
                        &"a json array with 2 values",
                    ))?;
                }

                Ok(Point::new(values[0], values[1]))
            }
        }

        deserializer.deserialize_any(PointVisitor)
    }

    fn format<S, TMapping>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
    where
        TMapping: GeoPointMapping<Format = Self>,
        S: Serializer,
    {
        [point.x(), point.y()].serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::prelude::*;

    #[test]
    fn object() {
        let point: GeoPoint<DefaultGeoPointMapping<GeoPointObject>> =
            serde_json::from_str(r#"{"lat":41.0,"lon":-71.34}"#).unwrap();

        assert_eq!((-71.34, 41.0), (point.x(), point.y()));

        let ser = serde_json::to_string(&point).unwrap();

        assert_eq!(r#"{"lat":41.0,"lon":-71.34}"#, ser);
    }

    #[test]
    fn string() {
        let point: GeoPoint<DefaultGeoPointMapping<GeoPointString>> =
            serde_json::from_str(r#""41.12,-71.34""#).unwrap();

        assert_eq!((-71.34, 41.12), (point.x(), point.y()));

        let ser = serde_json::to_string(&point).unwrap();

        assert_eq!(r#""41.12,-71.34""#, ser);
    }

    #[test]
    fn string_with_single_point() {
        let de =
            serde_json::from_str::<GeoPoint<DefaultGeoPointMapping<GeoPointString>>>(r#""41.12""#);

        assert!(de.is_err());
    }

    #[test]
    fn string_with_invalid_nums() {
        let de = serde_json::from_str::<GeoPoint<DefaultGeoPointMapping<GeoPointString>>>(
            r#""41.12,stuff""#,
        );

        assert!(de.is_err());
    }

    #[test]
    fn hash() {
        let point: GeoPoint<DefaultGeoPointMapping<GeoPointHash>> =
            serde_json::from_str(r#""drm3btev3e86""#).unwrap();

        assert_eq!(
            (-71.34000012651086, 41.12000000663102),
            (point.x(), point.y())
        );

        let ser = serde_json::to_string(&point).unwrap();

        assert_eq!(r#""drm3btev3e86""#, ser);
    }

    #[test]
    fn array() {
        let point: GeoPoint<DefaultGeoPointMapping<GeoPointArray>> =
            serde_json::from_str(r#"[-71.34,41]"#).unwrap();

        assert_eq!((-71.34, 41.0), (point.x(), point.y()));

        let ser = serde_json::to_string(&point).unwrap();

        assert_eq!(r#"[-71.34,41.0]"#, ser);
    }

    #[test]
    fn array_with_single_point() {
        let de =
            serde_json::from_str::<GeoPoint<DefaultGeoPointMapping<GeoPointArray>>>(r#"[-71.34]"#);

        assert!(de.is_err());
    }

}
