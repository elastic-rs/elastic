/*!
Request types for the Elasticsearch REST API.

This module contains implementation details that are useful if you want to customise the request process, but aren't generally important for sending requests.
*/

use std::sync::Arc;
use futures_cpupool::CpuPool;

use client::Client;
use client::sender::{AsyncSender, RequestParams, Sender};

pub use elastic_requests::{empty_body, DefaultBody, HttpMethod, HttpRequest, Url};
pub use elastic_requests::params;
pub use elastic_requests::endpoints;

pub use self::params::*;
pub use self::endpoints::*;

mod sync;
mod async;
pub use self::sync::*;
pub use self::async::*;

pub mod raw;
pub use self::raw::RawRequestBuilder;

// Search requests
pub mod search;
pub use self::search::SearchRequestBuilder;

// Document requests
pub mod document_get;
pub mod document_index;
pub mod document_update;
pub mod document_delete;
pub mod document_put_mapping;
pub use self::document_get::GetRequestBuilder;
pub use self::document_index::IndexRequestBuilder;
pub use self::document_update::UpdateRequestBuilder;
pub use self::document_delete::DeleteRequestBuilder;
pub use self::document_put_mapping::PutMappingRequestBuilder;

// Index requests
pub mod index_create;
pub mod index_open;
pub mod index_close;
pub mod index_delete;
pub mod index_exists;
pub use self::index_create::IndexCreateRequestBuilder;
pub use self::index_open::IndexOpenRequestBuilder;
pub use self::index_close::IndexCloseRequestBuilder;
pub use self::index_delete::IndexDeleteRequestBuilder;
pub use self::index_exists::IndexExistsRequestBuilder;

// Misc requests
pub mod ping;
pub use self::ping::PingRequestBuilder;

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
    params_builder: Option<Arc<Fn(RequestParams) -> RequestParams>>,
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
    fn new(client: Client<TSender>, builder: Option<Arc<Fn(RequestParams) -> RequestParams>>, req: TRequest) -> Self {
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
                        .params(|p| p.url_param("refresh", true));
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
                        .params(|p| p.base_url("http://different-host:9200"));
    # Ok(())
    # }
    ```
    */
    pub fn params<F>(mut self, builder: F) -> Self
    where
        F: Fn(RequestParams) -> RequestParams + 'static,
    {
        if let Some(old_params_builder) = self.params_builder {
            let params_builder = move |params: RequestParams| {
                let params = old_params_builder(params);
                builder(params)
            };

            self.params_builder = Some(Arc::new(params_builder));
        } else {
            self.params_builder = Some(Arc::new(builder));
        }

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
    # extern crate tokio_core;
    # extern crate futures_cpupool;
    # extern crate elastic;
    # use futures_cpupool::CpuPool;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let pool = CpuPool::new(4);
    let builder = client.request(get_req())
                        .serde_pool(pool.clone());
    # Ok(())
    # }
    ```
    
    Never deserialise the response on a thread pool:
    
    ```no_run
    # extern crate tokio_core;
    # extern crate futures_cpupool;
    # extern crate elastic;
    # use futures_cpupool::CpuPool;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .serde_pool(None);
    # Ok(())
    # }
    ```
    */
    pub fn serde_pool<P>(mut self, pool: P) -> Self
    where
        P: Into<Option<CpuPool>>,
    {
        self.client.sender.serde_pool = pool.into();

        self
    }
}

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::params::*;
    pub use super::endpoints::*;

    pub use super::{
        empty_body,
        DefaultBody,
        DeleteRequestBuilder,
        GetRequestBuilder,
        IndexCreateRequestBuilder,
        IndexDeleteRequestBuilder,
        IndexOpenRequestBuilder,
        IndexCloseRequestBuilder,
        IndexRequestBuilder,
        PutMappingRequestBuilder,
        RawRequestBuilder,
        SearchRequestBuilder,
        PingRequestBuilder,
        UpdateRequestBuilder,
    };
}
