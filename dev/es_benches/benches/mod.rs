#![feature(test, plugin)]
#![plugin(elastic_types_codegen)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types_codegen;
extern crate rs_es;
extern crate hyper;

extern crate test;

use std::io::Read;
use test::Bencher;

use hyper::header::{ Headers, ContentType };
use hyper::client::response::Response;
use hyper::error::Result;
use rs_es::query::*;

/*

Working on a few ideas for the client implementation. 

The current json! macro seems to have horrible performance for some reason.
Will need to look at the output binary and see what's actually happening.

*/

#[bench]
fn rs_es_query(b: &mut Bencher) {
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

#[bench]
fn elastic_hyper_query_a(b: &mut Bencher) {
	b.iter(|| {
		let mut client = hyper::Client::new();
		post_index_type_a(
			client, "http://localhost:9200", "bench_index", "docs", 
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

#[bench]
fn elastic_hyper_query_aa(b: &mut Bencher) {
	b.iter(|| {
		let mut client = hyper::Client::new();
		post_index_type_a(
			client, "http://localhost:9200", "bench_index", "docs", 
			"{ 'query': { 'query_string': { 'default_field': 'title', 'query': 'doc' } } }"
		).unwrap()
	});
}

fn post_index_type_a(client: hyper::Client, baseurl: &str, index: &str, _type: &str, body: &str) -> Result<Response> {
	let mut url = String::with_capacity(
		baseurl.len() +
		"/".len() + 
		index.len() + 
		"/".len() +
		_type.len() +
		"/_search".len()
	);

	//Push the parts/params in order
	url.push_str(&baseurl);
	url.push_str("/");
	url.push_str(&index);
	url.push_str("/");
	url.push_str(&_type);
	url.push_str("/_search");

	let mut headers = Headers::new();
	headers.set(ContentType::json());
	
	let res = client.post(&url)
		.headers(headers)
		.body(body);
	res.send()
}