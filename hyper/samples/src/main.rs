//! Elasticsearch Hyper Client Samples
//!
//! This sample executes a simple search request against a local cluster. No index or type info is provided.

#[macro_use]
extern crate json_str;
extern crate hyper;
extern crate elastic_hyper as elastic;

use hyper::client::Client;
use std::io::Read;

fn main() {
	//Create a hyper client
	let mut client = Client::new();
	let params = elastic::RequestParams::default()
		.url_params(vec![
			("pretty", "true".to_owned())
		]);

	//Execute a HTTP Post search request. Other variants include post_index, post_index_type
	let mut res = elastic::search::post(
		&mut client, &params,
		&json_str!({
			query: {
				query_string: {
					query: "*"
				}
			}
		})
	).unwrap();

	//Deserialisation goes here
	let mut message = String::new();
	let _ = res.read_to_string(&mut message);

	println!("Got response: {}", message);
}
