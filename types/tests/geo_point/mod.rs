#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros)]

pub mod mapping;
pub mod formats;

extern crate serde;
extern crate serde_json;

use georust::{ Geometry, ToGeo, Coordinate };

use elastic_types::geo::point::prelude::*;
use ::geo_point_fixtures::*;

#[test]
fn can_change_point_mapping() {
	fn takes_custom_mapping(_: GeoPoint<GeoPointObject>) -> bool {
		true
	}

	let point: GeoPoint<GeoPointString> = GeoPoint::new(Coordinate { x: 1.0, y: 1.0 });

	assert!(takes_custom_mapping(point.remap()));
}

#[test]
fn can_build_point_from_geo() {
	let coord = Coordinate { x: 1.0, y: 1.0 };

	let point = GeoPoint::<DefaultGeoPointFormat>::new(coord.clone());

	assert_eq!(
		(coord.x, coord.y),
		(point.x(), point.y())
	);
}

#[test]
fn can_convert_point_to_geo() {
	let point = GeoPoint::<DefaultGeoPointFormat>::new(Coordinate { x: 1.0, y: 1.0 });
	let geo = point.to_geo();

	match geo {
		Geometry::Point(point) => assert_eq!(
			(1.0, 1.0),
			(point.x(), point.y())
		),
		_ => panic!("expected point")
	}
}
