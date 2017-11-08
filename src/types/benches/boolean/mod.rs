use elastic_types;
use elastic_types::prelude::*;
use boolean_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
    b.iter(|| {
        elastic_types::derive::standalone_field_ser(MyBooleanMapping).unwrap()
    });
}
