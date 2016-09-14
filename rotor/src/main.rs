//! # `elastic_rotor`
//! 
//! A WIP implementation of an asynchronous http client for Elasticsearch.
//! 
//! Doesn't work... But will follow the following design:
//! - Provide a simple, fast constant connection pool
//! - Provide a more complex sniffed connection pool
//! 
//! Communication to the loop is through a non-blocking `Queue`, wrapped in a `Handle`.
//! Responses (if wanted) will be retrieved through a channel, which blocks when requesting data.

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
use std::sync::mpsc;
use std::thread;
use futures::Future;

use conn::constant;

//Define a global queue structure that will be shared by all producers / consumers
lazy_static! {
	static ref QUEUE: conn::Queue = conn::Queue::new();
}

fn main() {
	let (tx, rx) = mpsc::channel();

	//Spawn an io thread
	//TODO: Make this a future too, and clean up the node addition
	//TODO: Maybe this should expect an &'static Queue?
	//let client = ClientBuilder::new(&QUEUE).add_node(addr).add_node(addr).build();
	//client.then(|c| { ... })
	let client = {
		thread::spawn(move || {
			let mut handle = constant::Handle::new(&QUEUE);

			//Build a loop
			let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
			let mut loop_inst = creator.instantiate(constant::Context);

			//Add a state machine with a reference to our queue
			for _ in 0..3 {
				loop_inst.add_machine_with(|scope| {
					constant::connect_localhost(scope, &mut handle)
				}).unwrap();
			}

			//Send the constructed handle and start the loop
			//Using a future for this means we can avoid returning the value until the machines are connected
			tx.send(handle).unwrap();
			loop_inst.run().unwrap();
		});
		rx.recv().unwrap()
	};

	let sw = Stopwatch::start_new();

	let total_reqs = 100;
	let reqs: Vec<conn::ReqFut> = (0..total_reqs).map(|_| {
		client.req(conn::Message::get("/testindex/testtype/_search"))
	}).collect();

	futures::collect(reqs).wait().unwrap();

	let elapsed = Duration::from_std(sw.elapsed()).unwrap();
	let elapsed = elapsed.num_nanoseconds().unwrap();

	println!("took {}ns ({}ns per req)", elapsed, elapsed / (total_reqs as i64));
}