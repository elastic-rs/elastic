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
//! elastic_reqwest = "*"
//! reqwest = "*"
//! ```
//!
//! Then reference in your crate root:
//!
//! ```
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
//! # extern crate elastic_reqwest as cli;
//! use cli::ElasticClient;
//! use cli::req::PingRequest;
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
//! extern crate elastic_reqwest as cli;
//! use cli::{ ElasticClient, RequestParams };
//! use cli::req::SimpleSearchRequest;
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
//! extern crate elastic_reqwest as cli;
//! use cli::ElasticClient;
//! use cli::req::SearchRequest;
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
//! - [`elastic`](https://github.com/elastic-rs/elastic). 
//! A higher-level Elasticsearch client that uses `elastic_reqwest` as its HTTP layer.
//! - [`rs-es`](https://github.com/benashford/rs-es).
//! An alternative Elasticsearch client for Rust that provides an implementation of the Query DSL.
//! - [`json_str`](https://github.com/KodrAus/json_str)
//! A library for generating minified json strings from Rust syntax.
//!
//! # Links
//! - [Elasticsearch Docs](https://www.elastic.co/guide/en/elasticsearch/reference/master/index.html)
//! - [Github](https://github.com/elastic-rs/elastic-reqwest)

#![deny(warnings)]
#![deny(missing_docs)]

extern crate elastic_requests;
extern crate reqwest;
extern crate url;

use std::collections::BTreeMap;
use std::str;
use reqwest::header::{Header, HeaderFormat, Headers, ContentType};
use reqwest::{RequestBuilder, Response};
use url::form_urlencoded::Serializer;

/// Request types.
/// 
/// These are re-exported from `elastic_requests` for convenience.
pub mod req {
    pub use elastic_requests::*;
}

use self::req::{HttpRequest, HttpMethod};

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
/// # use elastic_reqwest::RequestParams;
/// let params = RequestParams::default();
/// ```
///
/// With a custom base url:
///
/// ```
/// # use elastic_reqwest::RequestParams;
/// let params = RequestParams::new("http://mybaseurl:9200");
/// ```
///
/// With custom headers:
///
/// ```
/// # extern crate reqwest;
/// # extern crate elastic_reqwest;
/// # use elastic_reqwest::RequestParams;
/// # use reqwest::header::Authorization;
/// # fn main() {
/// let params = RequestParams::default()
///     .header(Authorization("let me in".to_owned()));
/// # }
/// ```
///
/// With url query parameters:
///
/// ```
/// # extern crate elastic_reqwest;
/// # use elastic_reqwest::RequestParams;
/// # fn main() {
/// let params = RequestParams::default()
///     .url_param("pretty", true)
///     .url_param("q", "*");
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct RequestParams {
    /// Base url for Elasticsearch.
    pub base_url: String,
    /// Simple key-value store for url query params.
    pub url_params: BTreeMap<&'static str, String>,
    /// The complete set of headers that will be sent with the request.
    pub headers: Headers,
}

impl RequestParams {
    /// Create a new container for request parameters.
    /// 
    /// This method takes a fully-qualified url for the Elasticsearch
    /// node.
    /// It will also set the `Content-Type` header to `application/json`.
    pub fn new<T: Into<String>>(base: T) -> Self {
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        RequestParams {
            base_url: base.into(),
            headers: headers,
            url_params: BTreeMap::new(),
        }
    }

    /// Set the base url for the Elasticsearch node.
    pub fn base_url<T: Into<String>>(mut self, base: T) -> Self {
        self.base_url = base.into();

        self
    }

    /// Set a url param value.
    /// 
    /// These parameters are added as query parameters to request urls.
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

    /// Get the url query params as a formatted string.
    ///
    /// Follows the `application/x-www-form-urlencoded` format.
    /// This method returns the length of the query string and an optional
    /// value.
    /// If the value is `None`, then the length will be `0`.
    pub fn get_url_qry(&self) -> (usize, Option<String>) {
        if self.url_params.len() > 0 {
            let qry: String = Serializer::for_suffix(String::from("?"), 1)
                .extend_pairs(self.url_params.iter())
                .finish();

            (qry.len(), Some(qry))
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

/// Represents a client that can send Elasticsearch requests.
pub trait ElasticClient {
    /// Send a request and get a response.
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Result<Response, reqwest::Error> 
        where I: Into<HttpRequest<'static, B>>,
              B: Into<reqwest::Body>;
}

macro_rules! req_with_body {
    ($client:ident, $url:ident, $body:ident, $params:ident, $method:ident) => ({
        let body = $body.expect("Expected this request to have a body. This is a bug, please file an issue on GitHub.");

        $client.request(reqwest::Method::$method, &$url)
               .headers($params.headers.to_owned())
               .body(body.into())
    })
}

fn build_req<I, B>(client: &reqwest::Client, params: &RequestParams, req: I) -> RequestBuilder 
    where I: Into<HttpRequest<'static, B>>,
          B: Into<reqwest::Body>
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
        HttpMethod::Get => client.get(&url).headers(params.headers.to_owned()),

        HttpMethod::Post => req_with_body!(client, url, body, params, Post),

        HttpMethod::Head => client.head(&url).headers(params.headers.to_owned()),

        HttpMethod::Delete => client.request(reqwest::Method::Delete, &url).headers(params.headers.to_owned()),
        
        HttpMethod::Put => req_with_body!(client, url, body, params, Put),
        
        HttpMethod::Patch => req_with_body!(client, url, body, params, Patch),
    }
}

impl ElasticClient for reqwest::Client {
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Result<Response, reqwest::Error>
        where I: Into<HttpRequest<'static, B>>,
              B: Into<reqwest::Body>
    {
        build_req(&self, params, req).send()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, RequestBuilder, Method};
    use super::*;
    use super::req::*;

    fn params() -> RequestParams {
        RequestParams::new("eshost:9200/path")
            .url_param("pretty", true)
            .url_param("q", "*")
    }

    fn expected_req(cli: &Client, method: Method, url: &str, body: Option<Vec<u8>>) -> RequestBuilder {
        let req = cli.request(method, url)
                         .header(ContentType::json());

        match body {
            Some(body) => req.body(body),
            None => req
        }
    }

    fn assert_req(expected: RequestBuilder, actual: RequestBuilder) {
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    #[test]
    fn head_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), PingRequest::new());

        let url = "eshost:9200/path/?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Head, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn get_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), SimpleSearchRequest::new());

        let url = "eshost:9200/path/_search?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Get, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn post_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), PercolateRequest::for_index_ty("idx", "ty", vec![]));

        let url = "eshost:9200/path/idx/ty/_percolate?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Post, url, Some(vec![]));

        assert_req(expected, req);
    }

    #[test]
    fn put_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), IndicesCreateRequest::for_index("idx", vec![]));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Put, url, Some(vec![]));

        assert_req(expected, req);
    }

    #[test]
    fn delete_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), IndicesDeleteRequest::for_index("idx"));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Delete, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn request_params_has_default_content_type() {
        let req = RequestParams::default();
        assert_eq!(Some(&ContentType::json()), req.headers.get::<ContentType>());
    }

    #[test]
    fn request_params_has_default_base_url() {
        let req = RequestParams::default();

        assert_eq!("http://localhost:9200", req.base_url);
    }

    #[test]
    fn request_params_can_set_base_url() {
        let req = RequestParams::default()
            .base_url("http://eshost:9200");

        assert_eq!("http://eshost:9200", req.base_url);
    }

    #[test]
    fn request_params_can_set_url_query() {
        let req = RequestParams::default()
            .url_param("pretty", false)
            .url_param("pretty", true)
            .url_param("q", "*");

        assert_eq!((16, Some(String::from("?pretty=true&q=*"))), req.get_url_qry());
    }

    #[test]
    fn empty_request_params_returns_empty_string() {
        let req = RequestParams::default();

        assert_eq!((0, None), req.get_url_qry());
    }
}
