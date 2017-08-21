/*!
Elasticsearch REST API Client

A lightweight implementation of the Elasticsearch API based on the [`reqwest`][reqwest] HTTP client.

Each API endpoint is represented as its own type, that only accept a valid combination of route parameters.
This library makes very few assumptions, leaving it up to you to decide what to invest your precious CPU cycles into.

The API is generated from the official Elasticsearch spec, so it's always current.

# Supported Versions

 `elastic_types` | Elasticsearch
 --------------- | -------------
 `0.x`           | `5.x`

# Usage

This crate is on [crates.io][crates].
To get started, add `elastic_reqwest` and `reqwest` to your `Cargo.toml`:

```ignore
[dependencies]
elastic_reqwest = "*"
reqwest = "*"
```

Then reference in your crate root:

```
extern crate elastic_reqwest;
# fn main() {}
```

# Making synchronous requests

- Create a [`SyncClient`][default_sync]
- Call [`elastic_req`][elastic_req_sync] on the client
- Work with the raw http response
- Or call [`parse`][parse] to get a concrete response or API error

## Minimal Example

Execute a search request synchronously and parse the response:

```no_run
//HTTP POST /myindex/mytype/_search

#[macro_use]
extern crate serde_json;
extern crate elastic_reqwest;

use serde_json::Value;
use elastic_reqwest::{SyncElasticClient, SyncFromResponse, parse};
use elastic_reqwest::req::SearchRequest;
use elastic_reqwest::res::SearchResponse;

# fn main() {
let (client, params) = elastic_reqwest::sync::default().unwrap();

let search = SearchRequest::for_index_ty(
    "myindex", "mytype",
    json!({
        "query": {
            "filtered": {
                "query": {
                    "match_all": {}
                },
                "filter": {
                    "geo_distance": {
                        "distance": "20km",
                        "location": {
                            "lat": 37.776,
                            "lon": -122.41
                        }
                    }
                }
            }
        }
    })
);

let http_res = client.elastic_req(&params, search).unwrap();
let search_res = parse::<SearchResponse<Value>>().from_response(http_res).unwrap();
# }
```

# Making asynchronous requests

- Create an [`AsyncClient`][default_async]
- Call [`elastic_req`][elastic_req_async] on the client
- Work with the raw http response
- Or call [`parse`][parse] to get a concrete response or API error

## Minimal Example

Execute a search request asynchronously and parse the response:

```no_run
//HTTP POST /myindex/mytype/_search

#[macro_use]
extern crate serde_json;
extern crate elastic_reqwest;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;

use tokio_core::reactor::Core;
use futures::Future;
use serde_json::Value;
use elastic_reqwest::{AsyncElasticClient, AsyncFromResponse, parse};
use elastic_reqwest::req::SearchRequest;
use elastic_reqwest::res::SearchResponse;

# fn main() {
let mut core = Core::new().unwrap();

let (client, params) = elastic_reqwest::async::default(&core.handle()).unwrap();

let search = SearchRequest::for_index_ty(
    "myindex", "mytype",
    json!({
        "query": {
            "filtered": {
                "query": {
                    "match_all": {}
                },
                "filter": {
                    "geo_distance": {
                        "distance": "20km",
                        "location": {
                            "lat": 37.776,
                            "lon": -122.41
                        }
                    }
                }
            }
        }
    })
);

let search_future = client
    .elastic_req(&params, search)
    .and_then(|http_res| parse::<SearchResponse<Value>>().from_response(http_res))
    .and_then(|search_res| {
        println!("{:?}", search_res);
        Ok(())  
    });

core.run(search_future).unwrap();
# }
```

## Search Request with Url Param

Execute a search request with a url parameter:

```no_run
//HTTP GET /myindex/mytype/_search?q=*

extern crate serde_json;
extern crate elastic_reqwest;

use serde_json::Value;
use elastic_reqwest::{ SyncElasticClient, SyncFromResponse, RequestParams, parse };
use elastic_reqwest::req::SimpleSearchRequest;
use elastic_reqwest::res::SearchResponse;

# fn main() {
let (client, _) = elastic_reqwest::sync::default().unwrap();

let params = RequestParams::default()
    .url_param("pretty", true)
    .url_param("q", "*");

let search = SimpleSearchRequest::for_index_ty(
    "myindex", "mytype"
);

let http_res = client.elastic_req(&params, search).unwrap();
let search_res = parse::<SearchResponse<Value>>().from_response(http_res).unwrap();
# }
```

# See Also

- [`elastic`][elastic].
A higher-level Elasticsearch client that uses `elastic_reqwest` as its HTTP layer.
- [`rs-es`][rs-es].
A higher-level Elasticsearch client that provides strongly-typed Query DSL buiilders.
- [`json_str`][json_str]
A library for generating minified json strings from Rust syntax.

# Links
- [Elasticsearch Docs][es-docs]
- [Github][repo]

[default_sync]: sync/fn.default.html
[default_async]: async/fn.default.html
[elastic_req_sync]: sync/trait.SyncElasticClient.html#tymethod.elastic_req
[elastic_req_async]: async/trait.AsyncElasticClient.html#tymethod.elastic_req
[parse]: fn.parse.html
[elastic]: https://github.com/elastic-rs/elastic
[rs-es]: https://github.com/benashford/rs-es
[json_str]: https://github.com/KodrAus/json_str
[reqwest]: https://github.com/seanmonstar/reqwest/
[es-docs]: https://www.elastic.co/guide/en/elasticsearch/reference/master/index.html
[repo]: https://github.com/elastic-rs/elastic-reqwest
[crates]: https://crates.io/crates/elastic_reqwest
*/

#![deny(warnings)]
#![deny(missing_docs)]

#[macro_use]
extern crate quick_error;

extern crate elastic_requests;
extern crate elastic_responses;
extern crate serde;
#[cfg_attr(test, macro_use)] 
extern crate serde_json;
extern crate reqwest;
extern crate url;
extern crate bytes;
extern crate tokio_core;
extern crate tokio_io;
extern crate futures;

pub mod sync;
pub mod async;

pub use self::sync::{SyncElasticClient, SyncBody, SyncFromResponse};
pub use self::async::{AsyncElasticClient, AsyncBody, AsyncFromResponse};

/**
Request types.

These are re-exported from `elastic_requests` for convenience.
*/
pub mod req {
    pub use elastic_requests::*;
}

/**
Response types.

These are re-exported from `elastic_responses` for convenience.
*/
pub mod res {
    pub use elastic_responses::*;
}

pub use self::res::parse;

use std::sync::Arc;
use std::collections::BTreeMap;
use std::str;
use reqwest::Error as ReqwestError;
use reqwest::header::{Header, Headers, ContentType};
use url::form_urlencoded::Serializer;

use self::res::error::ResponseError;
use self::req::HttpMethod;

quick_error! {
    /** An error sending a request or parsing a response. */
    #[derive(Debug)]
    pub enum Error {
        /** A http error. */
        Http(err: ReqwestError) {
            from()
            description("http error")
            display("http error: {}", err)
            cause(err)   
        }
        /** A response error. */
        Response(err: ResponseError) {
            from()
            description("response error")
            display("response error: {}", err)
            cause(err)   
        }
    }
}

/**
Misc parameters for any request.

The `RequestParams` struct allows you to set headers and url parameters for your requests.
By default, the `ContentType::json` header will always be added.
Url parameters are added as simple key-value pairs, and serialised by [rust-url](http://servo.github.io/rust-url/url/index.html).

# Examples

With default query parameters:

```
# use elastic_reqwest::RequestParams;
let params = RequestParams::default();
```

With a custom base url:

```
# use elastic_reqwest::RequestParams;
let params = RequestParams::new("http://mybaseurl:9200");
```

With custom headers:

```
# extern crate reqwest;
# extern crate elastic_reqwest;
# use elastic_reqwest::RequestParams;
# use reqwest::header::Authorization;
# fn main() {
let params = RequestParams::default()
    .header(Authorization("let me in".to_owned()));
# }
```

With url query parameters:

```
# extern crate elastic_reqwest;
# use elastic_reqwest::RequestParams;
# fn main() {
let params = RequestParams::default()
    .url_param("pretty", true)
    .url_param("q", "*");
# }
```
*/
#[derive(Clone)]
pub struct RequestParams {
    /** Base url for Elasticsearch. */
    base_url: String,
    /** Simple key-value store for url query params. */
    url_params: BTreeMap<&'static str, String>,
    /** The complete set of headers that will be sent with the request. */
    headers_factory: Option<Arc<Fn(&mut Headers) + Send + Sync + 'static>>,
}

impl RequestParams {
    /** 
    Create a new container for request parameters.
    
    This method takes a fully-qualified url for the Elasticsearch node.
    It will also set the `Content-Type` header to `application/json`.
    */
    pub fn new<T: Into<String>>(base: T) -> Self {
        RequestParams {
            base_url: base.into(),
            headers_factory: None,
            url_params: BTreeMap::new(),
        }
    }

    /** Set the base url for the Elasticsearch node. */
    pub fn base_url<T: Into<String>>(mut self, base: T) -> Self {
        self.base_url = base.into();

        self
    }

    /** 
    Set a url param value.
    
    These parameters are added as query parameters to request urls.
    */
    pub fn url_param<T: ToString>(mut self, key: &'static str, value: T) -> Self {
        self.url_params.insert(key, value.to_string());
        self
    }

    /** 
    Set a request header.
    */
    pub fn header<H>(self, header: H) -> Self
        where H: Header + Clone
    {
        self.headers(move |h| h.set(header.clone()))
    }

    /** 
    Set a header value on the params.
    
    Each call to `headers` will chain to the end of the last call.
    This function allocates a new `Box` for each call.

    Once we can depend on `http` this might go away.
    */
    fn headers<F>(mut self, headers_factory: F) -> Self
        where F: Fn(&mut Headers) + Send + Sync + 'static
    {
        if let Some(old_headers_factory) = self.headers_factory {
            let headers_factory = move |mut headers: &mut Headers| {
                old_headers_factory(&mut headers);
                headers_factory(&mut headers);
            };

            self.headers_factory = Some(Arc::new(headers_factory));
        } else {
            self.headers_factory = Some(Arc::new(headers_factory));
        }

        self
    }

    /** Create a new `Headers` structure, and thread it through the configuration functions. */
    pub fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        if let Some(ref headers_factory) = self.headers_factory {
            headers_factory(&mut headers);
        }

        headers
    }

    /** 
    Get the url query params as a formatted string.
    
    Follows the `application/x-www-form-urlencoded` format.
    This method returns the length of the query string and an optional value.
    If the value is `None`, then the length will be `0`.
    */
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

fn build_url<'a>(req_url: &str, params: &RequestParams) -> String {
    let (qry_len, qry) = params.get_url_qry();

    let mut url = String::with_capacity(params.base_url.len() + req_url.len() + qry_len);

    url.push_str(&params.base_url);
    url.push_str(&req_url);

    if let Some(qry) = qry {
        url.push_str(&qry);
    }

    url
}

fn build_method(method: HttpMethod) -> reqwest::Method {
    match method {
        HttpMethod::Get => reqwest::Method::Get,
        HttpMethod::Post => reqwest::Method::Post,
        HttpMethod::Head => reqwest::Method::Head,
        HttpMethod::Delete => reqwest::Method::Delete,
        HttpMethod::Put => reqwest::Method::Put,
        HttpMethod::Patch => reqwest::Method::Patch,
    }
}

#[cfg(test)]
fn assert_send<T: Send>() {}

#[cfg(test)]
fn assert_sync<T: Sync>() {}

#[cfg(test)]
mod tests {
    use reqwest::header::{Referer, Authorization, ContentType};
    use super::*;

    #[test]
    fn assert_send_sync() {
        assert_send::<RequestParams>();
        assert_sync::<RequestParams>();
    }

    #[test]
    fn request_params_has_default_content_type() {
        let req = RequestParams::default();

        let headers = req.get_headers();

        assert_eq!(Some(&ContentType::json()), headers.get::<ContentType>());
    }

    #[test]
    fn set_multiple_headers() {
        let req = RequestParams::default()
            .headers(|h| h.set(Referer::new("/not-the-value")))
            .headers(|h| h.set(Referer::new("/People.html#tim")))
            .headers(|h| h.set(Authorization("let me in".to_owned())));

        let headers = req.get_headers();

        assert_eq!(Some(&ContentType::json()), headers.get::<ContentType>());
        assert_eq!(Some(&Referer::new("/People.html#tim")), headers.get::<Referer>());
        assert_eq!(Some(&Authorization("let me in".to_owned())), headers.get::<Authorization<String>>());
    }

    #[test]
    fn request_params_has_default_base_url() {
        let req = RequestParams::default();

        assert_eq!("http://localhost:9200", req.base_url);
    }

    #[test]
    fn request_params_can_set_base_url() {
        let req = RequestParams::default().base_url("http://eshost:9200");

        assert_eq!("http://eshost:9200", req.base_url);
    }

    #[test]
    fn request_params_can_set_url_query() {
        let req = RequestParams::default()
            .url_param("pretty", false)
            .url_param("pretty", true)
            .url_param("q", "*");

        assert_eq!((16, Some(String::from("?pretty=true&q=*"))),
                   req.get_url_qry());
    }

    #[test]
    fn empty_request_params_returns_empty_string() {
        let req = RequestParams::default();

        assert_eq!((0, None), req.get_url_qry());
    }
}
