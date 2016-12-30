use serde_json;
use georust::{ Point, Coordinate };
use elastic_types::prelude::*;
use ::geo_point_fixtures::*;

use test::Bencher;

#[bench]
fn parse_string(b: &mut Bencher) {
    let ser = r#""41.12,-71.34""#;
    b.iter(|| {
        let point: GeoPoint<GeoPointString> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_string(b: &mut Bencher) {
    let point = GeoPoint::<GeoPointString>::new(Point(Coordinate { x: -71.34, y: 41.12 }));

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn parse_object(b: &mut Bencher) {
    let ser = r#"{"lat":41,"lon":-71.34}"#;
    b.iter(|| {
        let point: GeoPoint<GeoPointObject> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_object(b: &mut Bencher) {
    let point = GeoPoint::<GeoPointObject>::new(Point(Coordinate { x: -71.34, y: 41.12 }));

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn parse_hash(b: &mut Bencher) {
    let ser = r#""drm3btev3e86""#;
    b.iter(|| {
        let point: GeoPoint<GeoPointHash> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_hash(b: &mut Bencher) {
    let point = GeoPoint::<GeoPointHash>::new(Point(Coordinate { x: -71.34, y: 41.12 }));

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn parse_array(b: &mut Bencher) {
    let ser = r#"[-71.34,41]"#;
    b.iter(|| {
        let point: GeoPoint<GeoPointArray> = serde_json::from_str(ser).unwrap();
        point
	});
}

#[bench]
fn fmt_array(b: &mut Bencher) {
    let point = GeoPoint::<GeoPointArray>::new(Point(Coordinate { x: -71.34, y: 41.12 }));

	b.iter(|| {
    	serde_json::to_string(&point).unwrap()
	});
}

#[bench]
fn mapping(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&Field::from(MyGeoPointMapping)).unwrap()
	});
}
