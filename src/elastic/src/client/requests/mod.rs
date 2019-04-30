/*!
Request types for the Elasticsearch REST API.

This module contains implementation details that are useful if you want to customise the request process, but aren't generally important for sending requests.
*/

use fluent_builder::FluentBuilder;
use std::sync::Arc;
use tokio_threadpool::ThreadPool;

use client::sender::{
    AsyncSender,
    RequestParams,
    Sender,
};
use client::Client;

pub use elastic_requests::endpoints;
pub use elastic_requests::params;
pub use elastic_requests::{
    empty_body,
    DefaultBody,
    Endpoint,
    UrlPath,
};

pub use self::endpoints::*;
pub use self::params::*;

pub mod raw;
pub use self::raw::RawRequestBuilder;

// Search requests
pub mod search;
pub use self::search::SearchRequestBuilder;

// Sql requests
pub mod sql;
pub use self::sql::SqlRequestBuilder;

// Document requests
pub mod document_delete;
pub mod document_get;
pub mod document_index;
pub mod document_put_mapping;
pub mod document_update;
pub use self::document_delete::DeleteRequestBuilder;
pub use self::document_get::GetRequestBuilder;
pub use self::document_index::IndexRequestBuilder;
pub use self::document_put_mapping::PutMappingRequestBuilder;
pub use self::document_update::UpdateRequestBuilder;

// Index requests
pub mod index_close;
pub mod index_create;
pub mod index_delete;
pub mod index_exists;
pub mod index_open;
pub use self::index_close::IndexCloseRequestBuilder;
pub use self::index_create::IndexCreateRequestBuilder;
pub use self::index_delete::IndexDeleteRequestBuilder;
pub use self::index_exists::IndexExistsRequestBuilder;
pub use self::index_open::IndexOpenRequestBuilder;

// Misc requests
pub mod bulk;
pub mod ping;
pub use self::bulk::BulkRequestBuilder;
pub use self::ping::PingRequestBuilder;

pub mod common;

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
    params_builder: FluentBuilder<RequestParams>,
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
            client: client,
            params_builder: FluentBuilder::new(),
            inner: req,
        }
    }

    fn new(client: Client<TSender>, builder: FluentBuilder<RequestParams>, req: TRequest) -> Self {
        RequestBuilder {
            client: client,
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
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .params_fluent(|p| p.url_param("refresh", true));
    # Ok(())
    # }
    ```

    Force the request to be sent to `http://different-host:9200`:

    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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
        builder: impl Fn(RequestParams) -> RequestParams + 'static,
    ) -> Self {
        self.params_builder = self.params_builder.fluent(builder).boxed();

        self
    }

    /**
    Specify default request parameters.

    This method differs from `params_fluent` by not taking any default parameters into account.
    The `RequestParams` passed in are exactly the `RequestParams` used to build the request.

    # Examples

    Add a url param to force an index refresh and send the request to `http://different-host:9200`:

    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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

/**
# Methods for asynchronous request builders

The following methods can be called on any asynchronous request builder.
*/
impl<TRequest> RequestBuilder<AsyncSender, TRequest> {
    /**
    Override the thread pool used for deserialisation for this request.

    # Examples

    Use the given thread pool to deserialise the response:

    ```no_run
    # extern crate tokio;
    # extern crate tokio_threadpool;
    # extern crate elastic;
    # use std::sync::Arc;
    # use tokio_threadpool::ThreadPool;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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
    # extern crate tokio;
    # extern crate tokio_threadpool;
    # extern crate elastic;
    # use tokio_threadpool::ThreadPool;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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

    pub use super::endpoints::*;
    pub use super::params::*;

    pub use super::bulk::{
        bulk,
        bulk_raw,
        BulkOperation,
    };

    pub use super::{
        empty_body,
        DefaultBody,
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
