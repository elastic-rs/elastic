use std::marker::PhantomData;

use crossbeam::sync::MsQueue;
use rotor::{ Scope, Time };
use rotor_http::client::{ Request, Requester, ResponseError, Head, RecvMode };

pub mod constant;
pub mod sniffed;

/// A request message.
pub struct Message {
	url: String,
	body: Vec<u8>
}
impl Message {
	/// Create a new POST request.
	pub fn post<I: Into<String>>(url: I, body: &[u8]) -> Self {
		Message {
			url: url.into(),
			body: body.to_vec()
		}
	}

	/// Get the url for this request.
	pub fn get_url(&self) -> &str {
		&self.url
	}

	/// Get the message body for this request.
	pub fn get_body(&self) -> &[u8] {
		&self.body
	}
}

/// A common message queue shared by multiple machines.
pub type Queue = MsQueue<Message>;

/// A state machine for managing the HTTP component of an Elasticsearch connection.
pub struct ApiRequest<C> {
	msg: Message,
	_marker: PhantomData<C>
}

impl <C> ApiRequest<C> {
	pub fn for_msg(msg: Message) -> Self {
		ApiRequest {
			msg: msg,
			_marker: PhantomData
		}
	}
}

impl <C> Requester for ApiRequest<C> {
	type Context = C;

	fn prepare_request(self, req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		println!("requester: prepare_request");

		unimplemented!();
	}

	fn headers_received(self, head: Head, _request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
		println!("requester: headers_received");

		unimplemented!();
	}

	fn response_received(self, data: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		println!("requester: response_received");

		unimplemented!();
	}

	fn bad_response(self, err: &ResponseError, _scope: &mut Scope<Self::Context>) {
		println!("requester: bad_response");

		unimplemented!();
	}

	fn response_chunk(self, _chunk: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		println!("requester: response_chunk");

		unimplemented!();
	}
	fn response_end(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		println!("requester: response_end");

		unimplemented!();
	}
	fn timeout(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
		println!("requester: timeout");

		unimplemented!();
	}
	fn wakeup(self, request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<Self> {
		println!("requester: wakeup");

		unimplemented!();
	}
}