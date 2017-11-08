use elastic_types;
use elastic_types::prelude::*;
use string_fixtures::*;

use test::Bencher;

#[bench]
fn keyword_mapping(b: &mut Bencher) {
    b.iter(|| {
        elastic_types::derive::standalone_field_ser(MyKeywordMapping).unwrap()
    });
}

#[bench]
fn text_mapping(b: &mut Bencher) {
    b.iter(|| {
        elastic_types::derive::standalone_field_ser(MyTextMapping).unwrap()
    });
}
