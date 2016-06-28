use std::io::{ stdout, stderr };
use std::io::Write;
use std::marker::PhantomData;
use std::time::Duration;
use std::process::exit;

use rotor::{ Scope, Time };
use rotor_http::client::*;

use super::{ ElasticContext, ElasticRequest };

/// A state machine for an individual HTTP request to an Elasticsearch API endpoint.
pub struct ElasticHttp<C: ElasticContext> {
	req: ElasticRequest,
	_marker: PhantomData<C>
}
impl <C: ElasticContext> ElasticHttp<C> {
	pub fn new(req: ElasticRequest) -> Self {
		ElasticHttp {
			req: req,
			_marker: PhantomData
		}
	}
}
impl <C: ElasticContext> Requester for ElasticHttp<C> {
    type Context = C;

    fn prepare_request(self, req: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
    	println!("Req.prepare_request");

        req.start("GET", &self.req.url.path(), Version::Http11);

        req.add_header("Host", self.req.url.host_str().unwrap().as_bytes()).unwrap();
        req.add_header("Content-Type", b"application/json").unwrap();
        req.add_header("Connection", b"keep-alive").unwrap();

        req.done_headers().unwrap();
        req.done();

        Some(self)
    }

    fn headers_received(self, head: Head, _request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<(Self, RecvMode, Time)> {
    	println!("Req.headers_received");

        println!("----- Headers -----");
        println!("Status: {} {}", head.code, head.reason);
        for header in head.headers {
            println!("{}: {}", header.name,
                String::from_utf8_lossy(header.value));
        }
        Some((self, RecvMode::Buffered(1 << 20),
            scope.now() + Duration::new(1000, 0)))
    }

    fn response_received(self, data: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) {
    	println!("Req.response_received");

        println!("----- Response -----");
        stdout().write_all(data).unwrap();
        if data.last() != Some(&b'\n') {
            println!("");
        }
    }

    fn bad_response(self, err: &ResponseError, _scope: &mut Scope<Self::Context>) {
    	println!("Req.bad_response");

        writeln!(&mut stderr(), "----- Bad response: {} -----", err).ok();
        exit(1);
    }

    fn response_chunk(self, _chunk: &[u8], _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<Self> {
        unreachable!();
    }
    fn response_end(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) {
        unreachable!();
    }
    fn timeout(self, _request: &mut Request, _scope: &mut Scope<Self::Context>) -> Option<(Self, Time)> {
        unreachable!();
    }
    fn wakeup(self, request: &mut Request, scope: &mut Scope<Self::Context>) -> Option<Self> {
        self.prepare_request(request, scope)
    }
}