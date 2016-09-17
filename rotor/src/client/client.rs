use std::thread;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use futures::{ oneshot, Oneshot };
use rotor::{ Config, Notifier, GenericScope, Response, Void, Loop };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::Persistent;

use super::{ Request, Queue, ResponseFuture };
use super::fsm::{ Fsm, Context };

/// A client-side handle to send request messages to a running loop.
#[derive(Clone)]
pub struct Client<'a> {
	q: &'a Queue,
	notifiers: Vec<Notifier>
}

impl <'a> Client<'a> {
	/// Create a new handle with no listeners.
	fn new(q: &'a Queue) -> Self {
		Client {
			q: q,
			notifiers: Vec::new()
		}
	}

	/// Add a machine as a listener on this handle's queue.
	fn add_listener(&mut self, notifier: Notifier) -> &'a Queue {
		self.notifiers.push(notifier);
		&self.q
	}

	/// Push a message to the queue and return a promise representing the response.
	pub fn req(&self, req: Request) -> ResponseFuture {
		let (c, p) = oneshot();

		self.q.push((req, c));
		self.wakeup();

		p
	}

	/// Attempt to wake up any active connection handlers.
	/// 
	/// This will ensure that any messages added to the request queue outside of
	/// this `Handler` will get picked up as quickly as possible.
	fn wakeup(&self) {
		for notifier in &self.notifiers {
			notifier.wakeup().unwrap();
		}
	}
}

pub struct ClientBuilder {
	client: Client<'static>,
	addrs: Vec<SocketAddr>
}

impl ClientBuilder {
	pub fn new(queue: &'static Queue) -> Self {
		ClientBuilder {
			client: Client::new(queue),
			addrs: Vec::new()
		}
	}

	pub fn connect_localhost(mut self) -> Self {
		self.addrs.push(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)));

		self
	}

	pub fn connect(mut self, addr: SocketAddr) -> Self {
		self.addrs.push(addr);

		self
	}

	pub fn build(self) -> Oneshot<Client<'static>> {
		let (c, p) = oneshot();

		let addrs = self.addrs;
		let mut client = self.client;

		thread::spawn(move || {
			let creator = Loop::new(&Config::new()).unwrap();
			let mut pool = creator.instantiate(Context);

			for addr in addrs {
				pool.add_machine_with(|scope| {
					connect_addr(scope, addr, &mut client)
				}).unwrap();
			}

			c.complete(client);
			
			pool.run().unwrap();
		});

		p
	}
}

/// Connect a persistent state machine to a node running at the given address.
fn connect_addr<S: GenericScope, C>(scope: &mut S, addr: SocketAddr, client: &mut Client<'static>) 
-> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	let q = client.add_listener(scope.notifier());

	Persistent::connect(scope, addr, q)
}