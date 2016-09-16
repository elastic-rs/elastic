//! # `elastic_rotor`
//! 
//! A WIP implementation of an asynchronous http client for Elasticsearch.
//! 
//! Only _sort of_ works... But will follow the following design:
//! - Provide a simple, fast constant connection pool
//! - Provide a more complex, but robust, sniffed connection pool
//! 
//! Communication to the loop is through a non-blocking `Queue`, wrapped in a `Handle`.

extern crate time;
extern crate stopwatch;
use time::Duration;
use stopwatch::Stopwatch;

extern crate crossbeam;
extern crate futures;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_tools;

#[macro_use]
extern crate lazy_static;

mod conn;

//Test usage
use std::str;
use futures::Future;

use conn::constant;

/*
Define a global queue structure that will be shared by all producers / consumers
We need to be clear that any messages on this queue may be handled, but putting a message
on the queue doesn't guarantee that it'll get handled now

TODO: Wrap this up so you don't have to build your own future type? Sounds good
impl Deref<Target = MsQueue<...>> for struct Queue(MsQueue<...>)

TODO: Also look into providing input to client pool as a futures Stream

TODO: Refactor the modules around. They're a mess right now. We probably won't need a separate
sniffed conn pool once the constance one is able to maintain health from a static list
*/
lazy_static! {
	static ref QUEUE: conn::Queue = conn::Queue::new();
}

fn main() {
	//Build a client
	//NOTE: The same addr can be added multiple times
	let builder = constant::ClientBuilder::new(&QUEUE)
		.add_localhost()
		.add_localhost();

	let client = builder.build().wait().unwrap();

	let sw = Stopwatch::start_new();

	//Run some requests asynchronously
	let total_reqs = 100;
	let reqs: Vec<conn::ReqFut> = (0..total_reqs).map(|_| {
		client.req(conn::Message::get("/testindex/testtype/_search"))
	}).collect();

	futures::collect(reqs).wait().unwrap();

	let elapsed = Duration::from_std(sw.elapsed()).unwrap();
	let elapsed = elapsed.num_nanoseconds().unwrap();

	println!("took {}ns ({}ns per req)", elapsed, elapsed / (total_reqs as i64));
}