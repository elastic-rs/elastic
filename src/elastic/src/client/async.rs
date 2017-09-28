use uuid::Uuid;
use futures::{Future, IntoFuture, Poll};
use futures_cpupool::CpuPool;
use tokio_core::reactor::Handle;
use elastic_reqwest::{AsyncBody, AsyncElasticClient};
use elastic_reqwest::sniffer::{MultipleAddresses, RoundRobin, AsyncClusterSniffer};
use reqwest::unstable::async::{Client as AsyncHttpClient, ClientBuilder as AsyncHttpClientBuilder};

use error::{self, Error, Result};
use client::requests::HttpRequest;
use client::responses::{async_response, AsyncResponseBuilder};
use client::{private, Client, RequestParams, RequestParamsBuilder, Sender, SendableRequest, DEFAULT_NODE_ADDRESS};

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
    addresses: AsyncAddresses,
}

#[derive(Clone)]
enum AsyncAddresses {
    Static(MultipleAddresses<RoundRobin>),
    Sniffed(AsyncClusterSniffer),
}

impl AsyncAddresses {
    fn next(&self) -> Box<Future<Item = RequestParams, Error = Error>> {
        match *self {
            AsyncAddresses::Static(ref addresses) => Box::new(Ok(addresses.next()).into_future()),
            AsyncAddresses::Sniffed(ref sniffer) => Box::new(sniffer.next().map_err(error::request)),
        }
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

        let params_future = self.addresses.next()
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

        let req_future = params_future.and_then(move |params| {
            http.elastic_req(&params, req)
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
    http: Option<AsyncHttpClient>,
    serde_pool: Option<CpuPool>,
    addresses: AsyncAddressesBuilder,
    params: RequestParamsBuilder,
}

enum AsyncAddressesBuilder {
    Static(Vec<String>),
    Sniffed(String),
}

impl Default for AsyncAddressesBuilder {
    fn default() -> Self {
        AsyncAddressesBuilder::Static(vec![DEFAULT_NODE_ADDRESS.to_owned()])
    }
}

impl AsyncAddressesBuilder {
    fn build(self, params: RequestParamsBuilder, client: AsyncHttpClient) -> AsyncAddresses {
        match self {
            AsyncAddressesBuilder::Static(addresses) => {
                AsyncAddresses::Static(MultipleAddresses::round_robin(addresses, params))
            },
            AsyncAddressesBuilder::Sniffed(base_address) => {
                let params = params.build(base_address);

                AsyncAddresses::Sniffed(AsyncClusterSniffer::new(client, params))
            }
        }
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
            http: None,
            serde_pool: None,
            params: RequestParamsBuilder::default(),
            addresses: AsyncAddressesBuilder::default(),
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: RequestParamsBuilder) -> Self {
        AsyncClientBuilder {
            http: None,
            serde_pool: None,
            params: params,
            addresses: AsyncAddressesBuilder::default(),
        }
    }

    /**
    Specify a set of static node addresses to load balance requests on.
    */
    pub fn static_addresses<I>(mut self, addresses: I) -> Self
        where I: IntoIterator<Item = String>
    {
        self.addresses = AsyncAddressesBuilder::Static(addresses.into_iter().collect());

        self
    }

    /**
    Specify a node address to sniff other nodes in the cluster from.
    */
    pub fn sniff_addresses<I>(mut self, address: I) -> Self
        where I: Into<String>
    {
        self.addresses = AsyncAddressesBuilder::Sniffed(address.into());

        self
    }

    /**
    Specify default request parameters.
    
    # Examples
    
    Require all responses use pretty-printing:
    
    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .params(|p| p.url_param("pretty", true));
    ```

    Add an authorization header:

    ```
    # use elastic::prelude::*;
    use elastic::http::header::Authorization;

    let builder = SyncClientBuilder::new()
        .params(|p| p.header(Authorization("let me in".to_owned())));
    ```

    Specify a base url (prefer the [`base_url`][SyncClientBuilder.base_url] method on `SyncClientBuilder` instead):

    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .params(|p| p.base_url("https://my_es_cluster/some_path"));
    ```

    [SyncClientBuilder.base_url]: #method.base_url
    */
    pub fn params<F>(mut self, builder: F) -> Self
    where
        F: Fn(RequestParamsBuilder) -> RequestParamsBuilder,
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

    let builder = AsyncClientBuilder::new()
        .serde_pool(pool);
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

    /** Use the given `reqwest::Client` for sending requests. */
    pub fn http_client(mut self, client: AsyncHttpClient) -> Self {
        self.http = Some(client);

        self
    }

    /** 
    Construct an [`AsyncClient`][AsyncClient] from this builder. 

    [Client]: struct.Client.html
    */
    pub fn build(self, handle: &Handle) -> Result<AsyncClient> {
        let http = self.http
            .map(Ok)
            .unwrap_or_else(|| AsyncHttpClientBuilder::new().build(handle))
            .map_err(error::build)?;
        
        let addresses = self.addresses.build(self.params, http.clone());

        Ok(AsyncClient {
            sender: AsyncSender {
                http: http,
                serde_pool: self.serde_pool,
                addresses: addresses,
            },
        })
    }
}
