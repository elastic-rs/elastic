use std::marker::PhantomData;
use std::time::Duration;
use std::io::{ Write, stdout };

use crossbeam::sync::MsQueue;
use rotor::{ Scope, Time };
use rotor_http::client::{ Request, Requester, ResponseError, Head, RecvMode, Version };

pub mod constant;
//pub mod sniffed;

/// A request message.
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

	/// Get the url for this request.
	pub fn get_url(&self) -> &str {
		&self.url
	}

	/// Get the verb for this request.
	pub fn get_verb(&self) -> &str {
		&self.verb
	}

	/// Get the message body for this request.
	pub fn get_body(&self) -> Option<&[u8]> {
		match self.body {
			Some(ref b) => Some(b),
			None => None
		}
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
		req.start(&self.msg.get_verb(), &self.msg.get_url(), Version::Http11);
		
		req.add_header("Content-Type", b"application/json").unwrap();

		if let Some(body) = self.msg.get_body() {
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
		Some((self, RecvMode::Buffered(1 << 20), scope.now() + Duration::new(1000, 0)))
	}

	fn response_received(self, data: &[u8], _req: &mut Request, _scope: &mut Scope<Self::Context>) {
		//TODO: Write the response to the request's channel
		stdout().write_all(data).unwrap();
		println!("");
	}

	fn bad_response(self, _err: &ResponseError, _scope: &mut Scope<Self::Context>) {
		unimplemented!();
	}

	fn response_chunk(self, _chunk: &[u8], _req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		unreachable!();
	}

	fn response_end(self, _req: &mut Request, _scope: &mut Scope<Self::Context>) {
		unreachable!();
	}

	fn timeout(self, _req: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
		Some((self, scope.now() + Duration::new(1000, 0)))
	}

	fn wakeup(self, _req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		Some(self)
	}
}