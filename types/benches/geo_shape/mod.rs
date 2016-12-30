use serde_json;
use elastic_types::prelude::*;
use ::geo_shape_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&Field::from(MyGeoShapeMapping)).unwrap()
	});
}
