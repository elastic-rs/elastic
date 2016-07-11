use std::marker::PhantomData;

use crossbeam::sync::MsQueue;
use rotor::{ Scope, Time };
use rotor_http::client::{ Request, Requester, ResponseError, Head, RecvMode };

pub mod constant;
pub mod sniffed;

/// A request message.
pub struct Message;

/// A common message queue shared by multiple machines.
pub type Queue = MsQueue<Message>;

/// A state machine for managing the HTTP component of an Elasticsearch connection.
pub struct ApiRequest<C> {
	_marker: PhantomData<C>
}

impl <C> Requester for ApiRequest<C> {
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