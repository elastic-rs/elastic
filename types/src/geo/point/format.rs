pub struct GeoPoint(f64, f64);

pub trait GeoPointFormat
where Self : Default + Copy {
	fn parse<D>(deserializer: &mut D) -> Result<GeoPoint, D::Error> where
	D: Deserializer;

	fn format<S>(point: &GeoPoint, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer;

	fn name() -> &'static str;
}
