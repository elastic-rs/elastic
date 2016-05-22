#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;
pub mod formats;

use elastic_types::geo::point::mapping::*;
use elastic_types::geo::point::prelude::*;
use georust::Coordinate;

use ::geo_point_fixtures::*;

#[test]
fn can_change_point_mapping() {
	fn takes_custom_mapping(point: ElasticGeoPoint<GeoPointObject, MyGeoPointMapping>) -> bool {
		true
	}

	let point: ElasticGeoPoint<GeoPointString, DefaultGeoPointMapping<GeoPointString>> = ElasticGeoPoint::new(Coordinate { x: 1.0, y: 1.0 });

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
	//Test we can run to_geo() on ElasticGeoPoint and get geo::Geometry::Point
	panic!("implement")
}

#[test]
fn serialise_elastic_point() {
	panic!("implement")
}

#[test]
fn deserialise_elastic_point() {
	panic!("implement")
}
