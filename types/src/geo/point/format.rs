use serde::{ Serializer, Deserializer };
use georust::Point;

/// A format used for parsing and formatting geo points.
pub trait GeoPointFormat
where Self : Default + Copy {
	/// Parses a `geo::Point` from a deserializer.
	///
	/// This requires access to the full deserializer because geo points can be serialised as
	/// complex objects.
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer;

	/// Formats a given `geo::Point`.
	///
	/// This requires access to the full serializer because geo points can be serialised as
	/// complex objects.
	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer;
}
