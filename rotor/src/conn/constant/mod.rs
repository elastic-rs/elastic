//! # Constant Connection Pool
//! 
//! A connection pool where the number of connections to the cluster and the addresses connected to don't change.
//! Messages are sent via a `Handle` to the pool, and are handled by any machine regardless of the connection.
//! 
//! The constant connection pool is fast to set up, but won't cope with node addresses that can change.

use std::str;
use std::marker::PhantomData;
use std::time::Duration;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use rotor::{ Notifier, Scope, GenericScope, Response, Void };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::{ Client, Requester, Persistent, Connection, ProtocolError, Task };
use super::{ Queue, Message, ApiRequest };

/// Connect a persistent state machine to a node running on `localhost:9200`.
pub fn connect_localhost<S: GenericScope, C>(scope: &mut S, handle: &mut Handle<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	connect_addr(scope, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)), handle)
}

/// Connect a persistent state machine to a node running at the given address.
pub fn connect_addr<S: GenericScope, C>(scope: &mut S, addr: SocketAddr, handle: &mut Handle<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	let queue = handle.add_listener(scope.notifier());

	Persistent::connect(scope, addr, queue)
}

/// A client-side handle to send request messages to a running loop.
pub struct Handle<'a> {
	queue: &'a Queue,
	notifiers: Vec<Notifier>
}

impl <'a> Handle<'a> {
	/// Create a new handle with no listeners.
	pub fn new(queue: &'a Queue) -> Self {
		Handle {
			queue: queue,
			notifiers: Vec::new()
		}
	}

	/// Add a machine as a listener on this handle's queue.
	pub fn add_listener(&mut self, notifier: Notifier) -> &'a Queue {
		self.notifiers.push(notifier);
		&self.queue
	}

	/// Push a message to the queue without blocking and notify listening machines.
	pub fn push(&self, msg: Message) {
		self.queue.push(msg);

		for notifier in &self.notifiers {
			notifier.wakeup().unwrap();
		}
	}
}

#[doc(hidden)]
pub struct Context;

/// A state machine for managing a persistent connection to an Elasticsearch node.
pub struct Fsm<'a, C> {
	queue: &'a Queue,
	_marker: PhantomData<C>
}

impl <'a, C> Client for Fsm<'a, C> {
	type Requester = ApiRequest<C>;
	type Seed = &'a Queue;

	fn create(seed: Self::Seed, _scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
		println!("client: create");

		Fsm {
			queue: seed,
			_marker: PhantomData
		}
	}

	fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
		//Look for a message without blocking
		if let Some(msg) = self.queue.try_pop() {
			println!("client: connection_idle: message: '{}', '{}'", msg.get_url(), str::from_utf8(msg.get_body()).unwrap());

			Task::Request(self, ApiRequest::for_msg(msg))
		}
		else {
			println!("client: connection_idle: no message");
			Task::Sleep(self, scope.now() + Duration::from_millis(2000))
		}
	}

	fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		if conn.is_idle() {
			println!("client: wakeup: idle");
			self.connection_idle(conn, scope)
		}
		else {
			println!("client: wakeup: not idle");
			Task::Sleep(self, scope.now() + Duration::from_millis(2000))
		}
	}

	fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		if conn.is_idle() {
			println!("client: timeout: idle");
			self.connection_idle(conn, scope)
		}
		else {
			println!("client: timeout: not idle");
			Task::Sleep(self, scope.now() + Duration::from_millis(2000))
		}
	}

	fn connection_error(self, _err: &ProtocolError, _scope: &mut Scope<C>) {
		unimplemented!()
	}
}