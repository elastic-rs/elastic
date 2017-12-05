/*!
Request types for the Elasticsearch REST API.

This module contains implementation details that are useful if you want to customise the request process, but aren't generally important for sending requests.
*/

use futures_cpupool::CpuPool;
use fluent_builder::FluentBuilder;

use client::Client;
use client::sender::{AsyncSender, RequestParams, Sender};

pub use elastic_requests::{empty_body, DefaultBody, Endpoint, UrlPath};
pub use elastic_requests::params;
pub use elastic_requests::endpoints;

pub use self::params::*;
pub use self::endpoints::*;

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
pub use self::index_create::IndexCreateRequestBuilder;
pub use self::index_open::IndexOpenRequestBuilder;
pub use self::index_close::IndexCloseRequestBuilder;
pub use self::index_delete::IndexDeleteRequestBuilder;

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
    pub fn params_fluent<F>(mut self, builder: F) -> Self
    where
        F: Fn(RequestParams) -> RequestParams + 'static,
    {
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
    pub fn params<I>(mut self, params: I) -> Self
    where
        I: Into<RequestParams>
    {
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

    pub use super::{empty_body, DefaultBody, GetRequestBuilder, IndexCreateRequestBuilder, IndexRequestBuilder, PutMappingRequestBuilder, RawRequestBuilder, SearchRequestBuilder};
}
