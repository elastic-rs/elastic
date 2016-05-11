use super::{ GeoPointFormat, GeoPoint };

#[derive(Debug, Default, Clone)]
pub struct GeoPointObject;
impl GeoPointFormat for GeoPointObject {
	fn parse<D>(deserializer: &mut D) -> Result<GeoPoint, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &GeoPoint, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }

	fn name() -> &'static str {
        panic!("implement")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GeoPointString;
impl GeoPointFormat for GeoPointString {
	fn parse<D>(deserializer: &mut D) -> Result<GeoPoint, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &GeoPoint, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }

	fn name() -> &'static str {
        panic!("implement")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GeoPointHash;
impl GeoPointFormat for GeoPointHash {
	fn parse<D>(deserializer: &mut D) -> Result<GeoPoint, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &GeoPoint, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }

	fn name() -> &'static str {
        panic!("implement")
    }
}

#[derive(Debug, Default, Clone)]
pub struct GeoPointArray;
impl GeoPointFormat for GeoPointArray {
	fn parse<D>(deserializer: &mut D) -> Result<GeoPoint, D::Error> where
	D: Deserializer {
        panic!("implement")
    }

	fn format<S>(point: &GeoPoint, serializer: &mut S) -> Result<(), S::Error> where
	S: Serializer {
        panic!("implement")
    }

	fn name() -> &'static str {
        panic!("implement")
    }
}
