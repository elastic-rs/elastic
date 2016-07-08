use std::marker::PhantomData;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use crossbeam::sync::MsQueue;
use rotor::{ Notifier, Scope, GenericScope, Response, Void, Time };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::{ Client, Request, Requester, Persistent, Connection, ResponseError, ProtocolError, Task, Head, RecvMode };

pub struct Message;

pub trait QueueContext<'a> { }
pub struct Context<'a> {
	queue: &'a Queue
}

impl <'a> Context<'a> {
	pub fn new(queue: &'a Queue) -> Self {
		Context {
			queue: queue
		}
	}
}

impl <'a> QueueContext<'a> for Context<'a> {
	
}

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
}

pub struct Fsm<'a, C: QueueContext<'a>> {
	phantom_c: PhantomData<C>,
	phantom_a: PhantomData<&'a ()>
}

impl <'a, C: QueueContext<'a>> Client for Fsm<'a, C> {
	type Requester = RequestFsm<'a, C>;
    type Seed = Option<()>;

    //Create a machine that looks at a queue in the global context
    fn create(seed: Self::Seed, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
    	unimplemented!()
    }

    //If we're twiddling thumbs, try to pop a request from our queue
    fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
    	unimplemented!()
    }

    //Check for a new message
    fn wakeup(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
    	unimplemented!()
    }

    //Wakeup and check for a new message
    fn timeout(self, conn: &Connection, scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Task<Self> {
        unimplemented!()
    }

    fn connection_error(self, err: &ProtocolError, _scope: &mut Scope<C>) {
    	unimplemented!()
    }
}

pub struct RequestFsm<'a, C: QueueContext<'a>> {
	phantom_c: PhantomData<C>,
	phantom_a: PhantomData<&'a ()>
}

impl <'a, C: QueueContext<'a>> Requester for RequestFsm<'a, C> {
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

pub fn connect_localhost<S: GenericScope, C: QueueContext<'static>>(scope: &mut S) -> Response<(Persistent<Fsm<'static, C>, TcpStream>, Notifier), Void> {
    connect_addr(scope, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)))
}

pub fn connect_addr<S: GenericScope, C: QueueContext<'static>>(scope: &mut S, addr: SocketAddr) -> Response<(Persistent<Fsm<'static, C>, TcpStream>, Notifier), Void> {
    let notifier = scope.notifier();
    Persistent::connect(scope, addr, None).wrap(|fsm| {
        (fsm, notifier)
    })
}