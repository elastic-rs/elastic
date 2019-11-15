/*!
Request types for the Elasticsearch REST API.

This module contains implementation details that are useful if you want to customise the request process, but aren't generally important for sending requests.
*/

use fluent_builder::{
    SharedFluentBuilder,
    TryIntoValue,
};
use serde::de::DeserializeOwned;
#[cfg(feature="async_sender")]
use std::sync::Arc;
#[cfg(feature="async_sender")]
use tokio_threadpool::ThreadPool;

use crate::{
    client::Client,
    endpoints::IntoEndpoint,
    error::Error,
    http::{
        receiver::IsOk,
        sender::{
            RequestParams,
            SendableRequest,
            SendableRequestParams,
            Sender,
            TypedSender,
        },
    },
};
#[cfg(feature="async_sender")]
use crate::http::sender::AsyncSender;

pub mod raw;

#[doc(inline)]
pub use self::raw::RawRequestBuilder;

// Search requests
pub mod search;

#[doc(inline)]
pub use self::search::SearchRequestBuilder;

// Sql requests
pub mod sql;

#[doc(inline)]
pub use self::sql::SqlRequestBuilder;

// Document requests
pub mod document_delete;
pub mod document_get;
pub mod document_index;
pub mod document_put_mapping;
pub mod document_update;

#[doc(inline)]
pub use self::{
    document_delete::DeleteRequestBuilder,
    document_get::GetRequestBuilder,
    document_index::IndexRequestBuilder,
    document_put_mapping::PutMappingRequestBuilder,
    document_update::UpdateRequestBuilder,
};

// Index requests
pub mod index_close;
pub mod index_create;
pub mod index_delete;
pub mod index_exists;
pub mod index_open;

#[doc(inline)]
pub use self::{
    index_close::IndexCloseRequestBuilder,
    index_create::IndexCreateRequestBuilder,
    index_delete::IndexDeleteRequestBuilder,
    index_exists::IndexExistsRequestBuilder,
    index_open::IndexOpenRequestBuilder,
};

// Misc requests
pub mod bulk;
pub mod ping;

#[doc(inline)]
pub use self::{
    bulk::BulkRequestBuilder,
    ping::PingRequestBuilder,
};

pub mod common;

/**
Trait for inner request object, that varies based on what request is being made.

Note that `RawRequestInner` does _not_ implement this, since it passes the response
as a raw object without deserialization.
*/
pub trait RequestInner {
    /**
    Full request type for this request.
    */
    type Request: IntoEndpoint<'static> + Send + 'static;

    /**
    Type for the response of the request.
    */
    type Response: IsOk + DeserializeOwned + Send + 'static;

    /**
    Converts this request builder to its corresponding request type.
    */
    fn into_request(self) -> Result<Self::Request, Error>;
}

/**
A builder for a request.

This structure wraps up a concrete REST API request type and lets you adjust parameters before sending it.
The `RequestBuilder` has two generic parameters:

- `TSender`: the kind of request sender. This can be either synchronous or asynchronous
- `TRequest`: the inner request type, for example `SearchRequestBuilder`.

`RequestBuilder` contains methods that are common to all request builders.
*/
pub struct RequestBuilder<TSender, TRequest>
where
    TSender: Sender,
{
    client: Client<TSender>,
    params_builder: SharedFluentBuilder<RequestParams>,
    inner: TRequest,
}

/**
# Methods for any request builder

The following methods can be called on any request builder, whether it's synchronous or asynchronous.
*/
impl<TSender, TRequest> RequestBuilder<TSender, TRequest>
where
    TSender: Sender,
{
    fn initial(client: Client<TSender>, req: TRequest) -> Self {
        RequestBuilder {
            client,
            params_builder: SharedFluentBuilder::new(),
            inner: req,
        }
    }

    fn new(
        client: Client<TSender>,
        builder: SharedFluentBuilder<RequestParams>,
        req: TRequest,
    ) -> Self {
        RequestBuilder {
            client,
            params_builder: builder,
            inner: req,
        }
    }

    /**
    Override the parameters for this request.

    This method will box the given closure and use it to mutate the request parameters.
    It will be called after a node address has been chosen so `params` can be used to override the url a request will be sent to.
    Each call to `params` will be chained so it can be called multiple times but it's recommended to only call once.

    # Examples

    Add a url param to force an index refresh:

    ```no_run
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .params_fluent(|p| p.url_param("refresh", true));
    # Ok(())
    # }
    ```

    Force the request to be sent to `http://different-host:9200`:

    ```no_run
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .params_fluent(|p| p.base_url("http://different-host:9200"));
    # Ok(())
    # }
    ```
    */
    pub fn params_fluent(
        mut self,
        builder: impl Fn(RequestParams) -> RequestParams + Send + 'static,
    ) -> Self {
        self.params_builder = self.params_builder.fluent(builder).shared();

        self
    }

    /**
    Specify default request parameters.

    This method differs from `params_fluent` by not taking any default parameters into account.
    The `RequestParams` passed in are exactly the `RequestParams` used to build the request.

    # Examples

    Add a url param to force an index refresh and send the request to `http://different-host:9200`:

    ```no_run
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .params(RequestParams::new("http://different-hos:9200").url_param("refresh", true));
    # Ok(())
    # }
    ```
    */
    pub fn params(mut self, params: impl Into<RequestParams>) -> Self {
        self.params_builder = self.params_builder.value(params.into());

        self
    }
}

impl<TSender, TReqInner> RequestBuilder<TSender, TReqInner>
where
    TReqInner: RequestInner,
    TSender: TypedSender<TReqInner>,
    <TReqInner::Request as IntoEndpoint<'static>>::BodyType: Send + Into<TSender::Body>,
{
    /**
    Sends the request.

    The returned object is a `Sender`-implementation-specific handle to the results
    of the query. For example, for the `SyncSender` this is a plain `Result<T, elastic::error::Error>`,
    while for the `AsyncSender` this is an object implementing `Future<Item=T, Error=elastic::error::Error>`.

    For information on the result type, consult the page for the request.
    */
    pub fn send(self) -> TSender::TypedResponse {
        let client = self.client;
        let params_builder = self.params_builder;
        let request_res = self.inner
            .into_request()
            .map(|req| {
                let endpoint = req.into_endpoint();

                // Only try fetch a next address if an explicit `RequestParams` hasn't been given
                let params = match params_builder.try_into_value() {
                    TryIntoValue::Value(value) => SendableRequestParams::Value(value),
                    TryIntoValue::Builder(builder) => SendableRequestParams::Builder {
                        params: client.sender.next_params(&client.addresses),
                        builder,
                    },
                };

                SendableRequest::new(endpoint, params)
            });
        client.sender.typed_send(request_res)
    }
}

/**
# Methods for asynchronous request builders

The following methods can be called on any asynchronous request builder.
*/
#[cfg(feature="async_sender")]
impl<TRequest> RequestBuilder<AsyncSender, TRequest> {
    /**
    Override the thread pool used for deserialisation for this request.

    # Examples

    Use the given thread pool to deserialise the response:

    ```no_run
    # extern crate tokio_threadpool;
    # use std::sync::Arc;
    # use tokio_threadpool::ThreadPool;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let pool = ThreadPool::new();
    let builder = client.request(get_req())
                        .serde_pool(Arc::new(pool));
    # Ok(())
    # }
    ```

    Never deserialise the response on a thread pool:

    ```no_run
    # extern crate tokio_threadpool;
    # use tokio_threadpool::ThreadPool;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .serde_pool(None);
    # Ok(())
    # }
    ```
    */
    pub fn serde_pool(mut self, pool: impl Into<Option<Arc<ThreadPool>>>) -> Self {
        self.client.sender.serde_pool = pool.into();

        self
    }
}

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::bulk::{
        bulk,
        bulk_raw,
        BulkOperation,
    };

    pub use super::{
        DeleteRequestBuilder,
        GetRequestBuilder,
        IndexCloseRequestBuilder,
        IndexCreateRequestBuilder,
        IndexDeleteRequestBuilder,
        IndexOpenRequestBuilder,
        IndexRequestBuilder,
        PingRequestBuilder,
        PutMappingRequestBuilder,
        RawRequestBuilder,
        SearchRequestBuilder,
        SqlRequestBuilder,
        UpdateRequestBuilder,
    };
}
