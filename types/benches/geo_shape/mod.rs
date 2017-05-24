use elastic_types::prelude::*;
use ::geo_shape_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
    b.iter(|| {
        standalone_field_ser(MyGeoShapeMapping).unwrap()
    });
}
