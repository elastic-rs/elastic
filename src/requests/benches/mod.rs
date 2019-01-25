#![feature(test)]

extern crate elastic_requests;
extern crate test;

use elastic_requests::*;
use test::Bencher;

#[bench]
fn new_req(b: &mut Bencher) {
    b.iter(|| {
        let req = SearchRequest::for_index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

        test::black_box(req)
    });
}

#[bench]
fn owned_req_into_http_req(b: &mut Bencher) {
    b.iter(|| {
        let req = SearchRequest::for_index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

        let http_req: Endpoint<_> = (req).into();

        test::black_box(http_req)
    });
}
