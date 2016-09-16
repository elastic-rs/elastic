use crossbeam::sync::MsQueue;
use futures::{ Oneshot, Complete };

mod client;
mod fsm;
mod req;

pub use self::client::*;

/// A request message.
/// 
/// This is what you supply to kick off a request.
pub struct Request {
	url: String,
	verb: &'static str,
	body: Option<Vec<u8>>
}
impl Request {
	/// Create a new GET request.
	pub fn get<I: Into<String>>(url: I) -> Self {
		Request {
			url: url.into(),
			verb: "GET",
			body: None
		}
	}

	/// Create a new POST request.
	pub fn post<I: Into<String>>(url: I, body: &[u8]) -> Self {
		Request {
			url: url.into(),
			verb: "POST",
			body: Some(body.to_vec())
		}
	}
}

//TODO: Proper error type here
/// A common data format shared between producer and consumer.
pub type Response = Result<Vec<u8>, &'static str>;

/// The promise part of a request future.
pub type ResponseFuture = Oneshot<Response>;

/// The completion part of a request future.
type ResponseComplete = Complete<Response>;

/// A message representing a request and the completion part of a response.
type Message = (Request, ResponseComplete);

/// A queue to link a client with a connection pool.
/// 
/// This is essentially just a wrapped `MsQueue`.
/// Messages can't be put onto this queue directly, you need to use the
/// appropriate `Client` structure.
pub struct Queue(MsQueue<Message>);
impl Queue {
	pub fn new() -> Self {
		Queue(MsQueue::new())
	}

	fn push(&self, msg: Message) {
		self.0.push(msg);
	}

	fn try_pop(&self) -> Option<Message> {
		self.0.try_pop()
	}
}

