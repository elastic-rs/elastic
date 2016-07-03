use std::collections::BTreeMap;
use std::net::{ SocketAddr, SocketAddrV4, Ipv4Addr };

use rotor::{ GenericScope, Void, Response, Notifier };
use rotor::mio::tcp::TcpStream;
use rotor_http::client::Persistent;
use url::Url;
use crossbeam::sync::MsQueue;

mod conn;
mod req;

pub use self::conn::*;
pub use self::req::*;

/// The attributes of a connection.
/// 
/// `State`s don't have to have unique attributes across connections, 
/// and can be used to share queues.
#[derive(Clone, Copy)]
pub struct State {
	/// The id of the queue to process requests from.
	pub queue: usize,
	//TODO: Add host etc
}

/// A request that can be processed by a connection.
pub struct ElasticRequest {
	/// The REST API url to call.
	pub url: Url
}

/// A global loop context required by `ElasticConnection`s.
/// 
/// The context stores a collection of queues that are accessed by individual connections.
pub trait ElasticContext {
	fn add_queue(&mut self, id: usize);
	fn queue_exists(&self, id: &usize) -> bool;
	fn try_pop(&self, id: &usize) -> Option<ElasticRequest>;
	fn push(&self, id: &usize, req: ElasticRequest) -> Option<()>;
}

pub struct Context {
	queues: BTreeMap<usize, MsQueue<ElasticRequest>>
}
impl Context {
	pub fn new() -> Self {
		Context {
			queues: BTreeMap::new()
		}
	}
}
impl ElasticContext for Context {
	fn add_queue(&mut self, id: usize) {
		self.queues.insert(id, MsQueue::new());
	}

	fn queue_exists(&self, id: &usize) -> bool {
		self.queues.contains_key(&id)
	}

	fn try_pop(&self, id: &usize) -> Option<ElasticRequest> {
		if let Some(queue) = self.queues.get(&id) {
			queue.try_pop()
		}
		else {
		    None
		}
	}

	fn push(&self, id: &usize, req: ElasticRequest) -> Option<()> {
		if let Some(queue) = self.queues.get(&id) {
			queue.push(req);
			Some(())
		}
		else {
			None
		}
	}
}

pub fn connect_localhost<S: GenericScope, C: ElasticContext>(scope: &mut S, seed: usize) -> Response<(Persistent<ElasticConnection<C>, TcpStream>, Notifier), Void> {
    connect_addr(scope, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9200)), seed)
}

pub fn connect_addr<S: GenericScope, C: ElasticContext>(scope: &mut S, addr: SocketAddr, seed: usize) -> Response<(Persistent<ElasticConnection<C>, TcpStream>, Notifier), Void> {
    let notifier = scope.notifier();
    Persistent::connect(scope, addr, seed).wrap(|fsm| {
        (fsm, notifier)
    })
}