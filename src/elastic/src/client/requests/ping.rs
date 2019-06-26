/*!
Builders for ping requests.
*/

use futures::{
    Future,
    Poll,
};

use client::{
    requests::{
        endpoints::PingRequest,
        raw::RawRequestInner,
        RequestBuilder,
    },
    responses::PingResponse,
    sender::{
        AsyncSender,
        Sender,
        SyncSender,
    },
    Client,
};
use error::{
    Error,
    Result,
};

/**
A ping request builder that can be configured before sending.

Call [`Client.ping`][Client.ping] to get a `PingRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.ping]: ../../struct.Client.html#ping-request
*/
pub type PingRequestBuilder<TSender> = RequestBuilder<TSender, PingRequestInner>;

#[doc(hidden)]
pub struct PingRequestInner;

/**
# Ping request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /**
    Create a [`PingRequestBuilder`][PingRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Ping an Elasticsearch node.

    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.ping().send()?;

    println!("node: {}", response.name());
    # Ok(())
    # }
    ```

    [PingRequestBuilder]: requests/ping/type.PingRequestBuilder.html
    [send-sync]: requests/ping/type.PingRequestBuilder.html#send-synchronously
    [send-async]: requests/ping/type.PingRequestBuilder.html#send-asynchronously
    */
    pub fn ping(&self) -> PingRequestBuilder<TSender> {
        RequestBuilder::initial(self.clone(), PingRequestInner)
    }
}

impl PingRequestInner {
    fn into_request(self) -> PingRequest<'static> {
        PingRequest::new()
    }
}

/**
# Send synchronously
*/
impl PingRequestBuilder<SyncSender> {
    /**
    Send a `PingRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Ping an Elasticsearch node:

    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.ping().send()?;

    println!("node: {}", response.name());
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<PingResponse> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl PingRequestBuilder<AsyncSender> {
    /**
    Send a `PingRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].

    This will return a future that will resolve to the deserialised ping response.

    # Examples

    Ping an Elasticsearch node:

    ```no_run
    # extern crate tokio;
    # extern crate futures;
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.ping().send();

    future.and_then(|response| {
        println!("node: {}", response.name());

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    */
    pub fn send(self) -> Pending {
        let req = self.inner.into_request();

        let res_future =
            RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response());

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<Future<Item = PingResponse, Error = Error> + Send>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = PingResponse, Error = Error> + Send + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = PingResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;
    use tests::*;

    #[test]
    fn is_send() {
        assert_send::<super::Pending>();
    }

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.ping().inner.into_request();

        assert_eq!("/", req.url.as_ref());
    }
}
