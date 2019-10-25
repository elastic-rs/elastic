/*!
Builders for ping requests.
*/

use futures::Future;

use crate::{
    client::{
        requests::{
            raw::RawRequestInner,
            Pending as BasePending,
            RequestBuilder,
        },
        responses::PingResponse,
        Client,
    },
    endpoints::PingRequest,
    error::Error,
    http::sender::{
        AsyncSender,
        Sender,
        SyncSender,
    },
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
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
    pub fn send(self) -> Result<PingResponse, Error> {
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
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
pub type Pending = BasePending<PingResponse>;

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        tests::*,
    };

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
