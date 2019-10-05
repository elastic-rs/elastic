/*!
Builders for ping requests.
*/

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::PingResponse,
        Client,
    },
    endpoints::PingRequest,
    error::Error,
    http::sender::Sender,
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

impl RequestInner for PingRequestInner {
    type Request = PingRequest<'static>;
    type Response = PingResponse;

    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(PingRequest::new())
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.ping().inner.into_request().unwrap();

        assert_eq!("/", req.url.as_ref());
    }
}
