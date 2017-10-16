use std::sync::Arc;
use std::collections::HashMap;

use reqwest;
use reqwest::header::{Headers, ContentType};

/**
An incomplete set of request parameters.

The difference between `PreRequestParams` and `RequestParams` is the absense of a base url to send requests to.
When requests are load balanced between multiple Elasticsearch nodes the url to send a request to might not be known upfront.
*/
#[derive(Clone)]
pub struct PreRequestParams {
    url_params: Arc<HashMap<&'static str, String>>,
    // We should be able to replace this with `Arc<HeadersMap>` from the `http` crate
    headers_builder: Option<Arc<Fn(&mut Headers) + Send + Sync + 'static>>,
}

/**
Parameters for a single REST API request.

The `RequestParams` struct allows you to set headers and url parameters for a given request.
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
    base_url: Arc<str>,
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
            headers_builder: None,
            url_params: Arc::new(HashMap::new()),
        }
    }

    /** 
    Set a url param value.
    
    These parameters are added as query parameters to request urls.
    */
    pub fn url_param<T: ToString>(mut self, key: &'static str, value: T) -> Self {
        Arc::make_mut(&mut self.url_params).insert(key, value.to_string());
        self
    }

    /** Set a request header. */
    pub fn header<H>(self, header: H) -> Self
    where
        H: Header + Clone,
    {
        self.headers(move |h| h.set(header.clone()))
    }

    /** 
    Set a header value on the params.
    
    Each call to `headers` will chain to the end of the last call.
    This function allocates a new `Arc` for each call.
    */
    fn headers<F>(mut self, headers_builder: F) -> Self
    where
        F: Fn(&mut Headers) + Send + Sync + 'static,
    {
        if let Some(old_headers_builder) = self.headers_builder {
            let headers_builder = move |mut headers: &mut Headers| {
                old_headers_builder(&mut headers);
                headers_builder(&mut headers);
            };

            self.headers_builder = Some(Arc::new(headers_builder));
        } else {
            self.headers_builder = Some(Arc::new(headers_builder));
        }

        self
    }
}

impl Default for PreRequestParams {
    fn default() -> Self {
        PreRequestParams::new()
    }
}

impl RequestParams {
    /** Create a container for request parameters from a base url and pre request parameters. */
    pub fn from_parts<T: Into<Arc<str>>>(base_url: T, inner: PreRequestParams) -> Self {
        RequestParams {
            base_url: base_url.into(),
            inner: inner,
        }
    }

    /** 
    Create a new container for request parameters.
    
    This method takes a fully-qualified url for the Elasticsearch node.
    It will also set the `Content-Type` header to `application/json`.
    */
    pub fn new<T: Into<Arc<str>>>(base_url: T) -> Self {
        RequestParams::from_parts(base_url.into(), PreRequestParams::new())
    }

    /** Set the base url for the Elasticsearch node. */
    pub fn base_url<T: Into<Arc<str>>>(mut self, base_url: T) -> Self {
        self.base_url = base_url.into();
        self
    }

    /** 
    Set a url param value.
    
    These parameters are added as query parameters to request urls.
    */
    pub fn url_param<T: ToString>(mut self, key: &'static str, value: T) -> Self {
        self.inner = self.inner.url_param(key, value);
        self
    }

    /** Set a request header. */
    pub fn header<H>(mut self, header: H) -> Self
    where
        H: Header + Clone,
    {
        self.inner = self.inner.header(header);
        self
    }

    /** Get the base url. */
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    /** Create a new `Headers` structure, and thread it through the configuration functions. */
    pub fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(ContentType::json());

        if let Some(ref headers_builder) = self.inner.headers_builder {
            headers_builder(&mut headers);
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
    pub fn split(self) -> (Arc<str>, PreRequestParams) {
        (self.base_url, self.inner)
    }
}

impl Default for RequestParams {
    fn default() -> Self {
        RequestParams::new(DEFAULT_NODE_ADDRESS)
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
mod tests {
    use reqwest::header::{Authorization, ContentType, Referer};
    use super::*;

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

        assert_eq!(Some(&ContentType::json()), headers.get::<ContentType>());
    }

    #[test]
    fn set_multiple_headers() {
        let req = RequestParams::new(DEFAULT_NODE_ADDRESS)
            .header(Referer::new("/not-the-value"))
            .header(Referer::new("/People.html#tim"))
            .header(Authorization("let me in".to_owned()));

        let headers = req.get_headers();

        assert_eq!(Some(&ContentType::json()), headers.get::<ContentType>());
        assert_eq!(
            Some(&Referer::new("/People.html#tim")),
            headers.get::<Referer>()
        );
        assert_eq!(
            Some(&Authorization("let me in".to_owned())),
            headers.get::<Authorization<String>>()
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

        assert_eq!(
            (12, Some(String::from("?pretty=true"))),
            req.get_url_qry()
        );
    }

    #[test]
    fn empty_request_params_returns_empty_string() {
        let req = RequestParams::default();

        assert_eq!((0, None), req.get_url_qry());
    }
}
