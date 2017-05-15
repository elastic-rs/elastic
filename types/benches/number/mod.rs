use serde_json;
use elastic_types::prelude::*;
use ::number_fixtures::*;

use test::Bencher;

#[bench]
fn mapping(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_string(&DocumentField::from(MyIntegerMapping)).unwrap()
    });
}
