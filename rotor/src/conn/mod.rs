use std::marker::PhantomData;
use std::time::Duration;
use crossbeam::sync::MsQueue;
use futures::{ Oneshot, Complete };
use rotor::{ Scope, Time };
use rotor_http::client::{ Request, Requester, ResponseError, Head, RecvMode, Version };

pub mod constant;
//pub mod sniffed;

/// A request message.
/// 
/// This is what you supply to kick off a request.
pub struct Message {
	url: String,
	verb: &'static str,
	body: Option<Vec<u8>>
}
impl Message {
	/// Create a new GET request.
	pub fn get<I: Into<String>>(url: I) -> Self {
		Message {
			url: url.into(),
			verb: "GET",
			body: None
		}
	}

	/// Create a new POST request.
	pub fn post<I: Into<String>>(url: I, body: &[u8]) -> Self {
		Message {
			url: url.into(),
			verb: "POST",
			body: Some(body.to_vec())
		}
	}
}

/// A common data format shared between producer and consumer.
pub type Data = Result<Vec<u8>, &'static str>;

/// A request future.
pub type ReqFut = Oneshot<Data>;

/// The completion part of a request future.
type ReqComp = Complete<Data>;

const DEFAULT_TIMEOUT: u64 = 1000;

/// A queue to link a client with a connection pool.
/// 
/// This is essentially just a wrapped `MsQueue`.
/// Messages can't be put onto this queue directly, you need to use the
/// appropriate `Client` structure.
pub struct Queue(MsQueue<(Message, ReqComp)>);
impl Queue {
	pub fn new() -> Self {
		Queue(MsQueue::new())
	}

	fn push(&self, msg: (Message, ReqComp)) {
		self.0.push(msg);
	}

	fn try_pop(&self) -> Option<(Message, ReqComp)> {
		self.0.try_pop()
	}
}

/// A state machine for managing the HTTP component of an Elasticsearch connection.
pub struct ApiRequest<C> {
	msg: Message,
	future: ReqComp,
	_marker: PhantomData<C>
}

impl <C> ApiRequest<C> {
	pub fn for_msg(msg: Message, future: ReqComp) -> Self {
		ApiRequest {
			msg: msg,
			future: future,
			_marker: PhantomData
		}
	}
}

impl <C> Requester for ApiRequest<C> {
	type Context = C;

	fn prepare_request(self, req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		req.start(&self.msg.verb, &self.msg.url, Version::Http11);
		
		req.add_header("Content-Type", b"application/json").unwrap();

		if let Some(ref body) = self.msg.body {
			req.add_length(body.len() as u64).unwrap();
			req.done_headers().unwrap();
			req.write_body(body);
		}
		else {
			req.done_headers().unwrap();
		}
		
		req.done();

		Some(self)
	}

	fn headers_received(self, _head: Head, _req: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
		//NOTE: 404's will come through here too, so we can set a correct error response
		Some((self, RecvMode::Buffered(1 << 20), scope.now() + Duration::new(DEFAULT_TIMEOUT, 0)))
	}

	fn response_received(self, data: &[u8], _req: &mut Request, _scope: &mut Scope<Self::Context>) {
		self.future.complete(Ok(data.to_vec()));
	}

	fn bad_response(self, _err: &ResponseError, _scope: &mut Scope<Self::Context>) {
		self.future.complete(Err("nah it's broke m8. should use a proper error type here."));
	}

	fn response_chunk(self, _chunk: &[u8], _req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		unreachable!();
	}

	fn response_end(self, _req: &mut Request, _scope: &mut Scope<Self::Context>) {
		unreachable!();
	}

	fn timeout(self, _req: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
		//TODO: Check for cancellation
		Some((self, scope.now() + Duration::new(DEFAULT_TIMEOUT, 0)))
	}

	fn wakeup(self, _req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		//TODO: Check for cancellation
		Some(self)
	}
}