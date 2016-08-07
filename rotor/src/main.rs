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
extern crate futures;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_tools;

#[macro_use]
extern crate lazy_static;

mod conn;

//Test usage
use std::str;
use std::time::Duration;
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
	//TODO: Make this a future too
	let pool = {
		thread::spawn(move || {
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
			//Using a future for this means we can avoid returning the value until the machines are connected
			tx.send(handle).unwrap();
			loop_inst.run().unwrap();
		});
		rx.recv().unwrap()
	};

	//Index some documents: don't wait for any success confirmation
	for i in 0..20 {
		pool.send(conn::Message::post(
			format!("/testindex/testtype/{}", i), 
			format!("{{\"id\":{}}}", i).as_bytes()
		));
	}

	//Search for documents
	loop {
		futures::Task::new().run(Box::new(
			pool.req(conn::Message::get(
				"/testindex/testtype/_search?pretty"
			))
			.then(|r| {
				print_resp(r.unwrap());
				futures::finished::<(), ()>(())
			})
		));

		thread::sleep(Duration::from_millis(2000));
	}
}

fn print_resp(r: conn::Data) {
	println!("{}", str::from_utf8(&r.unwrap()).unwrap());
	println!("----------");
	println!("");
}