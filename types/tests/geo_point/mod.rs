#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;
pub mod formats;

extern crate serde;
extern crate serde_json;

use elastic_types::geo::point::mapping::*;
use elastic_types::geo::point::prelude::*;
use georust::{ Geometry, ToGeo, Coordinate };

use ::geo_point_fixtures::*;

#[test]
fn can_change_point_mapping() {
	fn takes_custom_mapping(point: ElasticGeoPoint<GeoPointObject, MyGeoPointMapping>) -> bool {
		true
	}

	let point: ElasticGeoPoint<GeoPointString> = ElasticGeoPoint::new(Coordinate { x: 1.0, y: 1.0 });

	assert!(takes_custom_mapping(point.remap()));
}

#[test]
fn can_build_point_from_geo() {
	let coord = Coordinate { x: 1.0, y: 1.0 };

	let point = ElasticGeoPoint::<DefaultGeoPointFormat>::new(coord.clone());

	assert_eq!(
		(coord.x, coord.y),
		(point.x(), point.y())
	);
}

#[test]
fn can_convert_point_to_geo() {
	let point = ElasticGeoPoint::<DefaultGeoPointFormat>::new(Coordinate { x: 1.0, y: 1.0 });
	let geo = point.to_geo();

	match geo {
		Geometry::Point(_) => (),
		_ => panic!("expected point")
	}
}

#[test]
fn serialise_elastic_point() {
	let point = ElasticGeoPoint::<GeoPointString>::new(Coordinate { x: -71.34, y: 41.12 });

	let ser = serde_json::to_string(&point).unwrap();

	assert_eq!(r#""41.12,-71.34""#, ser);
}

#[test]
fn deserialise_elastic_point() {
	let point: ElasticGeoPoint<GeoPointString> = serde_json::from_str(r#""41.12,-71.34""#).unwrap();

	assert_eq!((-71.34, 41.12), (
		point.x(),
		point.y()
	));
}
