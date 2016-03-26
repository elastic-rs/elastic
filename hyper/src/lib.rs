//! Elasticsearch Hyper Client
//!
//! A lightweight implementation of the Elasticsearch API based on Hyper.
//!
//! Each API endpoint is represented as its own function,
//! so each possible http route gets its own function.
//! The functions are also designed to work well with the `elastic_types`
//! and `elastic_macros` crates, but deserialisation is the responsibility of the caller.
//!
//! # Links
//! - [elastic_types](http://kodraus.github.io/rustdoc/elastic_types/index.html)
//! - [elastic_macros](http://kodraus.github.io/rustdoc/elastic_macros/index.html)
//! - [Github](https://github.com/KodrAus/elasticsearch-rs)

extern crate hyper;
extern crate url;

use std::collections::BTreeMap;
use hyper::header::Header;
use hyper::header::Headers;
use hyper::header::ContentType;
use url::form_urlencoded::serialize;

pub struct RequestParams {
	pub url_params: BTreeMap<&'static str, String>,
	pub headers: Headers
}

impl RequestParams {
	pub fn new(mut headers: Headers) -> Self {
		headers.set(ContentType::json());

		RequestParams {
			headers: headers,
			url_params: BTreeMap::new()
		}
	}

	pub fn url_params<I>(mut self, url_params: I) -> Self
	where I: IntoIterator<Item=(&'static str, String)> {
		for (k, v) in url_params {
			self.url_params.insert(k, v);
		}

		self
	}

	pub fn get_url_qry(&self) -> String {
		serialize(self.url_params.iter())
	}
}

mod api;
pub use api::*;
