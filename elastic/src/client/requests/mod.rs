/*!
Request types for the Elasticsearch REST API.

This module contains implementation details that are useful if you want to customise the request process,
but aren't generally important for sending requests.
*/

use std::marker::PhantomData;
use futures::Future;
use elastic_reqwest::{SyncElasticClient, AsyncElasticClient};

use error::*;
use client::{Client, Sender, SyncSender, AsyncSender, RequestParams};
use client::responses::{sync_response, async_response, SyncResponseBuilder, AsyncResponseBuilder};

pub use elastic_reqwest::{SyncBody, AsyncBody};
pub use elastic_reqwest::req::{HttpRequest, HttpMethod, empty_body, Url, DefaultBody};
pub use elastic_reqwest::req::params;
pub use elastic_reqwest::req::endpoints;

pub use self::params::*;
pub use self::endpoints::*;

mod search;
pub use self::search::*;

mod get_document;
pub use self::get_document::*;

mod index_document;
pub use self::index_document::*;

mod put_mapping;
pub use self::put_mapping::*;

mod create_index;
pub use self::create_index::*;

/**
A builder for a raw request.

This structure wraps up a concrete REST API request type and lets you adjust parameters before sending it.
*/
pub struct RequestBuilder<TSender, TRequest> 
    where TSender: Sender
{
    client: Client<TSender>,
    params: Option<RequestParams>,
    req: TRequest,
}

/**
A builder for a raw [`Client.request`][Client.request]. 

[Client.request]: ../struct.Client.html#method.request
*/
pub struct RawRequestBuilder<TRequest, TBody> {
    inner: TRequest,
    _marker: PhantomData<TBody>
}

impl<TRequest, TBody> RawRequestBuilder<TRequest, TBody> {
    fn new(req: TRequest) -> Self {
        RawRequestBuilder {
            inner: req,
            _marker: PhantomData,
        }
    }
}

impl<TRequest, TBody> Into<HttpRequest<'static, TBody>> for RawRequestBuilder<TRequest, TBody>
    where TRequest: Into<HttpRequest<'static, TBody>>
{
    fn into(self) -> HttpRequest<'static, TBody> {
        self.inner.into()
    }
}

impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /**
    Create a [raw `RequestBuilder`][Client.raw_request] that can be configured before sending.
    
    The `request` method accepts any type that can be converted into a [`HttpRequest<'static>`][HttpRequest],
    which includes the endpoint types in the [`endpoints`][endpoints-mod] module.
    
    # Examples
    
    Send a cluster ping and read the returned metadata:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = ClientBuilder::new().build().unwrap();
    // `PingRequest` implements `Into<HttpRequest>`
    let req = PingRequest::new();
    
    // Turn the `PingRequest` into a `RequestBuilder`
    let builder = client.request(req);
    
    // Send the `RequestBuilder` and parse as a `PingResponse`
    let ping = builder.send().and_then(into_response::<PingResponse>).unwrap();

    println!("cluster: {}", ping.name);
    ```

    [HttpRequest]: requests/struct.HttpRequest.html
    [Client.raw_request]: requests/struct.RequestBuilder.html#raw-request-builder
    [endpoints-mod]: requests/endpoints/index.html
    */
    pub fn request<TRequest, TBody>(&self, req: TRequest) -> RequestBuilder<TSender, RawRequestBuilder<TRequest, TBody>>
        where TRequest: Into<HttpRequest<'static, TBody>>,
              TBody: Into<TSender::Body>
    {
        RequestBuilder::new(self.clone(), None, RawRequestBuilder::new(req))
    }
}

impl<TSender, TRequest> RequestBuilder<TSender, TRequest> 
    where TSender: Sender
{
    fn new(client: Client<TSender>, params: Option<RequestParams>, req: TRequest) -> Self {
        RequestBuilder {
            client: client,
            params: params,
            req: req,
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
    # let client = ClientBuilder::new().build().unwrap();
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let builder = client.request(get_req())
                        .params(|params| params.url_param("refresh", true));
    ```

    The `params` method is available for any request builder.
    */
    pub fn params<F>(mut self, builder: F) -> Self
        where F: Fn(RequestParams) -> RequestParams
    {
        self.params = Some(builder(self.params.unwrap_or(self.client.params.clone())));

        self
    }
}

/** 
# Raw request builder

A request builder for a [raw request type][endpoints-mod].

Call [`Client.request`][Client.request] to get a `RequestBuilder` for a raw request.

[Client.request]: ../struct.Client.html#method.request
[endpoints-mod]: endpoints/index.html
*/
impl<TRequest, TBody> RequestBuilder<SyncSender, RawRequestBuilder<TRequest, TBody>>
    where TRequest: Into<HttpRequest<'static, TBody>>, 
          TBody: Into<<SyncSender as Sender>::Body>
{
    /**
    Send this request and return the response.
    
    This method consumes the `RequestBuilder` and returns a [`ResponseBuilder`][ResponseBuilder] that can be used to parse the response.

    # Examples

    Send a raw request and parse it to a concrete response type:

    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    # let client = ClientBuilder::new().build().unwrap();
    let response = client.request(get_req())
                         .send()
                         .and_then(into_response::<SearchResponse<Value>>)
                         .unwrap();
    # }
    ```

    [ResponseBuilder]: ../responses/struct.ResponseBuilder.html
    */
    pub fn send(self) -> Result<SyncResponseBuilder> {
        let params = self.params.as_ref().unwrap_or(&self.client.params);

        let res = self.client.http.elastic_req(params, self.req)?;

        Ok(sync_response(res))
    }
}

/** 
# Raw request builder

A request builder for a [raw request type][endpoints-mod].

Call [`Client.request`][Client.request] to get a `RequestBuilder` for a raw request.

[Client.request]: ../struct.Client.html#method.request
[endpoints-mod]: endpoints/index.html
*/
impl<TRequest, TBody> RequestBuilder<AsyncSender, RawRequestBuilder<TRequest, TBody>>
    where TRequest: Into<HttpRequest<'static, TBody>>, 
          TBody: Into<<AsyncSender as Sender>::Body>
{
    /**
    Send this request and return the response.
    
    This method consumes the `RequestBuilder` and returns a [`ResponseBuilder`][ResponseBuilder] that can be used to parse the response.

    # Examples

    Send a raw request and parse it to a concrete response type:

    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    # let client = ClientBuilder::new().build().unwrap();
    let response = client.request(get_req())
                         .send()
                         .and_then(into_response::<SearchResponse<Value>>)
                         .unwrap();
    # }
    ```

    [ResponseBuilder]: ../responses/struct.ResponseBuilder.html
    */
    pub fn send(self) -> Box<Future<Item = AsyncResponseBuilder, Error = Error>> {
        let params = self.params.as_ref().unwrap_or(&self.client.params);

        let req = self.req;
        let http = self.client.http.inner;
        let de_pool = self.client.http.de_pool;

        let res_future = http
            .elastic_req(params, req)
            .map(|res| async_response(res, de_pool))
            .map_err(Into::into);
        
        Box::new(res_future)
    }
}

pub type SyncRequestBuilder<T> = RequestBuilder<SyncSender, T>;
pub type AsyncRequestBuilder<T> = RequestBuilder<AsyncSender, T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_builder_params() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = RequestBuilder::new(&client, None, PingRequest::new())
            .params(|p| p.url_param("pretty", true))
            .params(|p| p.url_param("refresh", true));

        let params = &req.params.unwrap();

        let (_, query) = params.get_url_qry();

        assert_eq!("http://eshost:9200", &params.base_url);
        assert_eq!("?pretty=true&refresh=true", query.unwrap());
    }
}
