//! Elasticsearch Hyper Client Samples
//! 
//! This sample executes a simple search request against a local cluster. No index or type info is provided.

#![feature(plugin)]
#![plugin(serde_macros, elastic_macros)]

//Hyper is the base HTTP library that we sit on top of
extern crate hyper;
use hyper::client::Client;
use std::io::Read;

//API functions live in elastic_hyper
extern crate elastic_hyper as elastic;

fn main() {
	//Create a hyper client
	let mut client = Client::new();

	//Execute a HTTP Post search request. Other variants include post_index, post_index_type
	let mut res = elastic::search::post(
		&mut client, "http://localhost:9200",
		json!({
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