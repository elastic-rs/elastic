use std::marker::PhantomData;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use crossbeam::sync::MsQueue;
use rotor::{ Notifier, Scope, GenericScope, Response, Void, Time, WakeupError };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::{ Client, Request, Requester, Persistent, Connection, ResponseError, ProtocolError, Task, Head, RecvMode };

pub struct Message;
pub type Queue = MsQueue<Message>;

pub struct Handle<'a> {
	queue: &'a Queue,
	notifier: Notifier
}

impl <'a> Handle<'a> {
	pub fn new(queue: &'a Queue, notifier: Notifier) -> Self {
		Handle {
			queue: queue,
			notifier: notifier
		}
	}

	pub fn wakeup(&self) -> Result<(), WakeupError> {
		self.notifier.wakeup()
	}

	pub fn push(&self, msg: Message) {
		self.queue.push(msg)
	}
}

//NOTE: Could add global queue here with a trait
pub struct Context;

//Our general state machine
pub struct Fsm<'a, C> {
	queue: &'a Queue,
	_marker: PhantomData<C>
}

impl <'a, C> Client for Fsm<'a, C> {
	type Requester = RequestFsm<C>;
	type Seed = &'a Queue;

	fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
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

//Our HTTP state machine
pub struct RequestFsm<C> {
	_marker: PhantomData<C>
}

impl <C> Requester for RequestFsm<C> {
	type Context = C;

	fn prepare_request(self, req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		unimplemented!();
	}

	fn headers_received(self, head: Head, _request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
		unimplemented!();
	}

	fn response_received(self, data: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		unimplemented!();
	}

	fn bad_response(self, err: &ResponseError, _scope: &mut Scope<Self::Context>) {
		unimplemented!();
	}

	fn response_chunk(self, _chunk: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		unimplemented!();
	}
	fn response_end(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		unimplemented!();
	}
	fn timeout(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
		unimplemented!();
	}
	fn wakeup(self, request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<Self> {
		unimplemented!();
	}
}

pub fn connect_localhost<S: GenericScope, C>(scope: &mut S, queue: &'static Queue) -> Response<(Persistent<Fsm<'static, C>, TcpStream>, Handle<'static>), Void> {
	connect_addr(scope, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)), queue)
}

pub fn connect_addr<S: GenericScope, C>(scope: &mut S, addr: SocketAddr, queue: &'static Queue) -> Response<(Persistent<Fsm<'static, C>, TcpStream>, Handle<'static>), Void> {
	let notifier = scope.notifier();
	Persistent::connect(scope, addr, queue).wrap(|fsm| {
		(fsm, Handle::new(queue, notifier))
	})
}