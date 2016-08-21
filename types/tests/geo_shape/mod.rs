#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;

extern crate serde;
extern crate serde_json;
extern crate geojson;
extern crate elastic_types;

use geojson::{ Geometry, Value };

use elastic_types::mapping::prelude::*;
use elastic_types::geo::shape::prelude::*;
use ::geo_shape_fixtures::*;

#[test]
fn can_change_geo_shape_mapping() {
	fn takes_custom_mapping(_: ElasticGeoShape<MyGeoShapeMapping>) -> bool {
		true
	}

	let point: ElasticGeoShape<DefaultGeoShapeMapping> = ElasticGeoShape::new(
		Geometry::new(
			Value::Point(vec![ 1.0, 1.0 ])
		)
	);

	assert!(takes_custom_mapping(point.remap()));
}

#[test]
fn serialise_elastic_geo_shape() {
	let shape = ElasticGeoShape::<DefaultGeoShapeMapping>::new(
		Geometry::new(
			Value::Point(vec![ 1.0, 1.0 ])
		)
	);

	let ser = serde_json::to_string(&shape).unwrap();

	assert_eq!(json_str!({
		"coordinates": [ 1.0, 1.0 ],
		"type": "Point"
	}), ser);
}

#[test]
fn deserialise_elastic_geo_shape() {
	let shape: ElasticGeoShape<DefaultGeoShapeMapping> = serde_json::from_str(&json_str!({
		"coordinates": [ 1, 1 ],
		"type": "Point"
	})).unwrap();

	assert_eq!(
		&Geometry::new(
			Value::Point(vec![ 1.0, 1.0 ])),
		shape.get()
	);
}
