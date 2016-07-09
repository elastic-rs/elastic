extern crate crossbeam;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_tools;

#[macro_use]
extern crate lazy_static;

mod fsm;

use std::sync::mpsc;
use std::thread;
use rotor_tools::loop_ext::LoopInstanceExt;

//Define a global queue structure
lazy_static! {
	static ref QUEUE: fsm::Queue = fsm::Queue::new();
}

fn main() {
	//Spawn an io thread
	let (tx, rx) = mpsc::channel();
	let t = thread::spawn(move || {
		//Get a shared queue reference
		let queue = &QUEUE;

		//Build a loop
		let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
		let mut loop_inst = creator.instantiate(fsm::Context);

		//Add a state machine with a reference to our queue
		let handle = loop_inst.add_and_fetch(|fsm| fsm, |scope| {
			fsm::connect_localhost(scope, queue)
		}).unwrap();

		tx.send(handle).unwrap();

		loop_inst.run().unwrap();
	});
	
	//Get the FSM handle. This is for a single machine.
	//We could either combine them to use a Vec<Handle>, or a Handle with a Vec<Notifier>
	let handle = rx.recv().unwrap();

	//Block
	t.join().unwrap();
}
