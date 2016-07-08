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
		let queue = fsm::Queue::new();

	    let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
		let mut loop_inst = creator.instantiate(fsm::Context::new(&QUEUE));

		let notifier = loop_inst.add_and_fetch(|fsm| fsm, |scope| {
	        fsm::connect_localhost(scope)
	    }).unwrap();

	    tx.send(fsm::Handle::new(&QUEUE, notifier)).unwrap();

	    loop_inst.run().unwrap();
	});
    
    let handle = rx.recv().unwrap();

    //Block
    t.join().unwrap();
}
