use std::marker::PhantomData;
use std::time::Duration;

use rotor::Scope;
use rotor_http::client::{ Client as RotorClient, Requester, Connection, ProtocolError, Task };

use super::Queue;
use super::req::ApiRequest;

const DEFAULT_TIMEOUT: u64 = 500;

/*
TODO: We should probably have a single 'wakeup' machine that uses the `Context` to wake the other machines up
This way, when a connection needs to be respawned, we don't have to try and change anything client side
So it'll be more like the sniffed pool, just without an external source of truth
This means the only notifier the Handler has is one to the wakeup machine.
It also opens the door for only waking up machines that aren't currently busy or for doing other match logic
In that case though, we'd need to put connection machines on separate queues
*/

#[doc(hidden)]
pub struct Context;

/// A state machine for managing a persistent connection to an Elasticsearch node.
pub struct Fsm<'a, C> {
	q: &'a Queue,
	_c: PhantomData<C>
}

impl <'a, C> RotorClient for Fsm<'a, C> {
	type Requester = ApiRequest<C>;
	type Seed = &'a Queue;

	fn create(seed: Self::Seed, _scope: &mut Scope<<Self::Requester as Requester>::Context>) -> Self {
		Fsm {
			q: seed,
			_c: PhantomData
		}
	}

	fn connection_idle(self, _conn: &Connection, scope: &mut Scope<C>) -> Task<Self> {
		if let Some((req, fut)) = self.q.try_pop() {
			Task::Request(self, ApiRequest::for_req(req, fut))
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