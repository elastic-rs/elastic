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
	let mut handle = fsm::Handle::new(&QUEUE);

	//Spawn an io thread
	let t = thread::spawn(move || {
		//Build a loop
		let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
		let mut loop_inst = creator.instantiate(fsm::Context);

		//Add a state machine with a reference to our queue
		loop_inst.add_machine_with(|scope| {
			fsm::connect_localhost(scope, &mut handle)
		}).unwrap();

		loop_inst.run().unwrap();
	});

	//Block
	t.join().unwrap();
}
