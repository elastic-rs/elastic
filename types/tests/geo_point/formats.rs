extern crate serde;
extern crate serde_json;

use elastic_types::geo::point::mapping::*;
use elastic_types::geo::point::prelude::*;
use georust::{ Geometry, ToGeo, Coordinate };

use ::geo_point_fixtures::*;

#[test]
fn object() {
    panic!("implement")
}

#[test]
fn string() {
    let point: ElasticGeoPoint<GeoPointString> = serde_json::from_str(r#""41.12,-71.34""#).unwrap();

    assert_eq!((-71.34, 41.12), (
		point.x(),
		point.y()
	));

    let ser = serde_json::to_string(&point).unwrap();

	assert_eq!(r#""41.12,-71.34""#, ser);
}

#[test]
fn string_with_single_point() {
    let de = serde_json::from_str::<ElasticGeoPoint<GeoPointString>>(r#""41.12""#);

    assert!(de.is_err());
}

#[test]
fn string_with_invalid_nums() {
    let de = serde_json::from_str::<ElasticGeoPoint<GeoPointString>>(r#""41.12,stuff""#);

    assert!(de.is_err());
}

#[test]
fn hash() {
    panic!("implement")
}

#[test]
fn array() {
    panic!("implement")
}
