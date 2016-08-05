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

extern crate crossbeam;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_tools;

#[macro_use]
extern crate lazy_static;

mod conn;


//Test usage

use std::sync::mpsc;
use std::thread;

use conn::constant;

//Define a global queue structure that will be shared by all producers / consumers
lazy_static! {
	static ref QUEUE: conn::Queue = conn::Queue::new();
}

fn main() {
	let (tx, rx) = mpsc::channel();

	//Spawn an io thread
	let t = thread::spawn(move || {
		let mut handle = constant::Handle::new(&QUEUE);

		//Build a loop
		let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
		let mut loop_inst = creator.instantiate(constant::Context);

		//Add a state machine with a reference to our queue
		loop_inst.add_machine_with(|scope| {
			constant::connect_localhost(scope, &mut handle)
		}).unwrap();

		loop_inst.add_machine_with(|scope| {
			constant::connect_localhost(scope, &mut handle)
		}).unwrap();

		//Send the constructed handle and start the loop
		tx.send(handle).unwrap();
		loop_inst.run().unwrap();
	});

	let handle = rx.recv().unwrap();

	//TODO: This needs to handle pushing a response back to the caller
	//Assume you want a response channel by default, allow calls to `push_no_resp` or something
	//Our codegen will probably wrap an initial call to `Message::verb`, taking the proper args
	//From then, we can use a builder to add extra details
	handle.push(conn::Message::post("/testindex/testtype/1", "{\"id\":1}".as_bytes()));
	handle.push(conn::Message::post("/testindex/testtype/2", "{\"id\":2}".as_bytes()));
	handle.push(conn::Message::post("/testindex/testtype/3", "{\"id\":3}".as_bytes()));

	//Block
	t.join().unwrap();
}
