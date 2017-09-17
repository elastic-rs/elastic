use elastic_types;
use elastic_types::prelude::*;
use number_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
    b.iter(|| elastic_types::derive::standalone_field_ser(MyIntegerMapping).unwrap());
}
