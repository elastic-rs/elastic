//! Elasticsearch Request Types
//!
//! An implementation of the Elasticsearch REST API using strong types for endpoints.
//!
//! The source is automatically generated from the official spec.
//! A `struct` is provided for each endpoint that works with borrowed or owned data.
//! There's also a more general `Endpoint` type that all requests can be converted into.
//!
//! Request types are generic over the body buffer, `B`.
//! This gives you a lot of flexibility when designing APIs,
//! but you should be careful to ensure the `B` is bound appropriately.
//!
//! # Supported Versions
//!
//!  `elastic_requests` | Elasticsearch
//!  ------------------ | -------------
//!  `0.x`              | `5.x`
//!
//! # Usage
//!
//! All request types provide constructor functions of the form
//! `param_1_param_2_param_n`:
//!
//! ```
//! # use elastic_requests::*;
//! let req = SearchRequest::for_index_ty(
//!     "test_index",
//!     "test_ty",
//!     "{'query': { 'match_all': {}}}"
//! );
//!
//! assert_eq!("/test_index/test_ty/_search", req.url.as_ref());
//! ```
//!
//! Or `new` if the endpoint takes no parameters:
//!
//! ```
//! # use elastic_requests::*;
//! let req = PingRequest::new();
//!
//! assert_eq!("/", req.url.as_ref());
//! ```
//!
//! Parameters can be borrowed or owned string values:
//!
//! ```
//! # use elastic_requests::*;
//! let req = SearchRequest::for_index(
//!     "test_index".to_string(),
//!     "{'query': { 'match_all': {}}}"
//! );
//!
//! assert_eq!("/test_index/_search", req.url.as_ref());
//! ```
//!
//! All request types can be converted into a more general `Endpoint`.
//! In this example, `takes_req` accepts anything that can be converted into
//! a `Endpoint` where the body buffer is `AsRef<[u8]>`:
//!
//! ```
//! # use elastic_requests::*;
//! fn takes_req<'a, I: Into<Endpoint<'a, B>>, B: AsRef<[u8]>>(req: I) {
//!     let req = req.into();
//!     let body = req.body.as_ref();
//!
//!     // do something with the request
//! }
//!
//! takes_req(PingRequest::new());
//! takes_req(SearchRequest::for_index("test_index", empty_body()));
//! ```
//!
//! # Why are these docs useless?
//!
//! This library is automatically generated, so there's a lot more work to do
//! to get the documentation up to scratch.
//!
//! # Links
//!
//! - [`elastic_reqwest`](https://github.com/elastic-rs/elastic-reqwest)
//! - [Github](https://github.com/elastic-rs/elastic-requests)

#![deny(warnings)]

mod genned;

/// Common url params like `Id` and `Index`.
///
/// The parameter types are basically just a wrapper around a maybe
/// owned string.
/// They can all be constructed from a `String` or an `&str`, but some
/// parameters may have other implementations in the future.
pub mod params {
    pub use genned::params::*;
}

/// REST API endpoints.
///
/// Each type corresponds to a single HTTP method on a single endpoint.
/// Request types have constructor functions that take the form
/// `for_param_1_param_2_param_n`, and accept a `Body` parameter if the underlying
/// method is a `POST` or `PUT`.
/// Other request parameters accept any type that can be converted into the
/// parameter type, usually a `String` or `&str`.
///
/// Request types don't take ownership of their inputs unless you pass in owned
/// data.
/// That means if some function expects a `SearchRequest<'static>` then you can
/// either use a `SearchRequest` with owned `String` inputs, or one that uses only
/// `'static` inputs.
pub mod endpoints {
    pub use genned::endpoints::*;
}

pub use genned::http::*;
pub use self::params::*;
pub use self::endpoints::*;

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    fn do_something_with_request<'a, I: Into<Endpoint<'a, B>>, B: AsRef<[u8]>>(_: I) {}

    fn do_something_with_static_request<I: Into<Endpoint<'static, B>>, B: 'static + AsRef<[u8]> + Send>(req: I) -> thread::JoinHandle<()> {
        let req = req.into();
        thread::spawn(move || {
            assert_eq!("/test_index/test_ty/_search", *req.url);
        })
    }

    #[test]
    fn it_works() {
        let req = SearchRequest::for_index_ty("test_index", "test_ty", "{'query': { 'match_all': {}}}");

        assert_eq!("/test_index/test_ty/_search", *req.url);

        do_something_with_request(req);
    }

    #[test]
    fn it_works_no_body() {
        let req = PingRequest::new();

        do_something_with_request(req);
    }

    #[test]
    fn it_works_static() {
        let req = SearchRequest::for_index_ty(String::from("test_index"), "test_ty", empty_body());

        do_something_with_static_request(req).join().unwrap();
    }

    #[test]
    fn id_from_number() {
        let ids = vec![
            Id::from(1i32),
            Id::from(1u32),
            Id::from(1i64),
            Id::from(1u64),
            Id::from(1isize),
            Id::from(1usize),
        ];

        for id in ids {
            assert_eq!("1", &*id);
        }
    }
}
