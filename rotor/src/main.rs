/*

rotor_http scratchpad

Expects Elasticsearch cluster at http://localhost:9200
The loop won't close on its own, you have to kill the process yourself
Adapted from: https://github.com/tailhook/rotor-http/blob/master/examples/client.rs

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
*/

extern crate rotor;
extern crate rotor_tools;
extern crate rotor_http;
extern crate url;
extern crate crossbeam;

use std::io::{ stdout, stderr };
use std::io::Write;
use std::collections::BTreeMap;
use std::net::ToSocketAddrs;
use std::time::Duration;
use std::process::exit;
use std::thread;

use url::Url;
use rotor::{ Scope, Time };
use rotor_http::client::{ connect_tcp, Request, Head, Client, RecvMode };
use rotor_http::client::{ Connection, Requester, Task, Version, ResponseError, ProtocolError };
use crossbeam::sync::MsQueue;

//Loop context
struct Context {
	pub queues: BTreeMap<usize, MsQueue<Url>>
}
impl Context {
	pub fn new() -> Self {
		Context {
			queues: BTreeMap::new()
		}
	}
}

//State machine for connections
struct Cli(usize);
impl Client for Cli {
    type Requester = Req;
    type Seed = usize;

    fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
    	println!("{}: Cli.create", seed);

    	if !scope.queues.contains_key(&seed) {
    		println!("{}: Cli.create: creating queue", seed);

    		scope.queues.insert(seed, MsQueue::new());
    	}
    	else {
    		println!("{}: Cli.create: queue exists", seed);
    	}

        Cli(seed)
    }

    fn connection_idle(mut self, _conn: &Connection, scope: &mut Scope<Context>) -> Task<Cli> {
    	println!("{}: Cli.connection_idle", self.0);

    	let queue = scope.queues.get(&self.0).unwrap();

    	match queue.try_pop() {
			Some(url) => {
				println!("{}: Cli.connection_idle: found message", self.0);

				Task::Request(self, Req(url))
			},
			_ => {
				println!("{}: Cli.connection_idle: no message", self.0);

				Task::Sleep(self, scope.now() + Duration::from_millis(1000))
			}
		}
    }

    fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<Context>) {
    	println!("{}: Cli.connection_error", self.0);

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }
    fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Cli> {
    	println!("{}: Cli.wakeup", self.0);

        self.connection_idle(conn, scope)
    }

    fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Cli> {
        println!("{}: Cli.timeout", self.0);

        self.wakeup(conn, scope)
    }
}

//State machine for HTTP requests
struct Req(Url);
impl Requester for Req {
    type Context = Context;
    fn prepare_request(self, req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
    	println!("Req.prepare_request");

        req.start("GET", &self.0.path(), Version::Http11);
        req.add_header("Host", self.0.host_str().unwrap().as_bytes()).unwrap();
        req.add_header("Connection", b"keep-alive").unwrap();
        req.done_headers().unwrap();
        req.done();
        Some(self)
    }
    fn headers_received(self, head: Head, _request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
    	println!("Req.headers_received");

        println!("----- Headers -----");
        println!("Status: {} {}", head.code, head.reason);
        for header in head.headers {
            println!("{}: {}", header.name,
                String::from_utf8_lossy(header.value));
        }
        Some((self, RecvMode::Buffered(1 << 20),
            scope.now() + Duration::new(1000, 0)))
    }
    fn response_received(self, data: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) {
    	println!("Req.response_received");

        println!("----- Response -----");
        stdout().write_all(data).unwrap();
        if data.last() != Some(&b'\n') {
            println!("");
        }
    }
    fn response_chunk(self, _chunk: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
        unreachable!();
    }
    fn response_end(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) {
        unreachable!();
    }
    fn timeout(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
        unreachable!();
    }
    fn wakeup(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
        unimplemented!();
    }
    fn bad_response(self, err: &ResponseError, _scope: &mut Scope<Context>) {
    	println!("Req.bad_response");

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }
}

fn main() {
	let url = Url::parse("http://localhost:9200").unwrap();
    let addr = url.to_socket_addrs().unwrap().next().unwrap();

    //TODO: Add notifier machine for registering new connections?
	let handle = thread::spawn(move || {
	    let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
    	let mut loop_inst = creator.instantiate(Context::new());

    	//Add two machines listening on queue 0
	    loop_inst.add_machine_with(|scope| {
	        connect_tcp::<Cli>(scope, &addr, 0)
	    }).unwrap();
	    loop_inst.add_machine_with(|scope| {
	        connect_tcp::<Cli>(scope, &addr, 0)
	    }).unwrap();

	    //Add one machine listening on queue 1
	    loop_inst.add_machine_with(|scope| {
	        connect_tcp::<Cli>(scope, &addr, 1)
	    }).unwrap();

	    loop_inst.run().unwrap();
	});

	handle.join();
}