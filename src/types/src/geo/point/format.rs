use serde::{Serializer, Deserializer};
use super::Point;
use super::mapping::GeoPointMapping;

/** A format used for parsing and formatting geo points. */
pub trait GeoPointFormat
    where Self: Default
{
    /**
    Parses a `geo::Point`.
    
    This requires access to the full `serde` deserializer because geo points can be serialised as
    different kinds of complex objects.
    */
    fn parse<'de, D>(deserializer: D) -> Result<Point, D::Error> where D: Deserializer<'de>;

    /**
    Formats a `geo::Point`.
    
    This requires access to the full `serde` serializer because geo points can be serialised as
    different kinds of complex objects.
    
    Formatting also has access to the mapping type, which could be needed to build the structure
    properly.
    */
    fn format<S, TMapping>(point: &Point, serializer: S) -> Result<S::Ok, S::Error>
        where TMapping: GeoPointMapping<Format = Self>,
              S: Serializer;
}
