#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

extern crate serde;
extern crate serde_json;

use elastic_types::geo::point::mapping::*;
use elastic_types::geo::point::prelude::*;
use georust::{ Geometry, ToGeo, Coordinate };

use ::geo_point_fixtures::*;

#[test]
fn serialise_mapping_default() {
	let mapping = DefaultGeoPointMapping::<DefaultGeoPointFormat>::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "geo_point"
	});

	assert_eq!(expected, ser);
}

#[test]
fn serialise_mapping_custom() {
	let mapping = MyGeoPointMapping::<DefaultGeoPointFormat>::default();
	let ser = serde_json::to_string(&mapping).unwrap();

	let expected = json_str!({
		"type": "geo_point",
		"geohash": false,
		"geohash_precision": 12,
		"geohash_prefix": true,
		"ignore_malformed": true,
		"lat_lon": true,
		"precision_step": 128
	});

	assert_eq!(expected, ser);
}
