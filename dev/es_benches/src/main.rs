#![feature(plugin)]
#![plugin(elastic_types_codegen)]
#![plugin(serde_macros)]

extern crate stopwatch;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate elastic_types_codegen;

use stopwatch::{Stopwatch};
use std::io::Read;
use hyper::header::{ Headers, ContentType };
use hyper::client::response::Response;

fn main() {
	let mut sw_setup = Stopwatch::start_new();
		let baseurl = "http://localhost:9200";
		let index = "bench_index";
		let _type = "docs";
		
		let body = json!({
			query: {
				query_string: {
					default_field: "title",
					query: "doc"
				}
			}
		});

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
	sw_setup.stop();
	println!("Setup: {}ns", sw_setup.elapsed().num_nanoseconds().unwrap());

	let mut sw_hyper = Stopwatch::start_new();
		let mut client = hyper::Client::new();
		let mut headers = Headers::new();
		headers.set(ContentType::json());
	sw_hyper.stop();
	println!("Hyper: {}ns", sw_hyper.elapsed().num_nanoseconds().unwrap());

	let mut sw_exc = Stopwatch::start_new();
		let req = client.post(&url)
			.headers(headers)
			.body(body);
		let mut res = req.send().unwrap();
	sw_exc.stop();
	println!("HTTP: {}ns", sw_exc.elapsed().num_nanoseconds().unwrap());

	println!("Total: {}ns", 
		sw_setup.elapsed().num_nanoseconds().unwrap() + 
		sw_hyper.elapsed().num_nanoseconds().unwrap() + 
		sw_exc.elapsed().num_nanoseconds().unwrap()
	);
	println!("Total: {}ms", 
		sw_setup.elapsed().num_milliseconds() + 
		sw_hyper.elapsed().num_milliseconds() + 
		sw_exc.elapsed().num_milliseconds()
	);
}