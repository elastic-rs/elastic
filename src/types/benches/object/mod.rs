use serde_json;
use elastic_types::prelude::*;
use object_fixtures::*;
use test::Bencher;

#[bench]
fn mapping_sml(b: &mut Bencher) {
    b.iter(|| serde_json::to_string(&MySmlType::index_mapping()).unwrap());
}

#[bench]
fn mapping_med(b: &mut Bencher) {
    b.iter(|| serde_json::to_string(&MyMedType::index_mapping()).unwrap());
}

#[bench]
fn mapping_lrg(b: &mut Bencher) {
    b.iter(|| serde_json::to_string(&MyLrgType::index_mapping()).unwrap());
}

#[bench]
fn mapping_value_sml(b: &mut Bencher) {
    b.iter(|| serde_json::to_value(&MySmlType::index_mapping()));
}

#[bench]
fn mapping_value_med(b: &mut Bencher) {
    b.iter(|| serde_json::to_value(&MyMedType::index_mapping()));
}

#[bench]
fn mapping_value_lrg(b: &mut Bencher) {
    b.iter(|| serde_json::to_value(&MyLrgType::index_mapping()));
}
