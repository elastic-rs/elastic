#![feature(test, plugin)]
#![plugin(json_str)]

extern crate serde;
extern crate serde_json;
extern crate test;
extern crate geo as georust;
extern crate elastic_types;

use georust::Coordinate;
use elastic_types::geo::point::prelude::*;
use ::geo_point_fixtures::*;

use test::Bencher;

#[bench]
fn parse_string(b: &mut Bencher) {
    let ser = r#""41.12,-71.34""#;
    b.iter(|| {
        let point: ElasticGeoPoint<GeoPointString> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_string(b: &mut Bencher) {
    let point = ElasticGeoPoint::<GeoPointString>::new(Coordinate { x: -71.34, y: 41.12 });

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn parse_object(b: &mut Bencher) {
    let ser = r#"{"lat":41,"lon":-71.34}"#;
    b.iter(|| {
        let point: ElasticGeoPoint<GeoPointObject> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_object(b: &mut Bencher) {
    let point = ElasticGeoPoint::<GeoPointObject>::new(Coordinate { x: -71.34, y: 41.12 });

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn parse_hash(b: &mut Bencher) {
    let ser = r#""drm3btev3e86""#;
    b.iter(|| {
        let point: ElasticGeoPoint<GeoPointHash> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_hash(b: &mut Bencher) {
    let point = ElasticGeoPoint::<GeoPointHash>::new(Coordinate { x: -71.34, y: 41.12 });

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn parse_array(b: &mut Bencher) {
    let ser = r#"[-71.34,41]"#;
    b.iter(|| {
        let point: ElasticGeoPoint<GeoPointArray> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_array(b: &mut Bencher) {
    let point = ElasticGeoPoint::<GeoPointArray>::new(Coordinate { x: -71.34, y: 41.12 });

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn mapping(b: &mut Bencher) {
	b.iter(|| {
		let mapping = MyGeoPointMapping::<DefaultGeoPointFormat>::default();
		serde_json::to_string(&mapping).unwrap()
	});
}
