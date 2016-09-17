//! # `elastic_rotor`
//! 
//! A WIP implementation of an asynchronous http client for Elasticsearch.
//! 
//! Only _sort of_ works... But will follow the following design:
//! - Provide a simple, fast constant connection pool
//! - Provide a more complex, but robust, sniffed connection pool
//! 
//! Communication to the loop is through a non-blocking `Queue`, wrapped in a `Client`.

extern crate time;

extern crate crossbeam;
extern crate futures;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_tools;

#[macro_use]
extern crate lazy_static;

mod client;
pub use client::*;

//Test usage
use futures::Future;

lazy_static! {
	static ref QUEUE: Queue = Queue::new();
}

fn main() {
	//Build a client
	let builder = ClientBuilder::new(&QUEUE)
		.connect_localhost();

	let cli = builder.build().wait().unwrap();

	//Run a post request asynchronously
	cli.req(Request::post("/testindex/testtype/1", b"{\"id\":1}"))
		.wait()
		.unwrap()
		.unwrap();

	//Run some search requests asynchronously
	let total_reqs = 100;
	let search_reqs: Vec<ResponseFuture> = (0..total_reqs).map(|_| {
		cli.req(Request::get("/testindex/testtype/_search"))
	}).collect();

	futures::collect(search_reqs).wait().unwrap();
}