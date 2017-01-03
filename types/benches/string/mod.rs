use serde_json;
use elastic_types::prelude::*;
use ::string_fixtures::*;

use test::Bencher;

#[bench]
fn keyword_mapping(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_string(&Field::from(MyKeywordMapping)).unwrap()
    });
}

#[bench]
fn text_mapping(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_string(&Field::from(MyTextMapping)).unwrap()
    });
}