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
use hyper::header::Headers;
use hyper::header::ContentType;
use url::form_urlencoded::serialize;

/// Misc parameters for any request.
/// 
/// The `RequestParams` struct allows you to set headers and url parameters for your requests.
/// By default, the `ContentType::json` header will always be added.
/// Url parameters are added as simple key-value pairs, and serialised by [rust-url](http://servo.github.io/rust-url/url/index.html).
/// 
/// # Examples
/// 
/// With default query parameters:
/// 
/// ```
/// extern crate hyper;
/// extern crate elastic_hyper as elastic;
/// 
/// let params = elastic::RequestParams::default();
/// ```
/// 
/// With custom headers:
/// 
/// ```
/// extern crate hyper;
/// extern crate elastic_hyper as elastic;
/// 
/// let mut params = elastic::RequestParams::default();
/// 
/// //Add your own headers
/// params.headers.set(hyper::header::Authorization("let me in".to_owned()));
/// ```
/// 
/// Add url query parameters to the request:
/// 
/// ```
/// extern crate hyper;
/// extern crate elastic_hyper as elastic;
/// 
/// let params = elastic::RequestParams::default()
/// 		.url_params(vec![
/// 			("pretty", "true".to_owned()),
/// 			("q", "*".to_owned())
/// 		]);
/// ```
/// 
/// With a custom base url:
/// 
/// ```
/// extern crate hyper;
/// extern crate elastic_hyper as elastic;
/// 
/// let params = elastic::RequestParams::new("http://mybaseurl:9200", hyper::header::Headers::new());
/// ```
pub struct RequestParams {
	/// Base url for Elasticsearch
	pub base_url: &'static str,
	/// Simple key-value store for url query params.
	pub url_params: BTreeMap<&'static str, String>,
	/// The complete set of headers that will be sent with the request.
	pub headers: Headers
}

impl RequestParams {
	/// Create a new container for request parameters.
	/// 
	/// Attempts to add `ContentType::json` to the passed in `headers` param.
	pub fn new(base: &'static str, mut headers: Headers) -> Self {
		headers.set(ContentType::json());

		RequestParams {
			base_url: base,
			headers: headers,
			url_params: BTreeMap::new()
		}
	}

	/// Add a collection of url params.
	pub fn url_params<I>(mut self, url_params: I) -> Self
	where I: IntoIterator<Item=(&'static str, String)> {
		for (k, v) in url_params {
			self.url_params.insert(k, v);
		}

		self
	}

	/// Get the url params as a formatted string.
	/// 
	/// Follows the `application/x-www-form-urlencoded` format.
	pub fn get_url_qry(&self) -> String {
		if self.url_params.len() > 0 {
			let qry = serialize(self.url_params.iter());
			let mut url_qry = String::with_capacity(qry.len() + 1);

			url_qry.push('?');
			url_qry.push_str(&qry);

			url_qry
		}
		else {
			String::with_capacity(0)
		}
	}
}

impl Default for RequestParams {
	fn default() -> Self {
		RequestParams::new("http://localhost:9200", Headers::new())
	}
}

mod api;
pub use api::*;
