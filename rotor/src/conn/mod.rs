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
	body: Vec<u8>
}
impl Message {
	/// Create a new POST request.
	pub fn post<I: Into<String>>(url: I, body: &[u8]) -> Self {
		Message {
			url: url.into(),
			verb: "POST",
			body: body.to_vec()
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

	/// Get the length of the message body.
	pub fn get_body_ln(&self) -> u64 {
		self.body.len() as u64
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

		req.start(&self.msg.get_verb(), &self.msg.get_url(), Version::Http11);
		req.add_length(self.msg.get_body_ln()).unwrap();

        req.add_header("Content-Type", b"application/json").unwrap();
        req.done_headers().unwrap();

        req.write_body(&self.msg.get_body());
        req.done();

        Some(self)
	}

	fn headers_received(self, _head: Head, _req: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
		println!("requester: headers_received");

		Some((self, RecvMode::Buffered(1 << 20), scope.now() + Duration::new(1000, 0)))
	}

	fn response_received(self, data: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		println!("requester: response_received");

		//TODO: Write the response to the request's channel
		stdout().write_all(data).unwrap();
		println!("");
	}

	fn bad_response(self, _err: &ResponseError, _scope: &mut Scope<Self::Context>) {
		println!("requester: bad_response");

		unimplemented!();
	}

	fn response_chunk(self, _chunk: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		println!("requester: response_chunk");

		unreachable!();
	}
	
	fn response_end(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) {
		println!("requester: response_end");

		unreachable!();
	}

	fn timeout(self, _request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
		println!("requester: timeout");

		Some((self, scope.now() + Duration::new(1000, 0)))
	}

	fn wakeup(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		println!("requester: wakeup");

		Some(self)
	}
}