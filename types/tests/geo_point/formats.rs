extern crate serde;
extern crate serde_json;

use elastic_types::geo::point::prelude::*;

#[test]
fn object() {
	let point: GeoPoint<GeoPointObject> = serde_json::from_str(r#"{"lat":41.0,"lon":-71.34}"#).unwrap();

	assert_eq!((-71.34, 41.0), (
		point.x(),
		point.y()
	));

	let ser = serde_json::to_string(&point).unwrap();

	assert_eq!(r#"{"lat":41.0,"lon":-71.34}"#, ser);
}

#[test]
fn string() {
	let point: GeoPoint<GeoPointString> = serde_json::from_str(r#""41.12,-71.34""#).unwrap();

	assert_eq!((-71.34, 41.12), (
		point.x(),
		point.y()
	));

	let ser = serde_json::to_string(&point).unwrap();

	assert_eq!(r#""41.12,-71.34""#, ser);
}

#[test]
fn string_with_single_point() {
	let de = serde_json::from_str::<GeoPoint<GeoPointString>>(r#""41.12""#);

	assert!(de.is_err());
}

#[test]
fn string_with_invalid_nums() {
	let de = serde_json::from_str::<GeoPoint<GeoPointString>>(r#""41.12,stuff""#);

	assert!(de.is_err());
}

#[test]
fn hash() {
	let point: GeoPoint<GeoPointHash> = serde_json::from_str(r#""drm3btev3e86""#).unwrap();

	assert_eq!((-71.34000012651086, 41.12000000663102), (
		point.x(),
		point.y()
	));

	let ser = serde_json::to_string(&point).unwrap();

	assert_eq!(r#""drm3btev3e86""#, ser);
}

#[test]
fn array() {
	let point: GeoPoint<GeoPointArray> = serde_json::from_str(r#"[-71.34,41]"#).unwrap();

	assert_eq!((-71.34, 41.0), (
		point.x(),
		point.y()
	));

	let ser = serde_json::to_string(&point).unwrap();

	assert_eq!(r#"[-71.34,41.0]"#, ser);
}

#[test]
fn array_with_single_point() {
	let de = serde_json::from_str::<GeoPoint<GeoPointArray>>(r#"[-71.34]"#);

	assert!(de.is_err());
}
