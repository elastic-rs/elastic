//! This is just a simple crate for mocking out simple client implementations
//! 
//! See the benches for progress.

#![feature(plugin)]
#![plugin(elastic_macros)]
#![plugin(serde_macros)]

extern crate rotor;
extern crate rotor_http;
extern crate time;
extern crate serde;
extern crate serde_json;
extern crate elastic_macros;

use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::{ Sender, Receiver };
use std::io::stdout;
use std::io::Write;
use std::net::ToSocketAddrs;

use rotor::{ Scope };
use rotor_http::client::{ connect_tcp, Request, Head, Client, RecvMode };
use rotor_http::client::{ Context as HttpCtx };
use rotor_http::Version::Http11;
use rotor_http::Deadline;

/*

Messing around with the Rotor-HTTP client API. This is really messy right now, but I'll tidy it up as I go.
So far, the level of control looks good, if I can avoid unnecessary string construction along the way I will.

A callback API would also be good.

Each request is a state machine, and each request is doing io on a socket registered with mio.
The io runs on its own io thread, which prevents blocking the app code while requests are in progress.

I still need to figure out some specifics for sending new requests to the io thread etc. 
But these are all concerns of the eventual client that comes out of this.

*/

struct Context;
impl HttpCtx for Context {}

//Req(Url, Body, Sender)
struct Req(String, String, Sender<String>);
impl Client for Req {
	type Context = Context;
	fn prepare_request(self, req: &mut Request) -> Option<Self> {
		let body = self.1.clone();
		let body_bytes = body.as_bytes();

		req.start("POST", &self.0, Http11);

		req.add_length(body_bytes.len() as u64);
		req.add_header("content-type", "application/json".as_bytes());
		req.done_headers().unwrap();

		req.write_body(body_bytes);
		req.done();
		Some(self)
	}
	fn headers_received(self, head: Head, _request: &mut Request,
		_scope: &mut Scope<Self::Context>)
		-> Option<(Self, RecvMode, Deadline)>
	{
		Some((self,  RecvMode::Buffered(16386), Deadline::now() +
			time::Duration::seconds(1000)))
	}
	fn response_received(self, data: &[u8], _request: &mut Request,
		scope: &mut Scope<Self::Context>)
	{
		println!("response");
		self.2.send(std::str::from_utf8(data).unwrap().to_string());
		//scope.shutdown_loop();
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
		-> Option<(Self, Deadline)>
	{
		unreachable!();
	}
	fn wakeup(self, _request: &mut Request, _scope: &mut Scope<Self::Context>)
		-> Option<Self>
	{
		unimplemented!();
	}
}

fn main() {
	//IO thread
	let (tx, rx) = mpsc::channel();
	let handle = thread::spawn(move|| {
		let creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
		let mut loop_inst = creator.instantiate(Context);
		loop_inst.add_machine_with(|scope| {
			connect_tcp(scope,
				&("localhost", 9200)
					.to_socket_addrs()
					.unwrap().collect::<Vec<_>>()[0],
				Req(
					"/bench_index/docs/_search?pretty".to_string(), 
					json!({
						query: {
							query_string: {
								default_field: "title",
								query: "doc"
							}
						}
					}).to_string(),
					tx.clone()
				)
			)
		}).unwrap();

		loop_inst.add_machine_with(|scope| {
			connect_tcp(scope,
				&("localhost", 9200)
					.to_socket_addrs()
					.unwrap().collect::<Vec<_>>()[0],
				Req(
					"/bench_index/docs/_search?pretty".to_string(), 
					json!({
						query: {
							query_string: {
								default_field: "title",
								query: "doc"
							}
						}
					}).to_string(),
					tx.clone()
				)
			)
		}).unwrap();

		loop_inst.run().unwrap();
	});

	let mut result = rx.recv().unwrap();
	println!("{}", result);
	println!("-------------------------");
	let result = rx.recv().unwrap();
	println!("{}", result);
}