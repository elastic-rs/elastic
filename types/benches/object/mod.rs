use serde_json;
use elastic_types::prelude::*;
use ::object_fixtures::*;
use test::Bencher;

#[bench]
fn mapping_sml(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&Document::from(MySmlMapping)).unwrap()
	});
}

#[bench]
fn mapping_med(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&Document::from(MyMedMapping)).unwrap()
	});
}

#[bench]
fn mapping_lrg(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_string(&Document::from(MyLrgMapping)).unwrap()
	});
}

#[bench]
fn mapping_value_sml(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_value(&Document::from(MySmlMapping))
	});
}

#[bench]
fn mapping_value_med(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_value(&Document::from(MyMedMapping))
	});
}

#[bench]
fn mapping_value_lrg(b: &mut Bencher) {
	b.iter(|| {
		serde_json::to_value(&Document::from(MyLrgMapping))
	});
}