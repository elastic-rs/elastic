use serde::{ Serializer, Deserializer };
use georust::Point;

pub trait GeoPointFormat
where Self : Default + Copy {
	fn parse<D>(deserializer: &mut D) -> Result<Point, D::Error> where
	D: Deserializer;

	fn format<S>(point: &Point, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer;

	fn name() -> &'static str;
}
