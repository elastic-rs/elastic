use std::collections::BTreeMap;

use url::Url;
use crossbeam::sync::MsQueue;

#[derive(Clone, Copy)]
pub struct MachineId {
	pub token: usize,
	//TODO: Add host etc
}
pub struct ElasticRequest {
	pub url: Url
}

pub trait ElasticContext {
	fn add_queue(&mut self, id: &MachineId);
	fn queue_exists(&self, id: &MachineId) -> bool;
	fn try_pop(&self, id: &MachineId) -> Option<ElasticRequest>;
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
	fn add_queue(&mut self, id: &MachineId) {
		self.queues.insert(id.token, MsQueue::new());
	}

	fn queue_exists(&self, id: &MachineId) -> bool {
		self.queues.contains_key(&id.token)
	}

	fn try_pop(&self, id: &MachineId) -> Option<ElasticRequest> {
		self.queues.get(&id.token).unwrap().try_pop()
	}
}

mod conn;
mod req;

pub use self::conn::*;
pub use self::req::*;