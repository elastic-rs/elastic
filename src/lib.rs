//! Elasticsearch REST API Client
//!
//! A lightweight implementation of the Elasticsearch API based on the
//! [`hyper`](http://hyper.rs/hyper/) HTTP client.
//!
//! Each API endpoint is represented as its own function, so each possible http route gets its own function.
//! This library makes very few assumptions, leaving it up to you to decide what to invest your
//! precious CPU cycles into.
//!
//! The entire API is generated from the official Elasticsearch spec, so it's always current.
//! 
//! # Supported Versions
//! 
//!  `elastic_types` | Elasticsearch
//!  --------------- | -------------
//!  `0.x`           | `5.x`
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/elastic_hyper).
//! To get started, add `elastic_hyper` and `hyper` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! elastic_hyper = "*"
//! hyper = "*"
//! ```
//!
//! For `Windows`, you may need to exclude `openssl` or the build can fail:
//!
//! ```ignore
//! [dependencies]
//! elastic_hyper = { version = "*", default-features = false }
//! hyper = { version = "*", default-features = false }
//! ```
//!
//! Then reference in your crate root:
//!
//! ```ignore
//! extern crate elastic_hyper as elastic;
//! extern crate hyper;
//! ```
//!
//! ## Minimal Example
//!
//! Ping the availability of your cluster:
//!
//! ```no_run
//! //HTTP HEAD /
//!
//! # extern crate hyper;
//! # extern crate elastic_hyper as elastic;
//! # fn main() {
//! let (mut client, params) = elastic::default();
//!
//! elastic::ping::head(&mut client, &params).unwrap();
//! # }
//! ```
//!
//! ## Search Request with Url Param
//!
//! Execute a search query with a url parameter:
//!
//! ```no_run
//! //HTTP GET /myindex/mytype/_search?q='my string'
//!
//! # extern crate hyper;
//! # extern crate elastic_hyper as elastic;
//! # fn main() {
//! let mut client = hyper::Client::new();
//! let mut params = elastic::RequestParams::default()
//! 	.url_params(vec![
//! 		("q", "'my string'".to_owned()),
//! 		("pretty", "true".to_owned())
//! 	]);
//!
//! elastic::search::get_index_type(&mut client, &params, "myindex", "mytype").unwrap();
//! # }
//! ```
//!
//! ## Search Request with Json
//!
//! Using the [`json_str`](http://kodraus.github.io/rustdoc/json_str/) crate, you can execute
//! queries using pure json:
//!
//! ```no_run
//! //HTTP POST /myindex/mytype/_search
//!
//! # #[macro_use]
//! # extern crate json_str;
//! # extern crate hyper;
//! # extern crate elastic_hyper as elastic;
//! # fn main() {
//! let (mut client, params) = elastic::default();
//!
//! elastic::search::post_index_type(&mut client, &params,
//! 	"myindex", "mytype", &json_str!({
//! 		"query": {
//! 			"filtered": {
//! 				"query": {
//! 					"match_all": {}
//! 				},
//! 				"filter": {
//! 					"geo_distance": {
//! 						"distance": "20km",
//! 						"location": {
//! 							"lat": 37.776,
//! 							"lon": -122.41
//! 						}
//! 					}
//! 				}
//! 			}
//! 		}
//! 	})
//! ).unwrap();
//! # }
//! ```
//!
//! See more [examples](https://github.com/KodrAus/elasticsearch-rs/tree/master/hyper/samples).
//!
//! # See Also
//!
//! ## [`rs-es`](http://benashford.github.io/rs-es/rs_es/index.html)
//!
//! An alternative Elasticsearch client for Rust that provides an implementation of the Query DSL.
//!
//! ## [`elastic_types`](http://kodraus.github.io/rustdoc/elastic_types/)
//!
//! A library that implements the core datatypes in Elasticsearch documents and automatically generates
//! a json mapping from your Rust structures.
//!
//! ## [`json_str`](http://kodraus.github.io/rustdoc/json_str/)
//!
//! A library for generating minified json strings from Rust syntax.
//!
//! # Links
//! - [Elasticsearch Docs](https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html)
//! - [Github](https://github.com/elastic-rs/elastic-hyper)

extern crate hyper;
extern crate url;

use std::collections::BTreeMap;
use hyper::header::Headers;
use hyper::header::ContentType;
use url::form_urlencoded::Serializer;

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
#[derive(Debug, Clone)]
pub struct RequestParams {
	/// Base url for Elasticsearch
	pub base_url: String,
	/// Simple key-value store for url query params.
	pub url_params: BTreeMap<&'static str, String>,
	/// The complete set of headers that will be sent with the request.
	pub headers: Headers
}

impl RequestParams {
	/// Create a new container for request parameters.
	///
	/// Attempts to add `ContentType::json` to the passed in `headers` param.
	pub fn new<T: Into<String>>(base: T, mut headers: Headers) -> Self {
		headers.set(ContentType::json());

		RequestParams {
			base_url: base.into(),
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
			let qry: String = Serializer::new(String::new())
				.extend_pairs(self.url_params.iter())
				.finish();
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

/// Get a default `Client` and `RequestParams`.
pub fn default() -> (hyper::Client, RequestParams) {
	(hyper::Client::new(), RequestParams::default())
}
