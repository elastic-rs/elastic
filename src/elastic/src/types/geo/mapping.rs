/*! Common mapping for the Elasticsearch `geo` types. */

use serde::{
    self,
    Serialize,
};

/** A unit of measure for distance. */
pub enum DistanceUnit {
    /** For `in`. */
    Inches,
    /** For `yd`. */
    Yards,
    /** For `mi`. */
    Miles,
    /** For `km`. */
    Kilometers,
    /** For `m`. */
    Meters,
    /** For `cm`. */
    Centimeters,
    /** For `mm`. */
    Millimeters,
}

/** A distance value paired with a unit of measure. */
pub struct Distance(pub f32, pub DistanceUnit);

impl ToString for Distance {
    fn to_string(&self) -> String {
        let value = self.0.to_string();
        let unit = match self.1 {
            DistanceUnit::Inches => "in",
            DistanceUnit::Yards => "yd",
            DistanceUnit::Miles => "mi",
            DistanceUnit::Kilometers => "km",
            DistanceUnit::Meters => "m",
            DistanceUnit::Centimeters => "cm",
            DistanceUnit::Millimeters => "mm",
        };

        let mut s = String::with_capacity(value.len() + unit.len());
        s.push_str(&value);
        s.push_str(unit);

        s
    }
}

impl Serialize for Distance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
