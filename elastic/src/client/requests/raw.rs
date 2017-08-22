use std::marker::PhantomData;

use client::{Client, Sender};
use client::requests::{HttpRequest, RequestBuilder};

/**
A raw request builder that can be configured before sending.

Call [`Client.request`][Client.request] to get an `IndexRequest`. 
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.request]: ../struct.Client.html#raw-request
*/
pub type RawRequestBuilder<TSender, TRequest, TBody> = RequestBuilder<TSender, RawRequestInner<TRequest, TBody>>;

#[doc(hidden)]
pub struct RawRequestInner<TRequest, TBody> {
    req: TRequest,
    _marker: PhantomData<TBody>
}

impl<TRequest, TBody> RawRequestInner<TRequest, TBody> {
    pub fn new(req: TRequest) -> Self {
        RawRequestInner {
            req: req,
            _marker: PhantomData,
        }
    }
}

/**
# Raw request
*/
impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /**
    Create a [`RawRequestBuilder`][RawRequestBuilder] with this `Client` that can be configured before sending.

    The `request` method accepts any type that can be converted into a [`HttpRequest<'static>`][HttpRequest],
    which includes the endpoint types in the [`endpoints`][endpoints-mod] module.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]
    
    # Examples
    
    Send a cluster ping and read the returned metadata:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = ClientBuilder::new().build()?;
    // `PingRequest` implements `Into<HttpRequest>`
    let req = PingRequest::new();
    
    // Turn the `PingRequest` into a `RequestBuilder`
    let builder = client.request(req);
    
    // Send the `RequestBuilder` and parse as a `PingResponse`
    let ping = builder.send().and_then(into_response::<PingResponse>)?;

    println!("cluster: {}", ping.name);
    ```

    [HttpRequest]: requests/struct.HttpRequest.html
    [RawRequestBuilder]: requests/type.RawRequestBuilder.html
    [builder-methods]: requests/type.RawRequestBuilder.html#builder-methods
    [send-sync]: requests/type.RawRequestBuilder.html#send-synchronously
    [send-async]: requests/type.RawRequestBuilder.html#send-asynchronously
    [endpoints-mod]: requests/endpoints/index.html
    */
    pub fn request<TRequest, TBody>(&self, req: TRequest) -> RawRequestBuilder<TSender, TRequest, TBody>
        where TRequest: Into<HttpRequest<'static, TBody>>,
              TBody: Into<TSender::Body>
    {
        RequestBuilder::new(self.clone(), None, RawRequestInner::new(req))
    }
}

impl<TSender, TRequest, TBody> RawRequestBuilder<TSender, TRequest, TBody>
    where TSender: Sender,
          TRequest: Into<HttpRequest<'static, TBody>>, 
          TBody: Into<<TSender>::Body>
{
    /**
    Send a `RawRequestBuilder`.

    If this request is for a [`SyncClient`](), then `send` will block the current thread until a response arrives and is deserialised.
    The returned [`SyncResponseBuilder`][SyncResponseBuilder] can be used to parse the response.

    If this request is for an [`AsyncClient`](), then `send` will return a future that will resolve to the deserialised index response.
    The returned [`AsyncHttpResponse`][AsyncHttpResponse] can be used to parse the response.

    # Examples

    Send a raw request synchronously and parse it to a concrete response type:

    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    let client = SyncClientBuilder::new().build()?;

    let response = client.request(get_req())
                         .send()?
                         .into_response::<SearchResponse<Value>>()?;
    # }
    ```

    Send a raw request asynchronously and parse it to a concrete response type:

    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    # let core = Core::new()?;
    # let handle = core.handle();
    let client = AsyncClientBuilder::new().build(handle)?;

    let response = client.request(get_req())
                         .send()
                         .and_then(|res| res.into_response::<SearchResponse<Value>>());
    # }
    ```

    [SyncResponseBuilder]: ../responses/struct.SyncResponseBuilder.html
    */
    pub fn send(self) -> TSender::Response {
        let client = self.client;
        let req = self.inner.req.into();
        let params = self.params.as_ref().unwrap_or(&client.params);

        client.sender.send(req, params)
    }
}
