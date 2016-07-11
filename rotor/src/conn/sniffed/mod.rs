//! # Sniffed Connection Pool
//! 
//! Connections to a cluster are sniffed from the `_cluster/stats` endpoint.
//! The caller creates a single `Sniffer` with an initial address, and connections are spawned from it.
//! 
//! When a message arrives in the pool, the `Sniffer` will wakeup the connections and one of them will handle the message.
//! 
//! The `Sniffer` will periodically refresh the addresses of active nodes, and spawn machines to accommodate them.
//! The number of active machines won't exceed a user-defined maximum.
//! 
//! The sniffed connection pool is more robust in environments where node addresses aren't static, but requires more plumbing.

//! # Constant Connection Pool
//! 
//! A connection pool where the number of connections to the cluster and the addresses connected to don't change.
//! Messages are sent via a `Handle` to the pool, and are handled by any machine regardless of the connection.
//! 
//! The constant connection pool is fast to set up, but won't cope with node addresses that can change.

use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use rotor::{ Notifier, Scope, GenericScope, Response, WakeupError, Void };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::{ Client, Requester, Persistent, Connection, ProtocolError, Task };
use super::{ Queue, Message, ApiRequest };

/// Connect a persistent state machine to a node running on `localhost:9200`.
pub fn connect_localhost<S: GenericScope, C: SniffedContext>(scope: &mut S, handle: &mut Handle<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	connect_addr(scope, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)), handle)
}

/// Connect a persistent state machine to a node running at the given address.
pub fn connect_addr<S: GenericScope, C: SniffedContext>(scope: &mut S, addr: SocketAddr, handle: &mut Handle<'static>) -> Response<Persistent<Fsm<'static, C>, TcpStream>, Void> {
	let queue = handle.set_sniffer(scope.notifier());

	Persistent::connect(scope, addr, queue)
}

/// A client-side handle to send request messages to a running loop.
pub struct Handle<'a> {
	queue: &'a Queue,
	notifier: Option<Notifier>
}

impl <'a> Handle<'a> {
	/// Create a new handle with no listeners.
	pub fn new(queue: &'a Queue) -> Self {
		Handle {
			queue: queue,
			notifier: None
		}
	}

	pub fn set_sniffer(&mut self, notifier: Notifier) -> &'a Queue {
		self.notifier = Some(notifier);
		&self.queue
	}

	/// Push a message to the queue without blocking and notify listening machines.
	pub fn push(&self, msg: Message) -> Result<(), WakeupError> {
		self.queue.push(msg);

		if let Some(ref notifier) = self.notifier {
			notifier.wakeup()
		}
		else {
			Ok(())
		}
	}

	/// Try pop a message off the queue without blocking.
	pub fn pop(&self) -> Option<Message> {
		self.queue.try_pop()
	}
}

/// A connection that is locked to a machine by its `Notifier`.
struct ConnectedNode(pub String, pub Notifier);

pub trait SniffedContext {
	/// Add a new idle connection address.
	fn add(&mut self, addr: String);
	/// Connect any idle connection address.
	fn connect(&mut self, notifier: Notifier) -> Option<String>;
	/// Poison a connected connection address.
	fn poison(&mut self, addr: String);
}
pub struct Context {
	idle: Vec<String>,
	conn: Vec<ConnectedNode>,
	poisoned: Vec<String>
}
impl Context {
	pub fn new() -> Self {
		Context {
			idle: Vec::new(),
			conn: Vec::new(),
			poisoned: Vec::new()
		}
	}
}

impl SniffedContext for Context {
	fn add(&mut self, addr: String) {
		self.idle.push(addr);
	}

	fn connect(&mut self, notifier: Notifier) -> Option<String> {
		if let Some(idle) = self.idle.pop() {
			self.conn.push(ConnectedNode(idle.clone(), notifier));
			Some(idle)
		}
		else {
			None
		}
	}

	fn poison(&mut self, addr: String) {
		unimplemented!()
	}
}

/// A state machine for managing active connections to nodes in an Elasticsearch cluster.
pub struct Sniffer<'a, C: SniffedContext> {
	queue: &'a Queue,
	_marker: PhantomData<C>
}

impl <'a, C: SniffedContext> Client for Sniffer<'a, C> {
	type Requester = ApiRequest<C>;
	type Seed = (&'a Queue, String);

	fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
		//Same as timeout
		let (queue, addr) = seed;
		
		Sniffer {
			queue: queue,
			_marker: PhantomData
		}
	}

	fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
		//Same as timeout
		//Check for new connections to spawn

		unimplemented!()
	}

	fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		//Wakeup notifiers in all active connections
		//Check for new connections to spawn
		unimplemented!()
	}

	fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		//Choose any active connection address and use to sniff
		unimplemented!()
	}

	fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<C>) {
		unimplemented!()
	}
}

/// A state machine for managing a persistent connection to an Elasticsearch node.
pub struct Fsm<'a, C: SniffedContext> {
	queue: &'a Queue,
	_marker: PhantomData<C>
}

impl <'a, C: SniffedContext> Client for Fsm<'a, C> {
	type Requester = ApiRequest<C>;
	type Seed = &'a Queue;

	fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
		//Get a connection from the context
		Fsm {
			queue: seed,
			_marker: PhantomData
		}
	}

	fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
		//Look for a message without blocking
		if let Some(msg) = self.queue.try_pop() {
			//Handle
		}
		else {
			//Snooze
		}

		unimplemented!()
	}

	fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		unimplemented!()
	}

	fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
		unimplemented!()
	}

	fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<C>) {
		unimplemented!()
	}
}