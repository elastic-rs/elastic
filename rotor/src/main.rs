/*

rotor_http scratchpad: First Principals Implementation

Expects Elasticsearch cluster at http://localhost:9200
The loop won't close on its own, you have to kill the process yourself
Adapted from: https://github.com/tailhook/rotor-http/blob/master/examples/ElasticConnectionent.rs

Need to figure out what the best way to implement certain features is.

Select a machine to execute request on with a `Selector`:

enum Selector {
	Any,
	Specific(Token)
}

where the `Token` is a `mio::Token` that defaults to some function of the hostname.
If the machine with the given token doesn't exist, then it should be created.

That way users can choose to execute requests round-robin, all hosts on a single machine,
or some API calls, like `_bulk` to a different machine than `_search`.

Need to figure out how to get access to the internal queues to push data.
Not sure how this will work. Should we allow references to queues to be returned?

Push a message to the queue, then call wakeup on all machines registered to that queue.

Need to determine whether the queue approach is idiomatic for rotor. 
Seems to be designed to compose state machines as a hierarchy rather than abstracting them.
Not sure how to do this yet though...

One possible option is to adjust the `ElasticConnection` state to be something like:

```
enum State {
	Idle,
	Processing(ElasticRequest)
}
```

Where a central machine with a queue will assign messages to machines and wake them up.

*/

extern crate rotor;
extern crate rotor_tools;
extern crate rotor_http;
extern crate url;
extern crate crossbeam;

use std::time;
use std::net::ToSocketAddrs;
use std::sync::mpsc;
use std::thread;

use url::Url;
use rotor::Notifier;
use rotor::mio::tcp::TcpStream;
use rotor_http::client::*;
use rotor_tools::loop_ext::LoopInstanceExt;

mod pool;
use pool::*;

fn main() {
	let url = Url::parse("http://localhost:9200").unwrap();
    let addr = url.to_socket_addrs().unwrap().next().unwrap();

    //TODO: Look at using crossbeam scope for this?
    //We need to be able to pass a reference to a work queue to our machines and the outside world
    let (tx, rx) = mpsc::channel();
	let handle = thread::spawn(move || {
	    let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
    	let mut loop_inst = creator.instantiate(Context::new());

    	let notifier = loop_inst.add_and_fetch(|fsm| fsm, |scope| {
	        connect_localhost(scope, 0)
	    }).unwrap();

	    let _ = tx.send(notifier);

	    //TODO: Create a Router state machine. Need a way to get requests into the loop

	    loop_inst.run().unwrap();
	});

	let notifier = rx.recv().unwrap();

	//Dirty wait to see if this works
	thread::sleep(time::Duration::from_secs(2));
	for _ in 0..10 {
		println!("main: wakeup");
		notifier.wakeup();
	}

	handle.join().unwrap();
}