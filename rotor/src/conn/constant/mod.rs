//! # Constant Connection Pool
//! 
//! A connection pool where the number of connections to the cluster and the addresses connected to don't change.
//! Messages are sent via a `Handle` to the pool, and are handled by any machine regardless of the connection.
//! 
//! The constant connection pool is fast to set up, but won't cope with node addresses that can change.

use std::marker::PhantomData;
use std::thread;
use std::time::Duration;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use futures::{ oneshot, Oneshot };
use rotor::{ Config, Notifier, Scope, GenericScope, Response, Void, Loop };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::{ Client as FsmClient, Requester, Persistent, Connection, ProtocolError, Task };
use super::{ Message, Queue, ApiRequest, ReqFut };

/// A client-side handle to send request messages to a running loop.
pub struct Client<'a> {
	queue: &'a Queue,
	notifiers: Vec<Notifier>
}

impl <'a> Client<'a> {
	/// Create a new handle with no listeners.
	fn new(queue: &'a Queue) -> Self {
		Client {
			queue: queue,
			notifiers: Vec::new()
		}
	}

	/// Add a machine as a listener on this handle's queue.
	fn add_listener(&mut self, notifier: Notifier) -> &'a Queue {
		self.notifiers.push(notifier);
		&self.queue
	}

	/// Push a message to the queue and return a promise representing the response.
	pub fn req(&self, msg: Message) -> ReqFut {
		let (c, p) = oneshot();

		self.queue.push((msg, c));

		//TODO: Come up with a better strategy for wakeups
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

	pub fn add_localhost(mut self) -> Self {
		self.addrs.push(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)));

		self
	}

	pub fn add(mut self, addr: SocketAddr) -> Self {
		self.addrs.push(addr);

		self
	}

	pub fn build(self) -> Oneshot<Client<'static>> {
		let (c, p) = oneshot();

		let addrs = self.addrs;
		let mut client = self.client;

		thread::spawn(move || {
			//Build a loop
			let creator = Loop::new(&Config::new()).unwrap();
			let mut loop_inst = creator.instantiate(Context);

			//Add a state machine with a reference to our queue
			for addr in addrs {
				loop_inst.add_machine_with(|scope| {
					connect_addr(scope, addr, &mut client)
				}).unwrap();
			}

			c.complete(client);
			
			loop_inst.run().unwrap();
		});

		p
	}
}

/// Connect a persistent state machine to a node running on `localhost:9200`.
fn connect_localhost<S: GenericScope, C>(scope: &mut S, handle: &mut Client<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	connect_addr(scope, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)), handle)
}

/// Connect a persistent state machine to a node running at the given address.
fn connect_addr<S: GenericScope, C>(scope: &mut S, addr: SocketAddr, handle: &mut Client<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	let queue = handle.add_listener(scope.notifier());

	Persistent::connect(scope, addr, queue)
}

const DEFAULT_TIMEOUT: u64 = 500;

/*
TODO: We should probably have a single 'wakeup' machine that uses the `Context` to wake the other machines up
This way, when a machine dies and is reborn, we don't have to try and change anything client side
So it'll be more like the sniffed pool, just without an external source of truth
This means the only notifier the Handler has is one to the wakeup machine.
It also opens the door for only waking up machines that aren't currently busy or for doing other match logic
In that case though, we'd need to put connection machines on separate queues
*/

#[doc(hidden)]
pub struct Context;

/// A state machine for managing a persistent connection to an Elasticsearch node.
pub struct Fsm<'a, C> {
	queue: &'a Queue,
	_marker: PhantomData<C>
}

impl <'a, C> FsmClient for Fsm<'a, C> {
	type Requester = ApiRequest<C>;
	type Seed = &'a Queue;

	fn create(seed: Self::Seed, _scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
		Fsm {
			queue: seed,
			_marker: PhantomData
		}
	}

	fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
		if let Some((msg, future)) = self.queue.try_pop() {
			Task::Request(self, ApiRequest::for_msg(msg, future))
		}
		else {
			Task::Sleep(self, scope.now() + Duration::from_millis(DEFAULT_TIMEOUT))
		}
	}

	fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		self.connection_idle(conn, scope)
	}

	fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		if conn.is_idle() {
			self.connection_idle(conn, scope)
		}
		else {
			Task::Sleep(self, scope.now() + Duration::from_millis(DEFAULT_TIMEOUT))
		}
	}

	fn connection_error(self, _err: &ProtocolError, _scope: &mut Scope<C>) {
		//TODO: On connection error, we need to do something... The handler needs to know things have changed
		unimplemented!()
	}
}