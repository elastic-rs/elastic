use serde_json;
use elastic_types::prelude::*;
use ::object_fixtures::*;
use test::Bencher;

#[bench]
fn mapping_sml(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_string(&IndexDocumentMapping::from(MySmlMapping)).unwrap()
    });
}

#[bench]
fn mapping_med(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_string(&IndexDocumentMapping::from(MyMedMapping)).unwrap()
    });
}

#[bench]
fn mapping_lrg(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_string(&IndexDocumentMapping::from(MyLrgMapping)).unwrap()
    });
}

#[bench]
fn mapping_value_sml(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_value(&IndexDocumentMapping::from(MySmlMapping))
    });
}

#[bench]
fn mapping_value_med(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_value(&IndexDocumentMapping::from(MyMedMapping))
    });
}

#[bench]
fn mapping_value_lrg(b: &mut Bencher) {
    b.iter(|| {
        serde_json::to_value(&IndexDocumentMapping::from(MyLrgMapping))
    });
}