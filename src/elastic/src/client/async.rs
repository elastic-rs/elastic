use std::error::Error as StdError;
use uuid::Uuid;
use futures::{Future, Poll};
use futures::future::{FutureResult, IntoFuture, Either};
use futures_cpupool::{CpuPool, CpuFuture};
use elastic_reqwest::{AsyncBody, AsyncElasticClient};
use reqwest::Error as ReqwestError;
use reqwest::async::{Client as AsyncHttpClient, ClientBuilder as AsyncHttpClientBuilder};

use error::{self, Error};
use client::requests::HttpRequest;
use client::responses::{async_response, AsyncResponseBuilder};
use client::{private, Client, RequestParams, Sender};

/** 
An asynchronous Elasticsearch client.

Use an [`AsyncClientBuilder`][AsyncClientBuilder] to configure and build an `AsyncClient`.

# Examples

Create an asynchronous `Client` and send a ping request:

```no_run
# extern crate futures;
# extern crate tokio;
# extern crate elastic;
# use futures::Future;
# use tokio::runtime::current_thread::block_on_all;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let client = AsyncClientBuilder::new().build()?;

let response_future = client.ping().send();

block_on_all(response_future)?;
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
}

impl private::Sealed for AsyncSender {}

impl Sender for AsyncSender {
    type Body = AsyncBody;
    type Response = Pending;

    fn send<TRequest, TBody>(&self, req: TRequest, params: &RequestParams) -> Self::Response
    where
        TRequest: Into<HttpRequest<'static, TBody>>,
        TBody: Into<Self::Body>,
    {
        let serde_pool = self.serde_pool.clone();
        let correlation_id = Uuid::new_v4();
        let req = req.into();

        info!(
            "Elasticsearch Request: correlation_id: '{}', method: '{:?}', path: '{}'",
            correlation_id,
            req.method,
            req.url.as_ref()
        );

        let req_future = self.http
            .elastic_req(params, req)
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
            });

        Pending::new(req_future)
    }
}

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
    params: RequestParams,
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
            params: RequestParams::default(),
        }
    }

    /**
    Create a new client builder with the given default request parameters.
    */
    pub fn from_params(params: RequestParams) -> Self {
        AsyncClientBuilder {
            serde_pool: None,
            params: params,
        }
    }

    /**
    Set the base url. 

    The url must be fully qualified.
    This method is a convenient alternative to using `params` to specify the `base_url`.

    # Examples

    Specify a base url for the client to send requests to.
    In this case, the base url is HTTPS, and not on the root path:

    ```
    # use elastic::prelude::*;
    let builder = SyncClientBuilder::new()
        .base_url("https://my_es_cluster/some_path");
    ```
    */
    pub fn base_url<I>(mut self, base_url: I) -> Self
    where
        I: Into<String>,
    {
        self.params = self.params.base_url(base_url);

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
        F: Fn(RequestParams) -> RequestParams,
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
    # extern crate tokio;
    # extern crate elastic;
    # use elastic::prelude::*;
    # use tokio::runtime::current_thread::block_on_all;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {

    let builder = AsyncClientBuilder::new().build();
    # Ok(())
    # }
    ```
    [AsyncClient]: type.AsyncClient.html
    */
    
    pub fn build(self) -> Result<AsyncClient, Error> {
        let http = AsyncHttpClient::new();

        Ok(AsyncClient {
            sender: AsyncSender {
                http: http,
                serde_pool: self.serde_pool,
            },
            params: self.params,
        })
    }
}
