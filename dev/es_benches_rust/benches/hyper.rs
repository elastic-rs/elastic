#![feature(test, plugin)]
#![plugin(elastic_macros)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_macros;
extern crate hyper;

extern crate test;

use std::io::Read;
use test::Bencher;

use hyper::header::{ Headers, ContentType };
use hyper::client::response::Response;
use hyper::error::Result;

#[bench]
fn query(b: &mut Bencher) {
	b.iter(|| {
		let mut client = hyper::Client::new();
		post_index_type(
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

fn post_index_type(client: hyper::Client, baseurl: &str, index: &str, _type: &str, body: &str) -> Result<Response> {
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