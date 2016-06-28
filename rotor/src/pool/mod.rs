use std::collections::BTreeMap;

use url::Url;
use crossbeam::sync::MsQueue;

/// The attributes of a machine.
/// 
/// `State`s don't have to have unique attributes across machines, 
/// and can be used to share queues.
#[derive(Clone, Copy)]
pub struct State {
	/// An id for the 
	pub queue: usize,
	//TODO: Add host etc
}
pub struct ElasticRequest {
	pub url: Url
}

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

mod conn;
mod req;

pub use self::conn::*;
pub use self::req::*;