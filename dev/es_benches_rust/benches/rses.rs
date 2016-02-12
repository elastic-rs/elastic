#![feature(test)]

extern crate rs_es;
extern crate test;

use test::Bencher;
use rs_es::query::*;

#[bench]
fn query(b: &mut Bencher) {
	b.iter(|| {
		let mut client = rs_es::Client::new("localhost", 9200);
		let query = Query::build_query_string("doc")
			.with_default_field("title")
			.build();

		client.search_query()
			.with_indexes(&["bench_index"])
			.with_types(&["docs"])
			.with_query(&query)
			.send()
			.unwrap()
	});
}