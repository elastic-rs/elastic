#![cfg(feature="test-integration")]

#![feature(test, plugin)]
#![plugin(json_str)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate json_str;
extern crate elastic_hyper as elastic;

extern crate test;

use test::Bencher;

use hyper::client::Client;
use hyper::client::response::Response;

#[bench]
fn search_post_index_type(b: &mut Bencher) {
	b.iter(|| {
		let mut client = hyper::Client::new();
		elastic::search::post_index_type(
			&mut client, "http://localhost:9200", "bench_index", "docs", 
			json!({
				query: {
					query_string: {
						default_field: "title",
						query: "doc"
					}
				}
			})
		).unwrap()
	});
}