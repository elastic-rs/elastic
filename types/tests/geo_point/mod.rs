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
	fn takes_custom_mapping(_: ElasticGeoPoint<GeoPointObject, MyGeoPointMapping>) -> bool {
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
		Geometry::Point(point) => assert_eq!(
			(1.0, 1.0),
			(point.x(), point.y())
		),
		_ => panic!("expected point")
	}
}
