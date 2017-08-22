/*!
Request types for the Elasticsearch REST API.

This module contains implementation details that are useful if you want to customise the request process,
but aren't generally important for sending requests.
*/

use futures_cpupool::CpuPool;

use client::{Client, Sender, AsyncSender, RequestParams};

pub use elastic_reqwest::{SyncBody, AsyncBody};
pub use elastic_reqwest::req::{HttpRequest, HttpMethod, empty_body, Url, DefaultBody};
pub use elastic_reqwest::req::params;
pub use elastic_reqwest::req::endpoints;

pub use self::params::*;
pub use self::endpoints::*;

mod raw;
pub use self::raw::RawRequestBuilder;

// Search requests
mod search;
pub use self::search::SearchRequestBuilder;

// Document requests
mod document_get;
mod document_index;
mod document_put_mapping;
pub use self::document_get::DocumentGetRequestBuilder;
pub use self::document_index::DocumentIndexRequestBuilder;
pub use self::document_put_mapping::DocumentPutMappingRequestBuilder;

// Index requests
mod index_create;
pub use self::index_create::IndexCreateRequestBuilder;

/**
A builder for a request.

This structure wraps up a concrete REST API request type and lets you adjust parameters before sending it.
The `RequestBuilder` has two generic parameters:

- `TSender`: the kind of request sender. This can be either synchronous or asynchronous
- `TRequest`: the inner request type, for example `SearchRequestBuilder`.

`RequestBuilder` contains methods that are common to all request builders.
*/
pub struct RequestBuilder<TSender, TRequest> 
    where TSender: Sender
{
    client: Client<TSender>,
    params: Option<RequestParams>,
    inner: TRequest,
}

/**
# Methods for any request builder

The following methods can be called on any request builder, whether it's synchronous or asynchronous.
*/
impl<TSender, TRequest> RequestBuilder<TSender, TRequest> 
    where TSender: Sender
{
    fn new(client: Client<TSender>, params: Option<RequestParams>, req: TRequest) -> Self {
        RequestBuilder {
            client: client,
            params: params,
            inner: req,
        }
    }

    /**
    Override the parameters for this request.
    
    This method will clone the `RequestParams` on the `Client` and pass
    them to the closure.
    
    # Examples
    
    Add a url param to force an index refresh:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = ClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .params(|params| params.url_param("refresh", true));
    ```
    */
    pub fn params<F>(mut self, builder: F) -> Self
        where F: Fn(RequestParams) -> RequestParams
    {
        self.params = Some(builder(self.params.unwrap_or(self.client.params.clone())));

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
    # use elastic::prelude::*;
    # let client = AsyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let pool = CpuPool::new(4)?;
    let builder = client.request(get_req())
                        .serde_pool(pool.clone());
    ```
    
    Never deserialise the response on a thread pool:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = AsyncClientBuilder::new().build()?;
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .serde_pool(None);
    ```
    */
    pub fn serde_pool<P>(mut self, pool: P) -> Self
        where P: Into<Option<CpuPool>>
    {
        self.client.sender.serde_pool = pool.into();

        self
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn request_builder_params() {
        let client = SyncClientBuilder::new().base_url("http://eshost:9200").build().unwrap();

        let req = RequestBuilder::new(client.clone(), None, PingRequest::new())
            .params(|p| p.url_param("pretty", true))
            .params(|p| p.url_param("refresh", true));

        let params = &req.params.unwrap();

        let (_, query) = params.get_url_qry();

        assert_eq!("http://eshost:9200", params.get_base_url());
        assert_eq!("?pretty=true&refresh=true", query.unwrap());
    }
}
