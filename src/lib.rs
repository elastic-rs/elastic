//! Elasticsearch REST API Client
//!
//! A lightweight implementation of the Elasticsearch API based on the
//! [`reqwest`](https://github.com/seanmonstar/reqwest/) HTTP client.
//!
//! Each API endpoint is represented as its own function,
//! so each possible http route gets its own function.
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
//! This crate is on [crates.io](https://crates.io/crates/reqwest).
//! To get started, add `elastic_reqwest` and `reqwest` to your `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! elastic_requests = "*"
//! elastic_reqwest = "*"
//! reqwest = "*"
//! ```
//!
//! For `Windows`, you may need to exclude `openssl` or the build can fail:
//!
//! ```ignore
//! [dependencies]
//! elastic_requests = "*"
//! elastic_reqwest = { version = "*", default-features = false }
//! ```
//!
//! Then reference in your crate root:
//!
//! ```
//! extern crate elastic_requests as req;
//! extern crate elastic_reqwest as cli;
//! ```
//!
//! ## Minimal Example
//!
//! Ping the availability of your cluster:
//!
//! ```no_run
//! //HTTP HEAD /
//!
//! # extern crate elastic_requests as req;
//! # extern crate elastic_reqwest as cli;
//! use cli::ElasticClient;
//! use req::PingRequest;
//!
//! # fn main() {
//! let (client, params) = cli::default().unwrap();
//!
//! client.elastic_req(&params, PingRequest::new()).unwrap();
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
//! extern crate reqwest;
//! extern crate elastic_requests as req;
//! extern crate elastic_reqwest as cli;
//! use cli::{ ElasticClient, RequestParams };
//! use req::SimpleSearchRequest;
//!
//! # fn main() {
//! let (client, _) = cli::default().unwrap();
//!
//! let params = RequestParams::default()
//!     .url_param("pretty", true)
//!     .url_param("q", "*");
//!
//! let search = SimpleSearchRequest::for_index_ty(
//!     "myindex", "mytype"
//! );
//!
//! client.elastic_req(&params, search).unwrap();
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
//! #
//! #[macro_use]
//! extern crate json_str;
//! extern crate elastic_requests as req;
//! extern crate elastic_reqwest as cli;
//! use cli::ElasticClient;
//! use req::SearchRequest;
//!
//! # fn main() {
//! let (client, params) = cli::default().unwrap();
//!
//! let search = SearchRequest::for_index_ty(
//!     "myindex", "mytype",
//!     json_str!({
//!         query: {
//!             filtered: {
//!                 query: {
//!                     match_all: {}
//!                 },
//!                 filter: {
//!                     geo_distance: {
//!                         distance: "20km",
//!                         location: {
//!                             lat: 37.776,
//!                             lon: -122.41
//!                         }
//!                     }
//!                 }
//!             }
//!         }
//!     })
//! );
//!
//! client.elastic_req(&params, search).unwrap();
//! # }
//! ```
//!
//! See more [examples](https://github.com/KodrAus/elasticsearch-rs/tree/master/hyper/samples).
//!
//! # See Also
//!
//! ## [`rs-es`](https://github.com/benashford/rs-es)
//!
//! An alternative Elasticsearch client for Rust that provides an implementation of the Query DSL.
//!
//! ## [`elastic_types`](https://github.com/elastic-rs/elastic-types)
//!
//! A library that implements the core datatypes in Elasticsearch
//! documents and automatically generates a json mapping from your Rust structures.
//!
//! ## [`json_str`](https://github.com/KodrAus/json_str)
//!
//! A library for generating minified json strings from Rust syntax.
//!
//! # Links
//! - [Elasticsearch Docs](https://www.elastic.co/guide/en/elasticsearch/reference/master/index.html)
//! - [Github](https://github.com/elastic-rs/elastic-hyper)

extern crate elastic_requests;
extern crate reqwest;
extern crate url;

use elastic_requests::*;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Cursor;
use std::str;
use reqwest::header::{Header, HeaderFormat, Headers, ContentType};
use reqwest::Response;
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
/// extern crate elastic_reqwest as elastic;
///
/// let params = elastic::RequestParams::default();
/// ```
///
/// With a custom base url:
///
/// ```
/// extern crate reqwest;
/// extern crate elastic_reqwest as elastic;
///
/// let params = elastic::RequestParams::new("http://mybaseurl:9200");
/// ```
///
/// With custom headers:
///
/// ```
/// extern crate reqwest;
/// extern crate elastic_reqwest as elastic;
///
/// use reqwest::header::Authorization;
///
/// # fn main() {
/// let params = elastic::RequestParams::default()
///     .header(Authorization("let me in".to_owned()));
/// # }
/// ```
///
/// With url query parameters:
///
/// ```
/// extern crate elastic_reqwest as elastic;
///
/// # fn main() {
/// let params = elastic::RequestParams::default()
///     .url_param("pretty", true)
///     .url_param("q", "*");
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RequestParams {
    /// Base url for Elasticsearch
    pub base_url: String,
    /// Simple key-value store for url query params.
    pub url_params: BTreeMap<&'static str, String>,
    /// The complete set of headers that will be sent with the request.
    pub headers: Headers,
}

impl RequestParams {
    /// Create a new container for request parameters.
    pub fn new<T: Into<String>>(base: T) -> Self {
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        RequestParams {
            base_url: base.into(),
            headers: headers,
            url_params: BTreeMap::new(),
        }
    }

    /// Set a url param value.
    pub fn url_param<T: ToString>(mut self, key: &'static str, value: T) -> Self
    {
        if self.url_params.contains_key(key) {
            let mut entry = self.url_params.get_mut(key).unwrap();
            *entry = value.to_string();
        }
        else {
            self.url_params.insert(key, value.to_string());
        }

        self
    }

    /// Set a header value on the params.
    pub fn header<H>(mut self, header: H) -> Self
        where H: Header + HeaderFormat
    {
        self.headers.set(header);

        self
    }

    /// Get the url params as a formatted string.
    ///
    /// Follows the `application/x-www-form-urlencoded` format.
    pub fn get_url_qry(&self) -> (usize, Option<String>) {
        if self.url_params.len() > 0 {
            let qry: String = Serializer::new(String::new())
                .extend_pairs(self.url_params.iter())
                .finish();
            let mut url_qry = String::with_capacity(qry.len() + 1);

            url_qry.push('?');
            url_qry.push_str(&qry);

            (url_qry.len(), Some(url_qry))
        } else {
            (0, None)
        }
    }
}

impl Default for RequestParams {
    fn default() -> Self {
        RequestParams::new("http://localhost:9200")
    }
}

/// Get a default `Client` and `RequestParams`.
pub fn default() -> Result<(reqwest::Client, RequestParams), reqwest::Error> {
    reqwest::Client::new().map(|cli| (cli, RequestParams::default()))
}

macro_rules! req_with_body {
    ($client:ident, $url:ident, $body:ident, $params:ident, $method:ident) => ({
        let body = $body.expect("Expected this request to have a body. This is a bug, please file an issue on GitHub.");

        let body = match body.into_raw() {
            Cow::Borrowed(b) => reqwest::Body::new(Cursor::new(b)),
            Cow::Owned(b) => b.into()
        };

        $client
            .request(reqwest::Method::$method, &$url)
            .headers($params.headers.to_owned())
            .body(body)
            .send()
    })
}

/// Represents a client that can send Elasticsearch requests.
pub trait ElasticClient {
    /// Send a request and get a response.
    fn elastic_req<I>(&self, params: &RequestParams, req: I) -> Result<Response, reqwest::Error> 
        where I: Into<HttpRequest<'static>>;
}

impl ElasticClient for reqwest::Client {
    fn elastic_req<I>(&self, params: &RequestParams, req: I) -> Result<Response, reqwest::Error>
        where I: Into<HttpRequest<'static>>
    {
        let req = req.into();

        let (qry_len, qry) = params.get_url_qry();

        let mut url = String::with_capacity(params.base_url.len() + req.url.len() + qry_len);

        url.push_str(&params.base_url);
        url.push_str(&req.url);

        if let Some(qry) = qry {
            url.push_str(&qry);
        }

        let method = req.method;
        let body = req.body;

        match method {
            HttpMethod::Get => self.get(&url).headers(params.headers.to_owned()).send(),

            HttpMethod::Post => req_with_body!(self, url, body, params, Post),

            HttpMethod::Head => self.head(&url).headers(params.headers.to_owned()).send(),

            HttpMethod::Delete => self.request(reqwest::Method::Delete, &url).headers(params.headers.to_owned()).send(),
            
            HttpMethod::Put => req_with_body!(self, url, body, params, Put),
            
            HttpMethod::Patch => req_with_body!(self, url, body, params, Patch),
        }
    }
}
