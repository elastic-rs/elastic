use std::marker::PhantomData;
use std::time::Duration;

use rotor::{ Scope, Time };
use rotor_http::client::{ Request as RotorRequest, Requester, ResponseError, Head, RecvMode, Version };

use super::{ Request, ResponseComplete };

const DEFAULT_TIMEOUT: u64 = 1000;

/// A state machine for managing the HTTP component of an Elasticsearch connection.
pub struct ApiRequest<C> {
	req: Request,
	fut: ResponseComplete,
	_c: PhantomData<C>
}

impl <C> ApiRequest<C> {
	pub fn for_req(req: Request, fut: ResponseComplete) -> Self {
		ApiRequest {
			req: req,
			fut: fut,
			_c: PhantomData
		}
	}
}

impl <C> Requester for ApiRequest<C> {
	type Context = C;

	fn prepare_request(self, r: &mut RotorRequest, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		r.start(&self.req.verb, &self.req.url, Version::Http11);
		
		r.add_header("Content-Type", b"application/json").unwrap();

		if let Some(ref body) = self.req.body {
			r.add_length(body.len() as u64).unwrap();
			r.done_headers().unwrap();
			r.write_body(body);
		}
		else {
			r.done_headers().unwrap();
		}
		
		r.done();

		Some(self)
	}

	fn headers_received(self, _head: Head, _req: &mut RotorRequest, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
		//NOTE: 404's will come through here too, so we can set a correct error response
		Some((self, RecvMode::Buffered(1 << 20), scope.now() + Duration::new(DEFAULT_TIMEOUT, 0)))
	}

	fn response_received(self, data: &[u8], _req: &mut RotorRequest, _scope: &mut Scope<Self::Context>) {
		self.fut.complete(Ok(data.to_vec()));
	}

	fn bad_response(self, _err: &ResponseError, _scope: &mut Scope<Self::Context>) {
		self.fut.complete(Err("nah it's broke m8. should use a proper error type here."));
	}

	fn response_chunk(self, _chunk: &[u8], _req: &mut RotorRequest, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		unreachable!();
	}

	fn response_end(self, _req: &mut RotorRequest, _scope: &mut Scope<Self::Context>) {
		unreachable!();
	}

	fn timeout(self, _req: &mut RotorRequest, scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
		//TODO: Check for cancellation
		Some((self, scope.now() + Duration::new(DEFAULT_TIMEOUT, 0)))
	}

	fn wakeup(self, _req: &mut RotorRequest, _scope: &mut Scope<Self::Context>) -> Option<Self> {
		//TODO: Check for cancellation
		Some(self)
	}
}