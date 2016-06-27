//rotor_http scratchpad

//Expects Elasticsearch cluster at http://localhost:9200
//The loop won't close on its own, you have to kill the process yourself

//Adapted from: https://github.com/tailhook/rotor-http/blob/master/examples/client.rs
//TODO: Mess with this to build a generic Elasticsearch rotor impl
//Need to figure out what the best way to implement certain features is.

extern crate rotor;
extern crate rotor_tools;
extern crate rotor_http;
extern crate url;

use std::io::{stdout, stderr};
use std::io::Write;
use std::net::ToSocketAddrs;
use std::time::Duration;
use std::process::exit;
use std::thread;
use std::sync::mpsc;

use url::Url;
use rotor::{ Scope, Time };
use rotor_tools::loop_ext::LoopExt;
use rotor_http::client::{ connect_tcp, Request, Head, Client, RecvMode };
use rotor_http::client::{ Connection, Requester, Task, Version, ResponseError, ProtocolError };

struct Context;

struct Cli(Option<Url>);
struct Req(Url);

impl Client for Cli {
    type Requester = Req;
    type Seed = Url;
    fn create(seed: Self::Seed,
        _scope: &mut Scope<<Self::Requester as Requester>::Context>)
        -> Self
    {
    	println!("Cli.create");

        Cli(Some(seed))
    }
    fn connection_idle(mut self, _conn: &Connection,
        scope: &mut Scope<Context>)
        -> Task<Cli>
    {
    	//TODO: Pull req url off queue.
    	//Avoid spinning, if nothing is there, wait for a wakeup?

    	println!("Cli.connection_idle");

        match self.0.take() {
            Some(url) => {
            	Task::Request(Cli(None), Req(url))
            },
            None => {
                //scope.shutdown_loop();
                Task::Close
            }
        }
    }
    fn connection_error(self, err: &ProtocolError,
        _scope: &mut Scope<Context>)
    {
    	println!("Cli.connection_error");

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }
    fn wakeup(self,
        _connection: &Connection,
        _scope: &mut Scope<<Self::Requester as Requester>::Context>)
        -> Task<Cli>
    {
    	println!("Cli.wakeup");

        unimplemented!();
    }
    fn timeout(self,
        _connection: &Connection,
        _scope: &mut Scope<<Self::Requester as Requester>::Context>)
        -> Task<Cli>
    {
        unimplemented!();
    }
}

impl Requester for Req {
    type Context = Context;
    fn prepare_request(self, req: &mut Request,
        _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
    	println!("Req.prepare_request");

        req.start("GET", &self.0.path(), Version::Http11);
        req.add_header("Host", self.0.host_str().unwrap().as_bytes()).unwrap();
        req.add_header("Connection", b"keep-alive").unwrap();
        req.done_headers().unwrap();
        req.done();
        Some(self)
    }
    fn headers_received(self, head: Head, _request: &mut Request,
        scope: &mut Scope<Self::Context>)
        -> Option<(Self, RecvMode, Time)>
    {
    	println!("Req.headers_received");

        println!("----- Headers -----");
        println!("Status: {} {}", head.code, head.reason);
        for header in head.headers {
            println!("{}: {}", header.name,
                String::from_utf8_lossy(header.value));
        }
        Some((self,  RecvMode::Buffered(1 << 20),
            scope.now() + Duration::new(1000, 0)))
    }
    fn response_received(self, data: &[u8], _request: &mut Request,
        _scope: &mut Scope<Self::Context>)
    {
    	println!("Req.response_received");

        println!("----- Response -----");
        stdout().write_all(data).unwrap();
        if data.last() != Some(&b'\n') {
            println!("");
        }
    }
    fn response_chunk(self, _chunk: &[u8], _request: &mut Request,
        _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unreachable!();
    }
    fn response_end(self, _request: &mut Request,
        _scope: &mut Scope<Self::Context>)
    {
        unreachable!();
    }
    fn timeout(self, _request: &mut Request, _scope: &mut Scope<Self::Context>)
        -> Option<(Self, Time)>
    {
        unreachable!();
    }
    fn wakeup(self, _request: &mut Request, _scope: &mut Scope<Self::Context>)
        -> Option<Self>
    {
        unimplemented!();
    }
    fn bad_response(self, err: &ResponseError, _scope: &mut Scope<Context>)
    {
    	println!("Req.bad_response");

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }
}

fn main() {
	let url = Url::parse("http://localhost:9200").unwrap();
    let addr = url.to_socket_addrs().unwrap().next().unwrap();

    //TODO: Add notifier machine for registering new connections?
    //Use https://github.com/aturon/crossbeam/blob/master/src/sync/ms_queue.rs

	let handle = thread::spawn(move || {
	    let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
    	let mut loop_inst = creator.instantiate(Context);

	    loop_inst.add_machine_with(|scope| {
	        connect_tcp::<Cli>(scope, &addr, url.clone())
	    }).unwrap();

	    loop_inst.run().unwrap();
	});

	handle.join();
}