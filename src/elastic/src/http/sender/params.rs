use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
};

use reqwest::{
    self,
    header::{
        HeaderMap,
        HeaderName,
        HeaderValue,
        InvalidHeaderValue,
        AUTHORIZATION,
        CONTENT_TYPE,
    },
};
use url::form_urlencoded::Serializer;

use crate::http::{
    sender::NodeAddress,
    Method,
};

/**
The default Elasticsearch address to connect to.
*/
pub const DEFAULT_NODE_ADDRESS: &str = "http://localhost:9200";

/**
An incomplete set of request parameters.

The difference between `PreRequestParams` and `RequestParams` is the absense of a base url to send requests to.
When requests are load balanced between multiple Elasticsearch nodes the url to send a request to might not be known upfront.
*/
#[derive(Clone)]
pub struct PreRequestParams {
    url_params: Arc<HashMap<&'static str, String>>,
    // We should be able to replace this with `Arc<HeaderMapMap>` from the `http` crate
    headers: Arc<HeaderMap>,
}

/**
Parameters for a single REST API request.

The `RequestParams` struct allows you to set headers and url parameters for a given request.
By default, the `ContentType::json` header will always be added.
Url parameters are added as simple key-value pairs, and serialised by [rust-url](http://servo.github.io/rust-url/url/index.html).

# Examples

With default query parameters:

```
# use elastic::client::RequestParams;
let params = RequestParams::default();
```

With a custom base url:

```
# use elastic::client::RequestParams;
let params = RequestParams::new("http://mybaseurl:9200");
```

With custom headers:

```
# use elastic::prelude::*;
# use std::str::FromStr;
# fn main() -> Result<(), Box<dyn ::std::error::Error>> {
use elastic::http::header::{self, AUTHORIZATION, HeaderValue};

let auth = HeaderValue::from_str("let me in")?;

let params = RequestParams::default()
    .header(AUTHORIZATION, auth);
# Ok(())
# }
```

With url query parameters:

```
# use elastic::client::RequestParams;
let params = RequestParams::default()
    .url_param("pretty", true)
    .url_param("q", "*");
```
*/
#[derive(Clone)]
pub struct RequestParams {
    base_url: NodeAddress,
    inner: PreRequestParams,
}

impl PreRequestParams {
    /**
    Create a new container for request parameters.

    This method takes a fully-qualified url for the Elasticsearch node.
    It will also set the `Content-Type` header to `application/json`.
    */
    pub fn new() -> Self {
        PreRequestParams {
            headers: Arc::new({
                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

                headers
            }),
            url_params: Arc::new(HashMap::new()),
        }
    }

    /**
    Set a url param value.

    These parameters are added as query parameters to request urls.
    */
    pub fn url_param(mut self, key: &'static str, value: impl ToString) -> Self {
        Arc::make_mut(&mut self.url_params).insert(key, value.to_string());
        self
    }

    /** Set a request header. */
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        Arc::make_mut(&mut self.headers).insert(key, value);
        self
    }

    /** Enables HTTP basic authentication. */
    pub fn basic_auth<U, P>(
        mut self,
        username: U,
        password: Option<P>,
    ) -> Result<Self, InvalidHeaderValue>
    where
        U: fmt::Display,
        P: fmt::Display,
    {
        let auth = match password {
            Some(password) => format!("{}:{}", username, password),
            None => format!("{}:", username),
        };
        let header_value = HeaderValue::from_str(&format!("Basic {}", base64::encode(&auth)))?;
        Arc::make_mut(&mut self.headers).insert(AUTHORIZATION, header_value);
        Ok(self)
    }
}

impl Default for PreRequestParams {
    fn default() -> Self {
        PreRequestParams::new()
    }
}

impl RequestParams {
    /** Create a container for request parameters from a base url and pre request parameters. */
    pub fn from_parts(base_url: impl Into<NodeAddress>, inner: PreRequestParams) -> Self {
        RequestParams {
            base_url: base_url.into(),
            inner,
        }
    }

    /**
    Create a new container for request parameters.

    This method takes a fully-qualified url for the Elasticsearch node.
    It will also set the `Content-Type` header to `application/json`.
    */
    pub fn new(base_url: impl Into<NodeAddress>) -> Self {
        RequestParams::from_parts(base_url.into(), PreRequestParams::new())
    }

    /** Set the base url for the Elasticsearch node. */
    pub fn base_url(mut self, base_url: impl Into<NodeAddress>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /**
    Set a url param value.

    These parameters are added as query parameters to request urls.
    */
    pub fn url_param(mut self, key: &'static str, value: impl ToString) -> Self {
        self.inner = self.inner.url_param(key, value);
        self
    }

    /** Set a request header. */
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.inner = self.inner.header(key, value);
        self
    }

    /** Get the base url. */
    pub fn get_base_url(&self) -> &str {
        self.base_url.as_ref()
    }

    pub(crate) fn get_headers(&self) -> Arc<HeaderMap> {
        self.inner.headers.clone()
    }

    /**
    Get the url query params as a formatted string.

    Follows the `application/x-www-form-urlencoded` format.
    This method returns the length of the query string and an optional value.
    If the value is `None`, then the length will be `0`.
    */
    pub fn get_url_qry(&self) -> (usize, Option<String>) {
        if self.inner.url_params.len() > 0 {
            let qry: String = Serializer::for_suffix(String::from("?"), 1)
                .extend_pairs(self.inner.url_params.iter())
                .finish();

            (qry.len(), Some(qry))
        } else {
            (0, None)
        }
    }

    /** Split the request parameters into its parts. */
    pub fn split(self) -> (NodeAddress, PreRequestParams) {
        (self.base_url, self.inner)
    }
}

impl Default for RequestParams {
    fn default() -> Self {
        RequestParams::new(DEFAULT_NODE_ADDRESS)
    }
}

pub(crate) fn build_url(req_url: &str, params: &RequestParams) -> String {
    let (qry_len, qry) = params.get_url_qry();

    let mut url = String::with_capacity(params.base_url.as_ref().len() + req_url.len() + qry_len);

    url.push_str(params.base_url.as_ref());
    url.push_str(&req_url);

    if let Some(qry) = qry {
        url.push_str(&qry);
    }

    url
}

pub(crate) fn build_reqwest_method(method: Method) -> reqwest::Method {
    match method {
        Method::GET => reqwest::Method::GET,
        Method::POST => reqwest::Method::POST,
        Method::HEAD => reqwest::Method::HEAD,
        Method::DELETE => reqwest::Method::DELETE,
        Method::PUT => reqwest::Method::PUT,
        Method::PATCH => reqwest::Method::PATCH,
        method => {
            reqwest::Method::from_bytes(method.as_str().as_bytes()).expect("invalid HTTP method")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        http::header::{
            AUTHORIZATION,
            CONTENT_TYPE,
            REFERER,
        },
        tests::{
            assert_send,
            assert_sync,
        },
    };

    #[test]
    fn assert_send_sync() {
        assert_send::<PreRequestParams>();
        assert_sync::<PreRequestParams>();

        assert_send::<RequestParams>();
        assert_sync::<RequestParams>();
    }

    #[test]
    fn request_params_has_default_content_type() {
        let req = RequestParams::default();

        let headers = req.get_headers();

        assert_eq!(
            Some("application/json"),
            headers
                .get(CONTENT_TYPE)
                .map(|header| header.to_str().unwrap())
        );
    }

    #[test]
    fn set_multiple_headers() {
        let req = RequestParams::new(DEFAULT_NODE_ADDRESS)
            .header(REFERER, HeaderValue::from_str("/not-the-value").unwrap())
            .header(REFERER, HeaderValue::from_str("/People.html#tim").unwrap())
            .header(AUTHORIZATION, HeaderValue::from_str("let me in").unwrap());

        let headers = req.get_headers();

        assert_eq!(
            Some("application/json"),
            headers
                .get(CONTENT_TYPE)
                .map(|header| header.to_str().unwrap())
        );
        assert_eq!(
            Some("/People.html#tim"),
            headers.get(REFERER).map(|header| header.to_str().unwrap())
        );
        assert_eq!(
            Some("let me in"),
            headers
                .get(AUTHORIZATION)
                .map(|header| header.to_str().unwrap())
        );
    }

    #[test]
    fn request_params_has_default_base_url() {
        let req = RequestParams::default();

        assert_eq!(DEFAULT_NODE_ADDRESS, req.get_base_url());
    }

    #[test]
    fn request_params_can_set_base_url() {
        let req = RequestParams::new("http://eshost:9200");

        assert_eq!("http://eshost:9200", req.get_base_url());
    }

    #[test]
    fn request_params_can_set_url_query() {
        let req = RequestParams::new(DEFAULT_NODE_ADDRESS)
            .url_param("pretty", false)
            .url_param("pretty", true);

        assert_eq!((12, Some(String::from("?pretty=true"))), req.get_url_qry());
    }

    #[test]
    fn empty_request_params_returns_empty_string() {
        let req = RequestParams::default();

        assert_eq!((0, None), req.get_url_qry());
    }
}
