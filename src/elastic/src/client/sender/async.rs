use std::sync::Arc;
use std::error::Error as StdError;
use uuid::Uuid;
use futures::{Future, IntoFuture, Poll};
use futures_cpupool::CpuPool;
use tokio_core::reactor::Handle;
use reqwest::Error as ReqwestError;
use reqwest::unstable::async::{Client as AsyncHttpClient, ClientBuilder as AsyncHttpClientBuilder, RequestBuilder as AsyncHttpRequestBuilder};

use error::{self, Error};
use private;
use client::requests::{AsyncBody, HttpRequest};
use client::sender::{DEFAULT_NODE_ADDRESS, build_method, build_url, RequestParams, PreRequestParams, Sender, SendableRequest};
use client::sender::static_nodes::StaticNodes;
use client::sender::sniffed_nodes::SniffedNodes;
use client::responses::{async_response, AsyncResponseBuilder};
use client::Client;

/** 
An asynchronous Elasticsearch client.

Use an [`AsyncClientBuilder`][AsyncClientBuilder] to configure and build an `AsyncClient`.

# Examples

Create an asynchronous `Client` and send a ping request:

```no_run
# extern crate futures;
# extern crate tokio_core;
# extern crate elastic;
# use futures::Future;
# use tokio_core::reactor::Core;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let mut core = Core::new()?;
let client = AsyncClientBuilder::new().build(&core.handle())?;

let response_future = client.request(PingRequest::new())
                            .send()
                            .and_then(|res| res.into_response::<PingResponse>());

core.run(response_future)?;
# Ok(())
# }
```

[AsyncClientBuilder]: struct.AsyncClientBuilder.html
*/
pub type AsyncClient = Client<AsyncSender>;

/** An asynchronous request sender. */
#[derive(Clone)]
pub struct AsyncSender {
    pub(in client) http: AsyncHttpClient,
    pub(in client) serde_pool: Option<CpuPool>,
    nodes: AsyncNodes,
}

#[derive(Clone)]
enum AsyncNodes {
    Static(StaticNodes),
    Sniffed(Box<SniffedNodes<AsyncSender>>),
}

impl AsyncNodes {
    fn next(&self) -> Box<Future<Item = RequestParams, Error = Error>> {
        match *self {
            AsyncNodes::Static(ref nodes) => Box::new(Ok(nodes.next()).into_future()),
            AsyncNodes::Sniffed(ref sniffer) => Box::new(sniffer.next().map_err(error::request)),
        }
    }
}

impl AsyncSender {
    fn params(&self, correlation_id: Uuid, params_builder: Option<Box<Fn(RequestParams) -> RequestParams>>) -> Box<Future<Item = RequestParams, Error = Error>> {
        let params_future = self.nodes.next()
            .map_err(move |e| {
                error!(
                    "Elasticsearch Node Selection: correlation_id: '{}', error: '{}'",
                    correlation_id,
                    e
                );
                e
            })
            .map(move |params| {
                if let Some(params_builder) = params_builder {
                   params_builder(params)
                } else {
                    params
                }
            });
        
        Box::new(params_future)
    }
}

impl private::Sealed for AsyncSender {}

impl Sender for AsyncSender {
    type Body = AsyncBody;
    type Response = Pending;

    fn send<TRequest, TBody>(&self, request: SendableRequest<TRequest, TBody>) -> Self::Response
    where
        TRequest: Into<HttpRequest<'static, TBody>>,
        TBody: Into<Self::Body> + 'static
    {
        let serde_pool = self.serde_pool.clone();
        let http = self.http.clone();
        let correlation_id = Uuid::new_v4();
        let params_builder = request.params_builder;
        let req = request.inner.into();

        info!(
            "Elasticsearch Request: correlation_id: '{}', path: '{}'",
            correlation_id,
            req.url.as_ref()
        );

        let params_future = self.params(correlation_id, params_builder);

        let req_future = params_future.and_then(move |params| {
            build_req(&http, params, req)
                .send()
                .map_err(move |e| {
                    error!(
                        "Elasticsearch Response: correlation_id: '{}', error: '{}'",
                        correlation_id,
                        e
                    );
                    error::request(e)
                })
                .map(move |res| {
                    info!(
                        "Elasticsearch Response: correlation_id: '{}', status: '{}'",
                        correlation_id,
                        res.status()
                    );
                    async_response(res, serde_pool)
                })
        });

        Pending::new(req_future)
    }
}

/** Build an asynchronous `reqwest::RequestBuilder` from an Elasticsearch request. */
fn build_req<I, B>(client: &AsyncHttpClient, params: &RequestParams, req: I) -> AsyncHttpRequestBuilder
where
    I: Into<HttpRequest<'static, B>>,
    B: Into<AsyncBody>,
{
    let req = req.into();

    let url = build_url(&req.url, &params);
    let method = build_method(req.method);
    let body = req.body;

    let mut req = client.request(method, &url);
    {
        req.headers(params.get_headers());

        if let Some(body) = body {
            req.body(body.into().into_inner());
        }
    }

    req
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<Future<Item = AsyncResponseBuilder, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = AsyncResponseBuilder, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = AsyncResponseBuilder;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/** A builder for an asynchronous client. */
pub struct AsyncClientBuilder {
    serde_pool: Option<CpuPool>,
    nodes: AsyncNodesBuilder,
    params: PreRequestParams,
}

enum AsyncNodesBuilder {
    Static(Vec<Arc<str>>),
    Sniffed(Arc<str>),
}

impl Default for AsyncNodesBuilder {
    fn default() -> Self {
        AsyncNodesBuilder::Static(vec![DEFAULT_NODE_ADDRESS.into()])
    }
}

impl AsyncNodesBuilder {
    fn build(self, params: PreRequestParams, client: AsyncHttpClient) -> AsyncNodes {
        match self {
            AsyncNodesBuilder::Static(nodes) => {
                AsyncNodes::Static(StaticNodes::round_robin(nodes, params))
            },
            AsyncNodesBuilder::Sniffed(default_node) => {
                let params = RequestParams::from_parts(default_node, params);

                AsyncNodes::Sniffed(SniffedNodes::new(client, params))
            }
        }
    }
}

/**
A type that can be used to construct an async http client.

This trait has a few default implementations:

- `AsyncHttpClient`: returns `self`
- `Handle`: returns a new `AsyncHttpClient` bound to `self`.
*/
pub trait IntoAsyncHttpClient {
    /** The type of error returned by the conversion. */
    type Error: StdError + Send + 'static;

    /** Convert `self` into an `AsyncHttpClient`. */
    fn into_async_http_client(self) -> Result<AsyncHttpClient, Self::Error>;
}

impl IntoAsyncHttpClient for AsyncHttpClient {
    type Error = Error;

    fn into_async_http_client(self) -> Result<AsyncHttpClient, Self::Error> {
        Ok(self)
    }
}

impl<'a> IntoAsyncHttpClient for &'a Handle {
    type Error = ReqwestError;

    fn into_async_http_client(self) -> Result<AsyncHttpClient, Self::Error> {
        AsyncHttpClientBuilder::new().build(self)
    }
}

impl Default for AsyncClientBuilder {
    fn default() -> Self {
        AsyncClientBuilder::new()
    }
}

impl AsyncClientBuilder {
    /** 
    Create a new client builder.

    By default, a client constructed by this builder will:

    - Send requests to `localhost:9200`
    - Not deserialise repsonses on a cpu pool
    - Not use any authentication
    - Not use TLS
    */
    pub fn new() -> Self {
        AsyncClientBuilder {
            serde_pool: None,
            params: PreRequestParams::default(),
            nodes: AsyncNodesBuilder::default(),
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: PreRequestParams) -> Self {
        AsyncClientBuilder {
            serde_pool: None,
            params: params,
            nodes: AsyncNodesBuilder::default(),
        }
    }

    /**
    Specify a static node nodes to send requests to.
    */
    pub fn static_node<S>(self, node: S) -> Self
        where S: Into<Arc<str>>,
    {
        self.static_nodes(vec![node])
    }

    /**
    Specify a set of static node nodes to load balance requests on.
    */
    pub fn static_nodes<I, S>(mut self, nodes: I) -> Self
        where I: IntoIterator<Item = S>,
              S:Into<Arc<str>>,
    {
        let nodes = nodes.into_iter().map(|address| address.into()).collect();
        self.nodes = AsyncNodesBuilder::Static(nodes);

        self
    }

    /**
    Specify a node address to sniff other nodes in the cluster from.
    */
    pub fn sniff_nodes<I>(mut self, address: I) -> Self
        where I: Into<Arc<str>>
    {
        self.nodes = AsyncNodesBuilder::Sniffed(address.into());

        self
    }

    /**
    Specify default request parameters.
    
    # Examples
    
    Require all responses use pretty-printing:
    
    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .params(|p| {
            p.url_param("pretty", true)
        });
    ```

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = SyncClientBuilder::new()
        .params(|p| {
            p.header(Authorization("let me in".to_owned()))
        });
    ```

    Specify a base url (prefer the [`base_url`][SyncClientBuilder.base_url] method on `SyncClientBuilder` instead):

    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .params(|p| {
            p.base_url("https://my_es_cluster/some_path")
        });
    ```

    [SyncClientBuilder.base_url]: #method.base_url
    */
    pub fn params<F>(mut self, builder: F) -> Self
    where
        F: Fn(PreRequestParams) -> PreRequestParams,
    {
        self.params = builder(self.params);

        self
    }

    /** 
    Use the given `CpuPool` for serialising and deserialising responses.

    If the pool is `None` then responses will be serialised and deserialised on the same thread as the io `Core`.

    # Examples

    Use a cpu pool to serialise and deserialise responses:

    ```
    # extern crate futures_cpupool;
    # extern crate elastic;
    # use elastic::prelude::*;
    # use futures_cpupool::CpuPool;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    let pool = CpuPool::new(4);

    let builder = AsyncClientBuilder::new().serde_pool(pool);
    # Ok(())
    # }
    ```
    */
    pub fn serde_pool<P>(mut self, serde_pool: P) -> Self
    where
        P: Into<Option<CpuPool>>,
    {
        self.serde_pool = serde_pool.into();

        self
    }

    /** 
    Construct an [`AsyncClient`][AsyncClient] from this builder.

    The `build` method accepts any type that can be used to construct a http client from.

    # Examples

    Build with an asynchronous `Handle`.
    This will build an `AsyncClient` with a default underlying `AsyncHttpClient` using the handle.

    ```no_run
    # extern crate tokio_core;
    # extern crate elastic;
    # use elastic::prelude::*;
    # use tokio_core::reactor::Core;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    let mut core = Core::new()?;

    let builder = AsyncClientBuilder::new().build(&core.handle());
    # Ok(())
    # }
    ```

    Build with a given `AsyncHttpClient`.

    ```no_run
    # extern crate tokio_core;
    # extern crate reqwest;
    # extern crate elastic;
    # use tokio_core::reactor::Core;
    # use reqwest::unstable::async::Client;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let builder = AsyncClientBuilder::new().build(client);
    # Ok(())
    # }
    ```

    [AsyncClient]: type.AsyncClient.html
    */
    pub fn build<TIntoHttp>(self, client: TIntoHttp) -> Result<AsyncClient, Error> 
        where TIntoHttp: IntoAsyncHttpClient
    {
        let http = client.into_async_http_client()
            .map_err(error::build)?;
        
        let nodes = self.nodes.build(self.params, http.clone());

        Ok(AsyncClient {
            sender: AsyncSender {
                http: http,
                serde_pool: self.serde_pool,
                nodes: nodes,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use reqwest::unstable::async::{Client, RequestBuilder};
    use reqwest::header::ContentType;
    use tokio_core::reactor::Core;

    use super::*;
    use req::*;

    fn params() -> RequestParams {
        RequestParams::new("eshost:9200/path")
            .url_param("pretty", true)
    }

    fn expected_req(cli: &Client, method: Method, url: &str, body: Option<Vec<u8>>) -> RequestBuilder {
        let mut req = cli.request(method, url);
        {
            req.header(ContentType::json());

            if let Some(body) = body {
                req.body(body);
            }
        }

        req
    }

    fn assert_req(expected: RequestBuilder, actual: RequestBuilder) {
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    fn core() -> Core {
        Core::new().unwrap()
    }

    #[test]
    fn head_req() {
        let cli = Client::new(&core().handle());
        let req = build_req(&cli, &params(), PingHeadRequest::new());

        let url = "eshost:9200/path/?pretty=true";

        let expected = expected_req(&cli, Method::Head, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn get_req() {
        let cli = Client::new(&core().handle());
        let req = build_req(&cli, &params(), SimpleSearchRequest::new());

        let url = "eshost:9200/path/_search?pretty=true";

        let expected = expected_req(&cli, Method::Get, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn post_req() {
        let cli = Client::new(&core().handle());
        let req = build_req(
            &cli,
            &params(),
            PercolateRequest::for_index_ty("idx", "ty", vec![]),
        );

        let url = "eshost:9200/path/idx/ty/_percolate?pretty=true";

        let expected = expected_req(&cli, Method::Post, url, Some(vec![]));

        assert_req(expected, req);
    }

    #[test]
    fn put_req() {
        let cli = Client::new(&core().handle());
        let req = build_req(
            &cli,
            &params(),
            IndicesCreateRequest::for_index("idx", vec![]),
        );

        let url = "eshost:9200/path/idx?pretty=true";

        let expected = expected_req(&cli, Method::Put, url, Some(vec![]));

        assert_req(expected, req);
    }

    #[test]
    fn delete_req() {
        let cli = Client::new(&core().handle());
        let req = build_req(&cli, &params(), IndicesDeleteRequest::for_index("idx"));

        let url = "eshost:9200/path/idx?pretty=true";

        let expected = expected_req(&cli, Method::Delete, url, None);

        assert_req(expected, req);
    }

    #[test]
    fn owned_string_into_body() {
        AsyncBody::from(String::new());
    }

    #[test]
    fn borrowed_string_into_body() {
        AsyncBody::from("abc");
    }

    #[test]
    fn owned_vec_into_body() {
        AsyncBody::from(Vec::new());
    }

    #[test]
    fn borrowed_vec_into_body() {
        static BODY: &'static [u8] = &[0, 1, 2];

        AsyncBody::from(BODY);
    }

    #[test]
    fn empty_body_into_body() {
        AsyncBody::from(empty_body());
    }

    #[test]
    fn json_value_into_body() {
        AsyncBody::from(json!({}));
    }
}
