use std::sync::Arc;
use std::error::Error as StdError;
use futures::{Future, IntoFuture, Poll};
use futures::future::{FutureResult, Either};
use futures_cpupool::{CpuPool, CpuFuture};
use tokio_core::reactor::Handle;
use reqwest::Error as ReqwestError;
use reqwest::unstable::async::{Client as AsyncHttpClient, ClientBuilder as AsyncHttpClientBuilder, RequestBuilder as AsyncHttpRequestBuilder};
use fluent_builder::FluentBuilder;

use error::{self, Error};
use private;
use client::requests::Endpoint;
use http::{AsyncBody, AsyncHttpRequest, Url};
use client::sender::{build_reqwest_method, build_url, NextParams, NodeAddress, NodeAddresses, NodeAddressesBuilder, NodeAddressesInner, PreRequestParams, RequestParams, SendableRequest, SendableRequestParams, Sender};
use client::sender::sniffed_nodes::SniffedNodesBuilder;
use client::responses::{async_response, AsyncResponseBuilder};
use client::Client;

/** 
An asynchronous Elasticsearch client.

Use an [`AsyncClientBuilder`][AsyncClientBuilder] to configure and build an `AsyncClient`.
For more details about the methods available to an `AsyncClient`, see the base [`Client`][Client] type.

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

[Client]: ../struct.Client.html
[AsyncClientBuilder]: struct.AsyncClientBuilder.html
*/
pub type AsyncClient = Client<AsyncSender>;

/** An asynchronous request sender. */
#[derive(Clone)]
pub struct AsyncSender {
    pub(in client) http: AsyncHttpClient,
    pub(in client) serde_pool: Option<CpuPool>,
    pre_send: Option<Arc<Fn(&mut AsyncHttpRequest) -> Box<Future<Item = (), Error = Box<StdError + Send + Sync>>>>>,
}

impl private::Sealed for AsyncSender {}

impl AsyncSender {
    pub(crate) fn maybe_async<TFn, TResult>(&self, f: TFn)
    -> Either<CpuFuture<TResult, Error>, FutureResult<TResult, Error>>
    where
        TFn: FnOnce() -> Result<TResult, Error> + Send + 'static,
        TResult: Send + 'static,
    {
        if let Some(ref ser_pool) = self.serde_pool {
            Either::A(ser_pool.spawn_fn(f))
        } else {
            Either::B(f().into_future())
        }
    }
}

impl Sender for AsyncSender {
    type Body = AsyncBody;
    type Response = PendingResponse;
    type Params = PendingParams;

    fn send<TEndpoint, TParams, TBody>(&self, request: SendableRequest<TEndpoint, TParams, TBody>) -> Self::Response
    where
        TEndpoint: Into<Endpoint<'static, TBody>>,
        TBody: Into<Self::Body> + 'static,
        TParams: Into<Self::Params> + 'static,
    {
        let correlation_id = request.correlation_id;
        let serde_pool = self.serde_pool.clone();
        let params = request.params;
        let Endpoint {
            url, method, body, ..
        } = request.inner.into();

        info!(
            "Elasticsearch Request: correlation_id: '{}', path: '{}'",
            correlation_id,
            url.as_ref()
        );

        let params_future = match params {
            SendableRequestParams::Value(params) => Either::A(Ok(params).into_future()),
            SendableRequestParams::Builder { params, builder } => {
                let params = params.into().log_err(move |e| {
                    error!(
                        "Elasticsearch Node Selection: correlation_id: '{}', error: '{:?}'",
                        correlation_id, e
                    )
                });

                Either::B(params.and_then(|params| Ok(builder.into_value(move || params))))
            }
        };

        let build_req_future = params_future
            .and_then(move |params| {
                Url::parse(&build_url(&url, &params))
                    .map_err(error::request)
                    .map(|url| (params, url))
            })
            .and_then(move |(params, url)| {
                Ok(AsyncHttpRequest {
                    url,
                    method,
                    headers: params.get_headers(),
                    body: body.map(|body| body.into()),
                    _private: (),
                })
            });

        let pre_send = self.pre_send.clone();
        let pre_send_future = build_req_future.and_then(move |mut req| {
            if let Some(pre_send) = pre_send {
                Either::A(
                    pre_send(&mut req)
                        .map_err(error::wrapped)
                        .map_err(error::request)
                        .and_then(move |_| Ok(req).into_future()),
                )
            } else {
                Either::B(Ok(req).into_future())
            }
        });

        let pre_send_http = self.http.clone();
        let pre_send_future = pre_send_future
            .and_then(move |req| {
                build_reqwest(&pre_send_http, req)
                    .build()
                    .map_err(error::request)
            })
            .log_err(move |e| {
                error!(
                    "Elasticsearch Request: correlation_id: '{}', error: '{:?}'",
                    correlation_id, e
                )
            });

        let req_http = self.http.clone();
        let req_future = pre_send_future.and_then(move |req| {
            req_http
                .execute(req)
                .map_err(error::request)
                .and_then(move |res| {
                    info!(
                        "Elasticsearch Response: correlation_id: '{}', status: '{}'",
                        correlation_id,
                        res.status()
                    );
                    async_response(res, serde_pool).into_future()
                })
                .log_err(move |e| {
                    error!(
                        "Elasticsearch Response: correlation_id: '{}', error: '{:?}'",
                        correlation_id, e
                    )
                })
        });

        PendingResponse::new(req_future)
    }
}

impl NextParams for NodeAddresses<AsyncSender> {
    type Params = PendingParams;

    fn next(&self) -> Self::Params {
        match self.inner {
            NodeAddressesInner::Static(ref nodes) => PendingParams::new(nodes.next().into_future()),
            NodeAddressesInner::Sniffed(ref sniffer) => PendingParams::new(sniffer.next()),
        }
    }
}

/** A future returned by calling `next` on an async set of `NodeAddresses`. */
pub struct PendingParams {
    inner: Box<Future<Item = RequestParams, Error = Error>>,
}

impl PendingParams {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = RequestParams, Error = Error> + 'static,
    {
        PendingParams {
            inner: Box::new(fut),
        }
    }
}

impl Future for PendingParams {
    type Item = RequestParams;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

impl From<RequestParams> for PendingParams {
    fn from(params: RequestParams) -> Self {
        PendingParams::new(Ok(params).into_future())
    }
}

/** Build an asynchronous `reqwest::RequestBuilder` from an Elasticsearch request. */
fn build_reqwest(client: &AsyncHttpClient, req: AsyncHttpRequest) -> AsyncHttpRequestBuilder {
    let AsyncHttpRequest {
        url,
        method,
        headers,
        body,
        ..
    } = req;

    let method = build_reqwest_method(method);

    let mut req = client.request(method, url);
    {
        req.headers(headers);

        if let Some(body) = body {
            req.body(body.into_inner());
        }
    }

    req
}

/** A future returned by calling `send` on an `AsyncSender`. */
pub struct PendingResponse {
    inner: Box<Future<Item = AsyncResponseBuilder, Error = Error>>,
}

impl PendingResponse {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = AsyncResponseBuilder, Error = Error> + 'static,
    {
        PendingResponse {
            inner: Box::new(fut),
        }
    }
}

impl Future for PendingResponse {
    type Item = AsyncResponseBuilder;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/** A builder for an asynchronous client. */
pub struct AsyncClientBuilder {
    serde_pool: Option<CpuPool>,
    nodes: NodeAddressesBuilder,
    params: FluentBuilder<PreRequestParams>,
    pre_send: Option<Arc<Fn(&mut AsyncHttpRequest) -> Box<Future<Item = (), Error = Box<StdError + Send + Sync>>>>>,
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
            params: FluentBuilder::new(),
            nodes: NodeAddressesBuilder::default(),
            pre_send: None,
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: PreRequestParams) -> Self {
        AsyncClientBuilder {
            serde_pool: None,
            params: FluentBuilder::new().value(params),
            nodes: NodeAddressesBuilder::default(),
            pre_send: None,
        }
    }

    /**
    Specify a static node nodes to send requests to.
    */
    pub fn static_node<S>(self, node: S) -> Self
    where
        S: Into<NodeAddress>,
    {
        self.static_nodes(vec![node])
    }

    /**
    Specify a set of static node nodes to load balance requests on.
    */
    pub fn static_nodes<I, S>(mut self, nodes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NodeAddress>,
    {
        let nodes = nodes.into_iter().map(|address| address.into()).collect();
        self.nodes = NodeAddressesBuilder::Static(nodes);

        self
    }

    /**
    Specify a node address to sniff other nodes in the cluster from.

    # Examples

    Use a given base url for sniffing the cluster's node addresses from:

    ```
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .sniff_nodes("http://localhost:9200");
    ```
    */
    pub fn sniff_nodes<I>(mut self, builder: I) -> Self
    where
        I: Into<SniffedNodesBuilder>,
    {
        self.nodes = self.nodes.sniff_nodes(builder.into());

        self
    }

    /**
    Specify a node address to sniff other nodes in the cluster from.

    # Examples

    Use a given base url for sniffing the cluster's node addresses from and specify a minimum duration to wait before refreshing:

    ```
    # use std::time::Duration;
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .sniff_nodes_fluent("http://localhost:9200", |n| n
            .wait(Duration::from_secs(90)));
    ```
    */
    pub fn sniff_nodes_fluent<I, F>(mut self, address: I, builder: F) -> Self
    where
        I: Into<NodeAddress>,
        F: Fn(SniffedNodesBuilder) -> SniffedNodesBuilder + 'static,
    {
        self.nodes = self.nodes.sniff_nodes_fluent(address.into(), builder);

        self
    }

    /**
    Specify default request parameters.
    
    # Examples
    
    Require all responses use pretty-printing:
    
    ```
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .params_fluent(|p| p
            .url_param("pretty", true));
    ```

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = AsyncClientBuilder::new()
        .params_fluent(|p| p
            .header(Authorization("let me in".to_owned())));
    ```
    */
    pub fn params_fluent<F>(mut self, builder: F) -> Self
    where
        F: Fn(PreRequestParams) -> PreRequestParams + 'static,
    {
        self.params = self.params.fluent(builder).boxed();

        self
    }

    /**
    Specify default request parameters.

    # Examples
    
    Require all responses use pretty-printing:
    
    ```
    # use elastic::prelude::*;
    let builder = AsyncClientBuilder::new()
        .params(PreRequestParams::new()
            .url_param("pretty", true));
    ```

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = AsyncClientBuilder::new()
        .params(PreRequestParams::new()
            .header(Authorization("let me in".to_owned())));
    ```
    */
    pub fn params<I>(mut self, params: I) -> Self
    where
        I: Into<PreRequestParams>,
    {
        self.params = self.params.value(params.into());

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
    Specify a function to tweak a raw request before sending.

    This function will be applied to all outgoing requests and gives you the chance to perform operations the require the complete raw request,
    such as request singing.
    Prefer the `params` method on the client or individual requests where possible.
    */
    pub fn pre_send_raw<F>(mut self, pre_send: F) -> Self
    where
        F: Fn(&mut AsyncHttpRequest) -> Box<Future<Item = (), Error = Box<StdError + Send + Sync>>> + 'static,
    {
        self.pre_send = Some(Arc::new(pre_send));

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
    where
        TIntoHttp: IntoAsyncHttpClient,
    {
        let http = client.into_async_http_client().map_err(error::build)?;
        let params = self.params.into_value(|| PreRequestParams::default());

        let sender = AsyncSender {
            http: http,
            serde_pool: self.serde_pool,
            pre_send: self.pre_send,
        };

        let addresses = self.nodes.build(params, sender.clone());

        Ok(AsyncClient {
            sender: sender,
            addresses: addresses,
        })
    }
}

struct PendingLogErr<F, L> {
    future: F,
    log: Option<L>,
}

impl<F, L> Future for PendingLogErr<F, L>
where
    F: Future,
    L: FnOnce(&F::Error),
{
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.future.poll() {
            Err(e) => {
                let log = self.log.take().expect("attempted to poll log twice");
                log(&e);
                Err(e)
            }
            other => other,
        }
    }
}

trait LogErr<E>
where
    Self: Sized,
{
    fn log_err<L>(self, log: L) -> PendingLogErr<Self, L>
    where
        L: FnOnce(&E);
}

impl<F, T, E> LogErr<E> for F
where
    F: Future<Item = T, Error = E>,
{
    fn log_err<L>(self, log: L) -> PendingLogErr<F, L>
    where
        L: FnOnce(&E),
    {
        PendingLogErr {
            future: self,
            log: Some(log),
        }
    }
}
