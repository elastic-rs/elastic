#![feature(test)]

extern crate test;
extern crate elastic_requests;

use test::Bencher;
use elastic_requests::*;

#[bench]
fn new_req(b: &mut Bencher) {
	b.iter(|| {
		let req = SearchRequestParams::index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

		test::black_box(req)
	});
}

#[bench]
fn ref_req_into_http_req(b: &mut Bencher) {
	let req = SearchRequestParams::index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

	b.iter(|| {
		let http_req: HttpRequest = (&req).into();

		test::black_box(http_req)
	});
}