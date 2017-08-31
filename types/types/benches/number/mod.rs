use elastic_types::prelude::*;
use number_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
    b.iter(|| standalone_field_ser(MyIntegerMapping).unwrap());
}
