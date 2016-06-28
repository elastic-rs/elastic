/*

rotor_http scratchpad

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

*/

extern crate rotor;
extern crate rotor_tools;
extern crate rotor_http;
extern crate url;
extern crate crossbeam;

use std::net::ToSocketAddrs;
use std::thread;

use url::Url;
use rotor::mio::tcp::TcpStream;
use rotor_http::client::*;

mod pool;
use pool::*;

fn main() {
	let url = Url::parse("http://localhost:9200").unwrap();
    let addr = url.to_socket_addrs().unwrap().next().unwrap();

    //TODO: Add notifier machine for registering new connections?
	let handle = thread::spawn(move || {
	    let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
    	let mut loop_inst = creator.instantiate(Context::new());

	    loop_inst.add_machine_with(|scope| {
	    	//TODO: Add queue here?
	    	Persistent::<ElasticConnection<Context>, TcpStream>::connect(scope, addr.clone(), 0)
	    }).unwrap();

	    //TODO: Create a Router state machine

	    loop_inst.run().unwrap();
	});

	handle.join().unwrap();
}