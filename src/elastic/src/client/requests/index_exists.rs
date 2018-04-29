/*!
Builders for [index exists requests][docs-index-exists].

[docs-index-exists]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-exists.html
*/

use futures::{Future, Poll};

use error::*;
use client::Client;
use client::sender::{AsyncSender, Sender, SyncSender};
use client::requests::RequestBuilder;
use client::requests::params::Index;
use client::requests::endpoints::IndicesExistsRequest;
use client::requests::raw::RawRequestInner;
use client::responses::IndicesExistsResponse;

/** 
An [index exists request][docs-index-exists] builder that can be configured before sending. 

Call [`Client.index_exists`][Client.index_exists] to get an `IndexExistsRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was opend from.

[docs-index-exists]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-exists.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.index_exists]: ../../struct.Client.html#index-exists-request
*/
pub type IndexExistsRequestBuilder<TSender> = RequestBuilder<TSender, IndexExistsRequestInner>;

#[doc(hidden)]
pub struct IndexExistsRequestInner {
    index: Index<'static>,
}

/**
# Index exists request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /** 
    Open an [`IndexExistsRequestBuilder`][IndexExistsRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [send synchronously][send-sync]
    - [send asynchronously][send-async]
    
    # Examples
    
    Check whether an index called `myindex` exists:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index_exists(index("myindex")).send()?;

    assert!(response.exists());
    # Ok(())
    # }
    ```

    [IndexExistsRequestBuilder]: requests/index_exists/type.IndexExistsRequestBuilder.html
    [builder-methods]: requests/index_exists/type.IndexExistsRequestBuilder.html#builder-methods
    [send-sync]: requests/index_exists/type.IndexExistsRequestBuilder.html#send-synchronously
    [send-async]: requests/index_exists/type.IndexExistsRequestBuilder.html#send-asynchronously
    */
    pub fn index_exists(&self, index: Index<'static>) -> IndexExistsRequestBuilder<TSender> {
        RequestBuilder::initial(self.clone(), IndexExistsRequestInner { index: index })
    }
}

impl IndexExistsRequestInner {
    fn into_request(self) -> IndicesExistsRequest<'static> {
        IndicesExistsRequest::for_index(self.index)
    }
}

/**
# Send synchronously
*/
impl IndexExistsRequestBuilder<SyncSender> {
    /**
    Send an `IndexExistsRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples
    
    Check whether an index called `myindex` exists:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index_exists(index("myindex")).send()?;

    assert!(response.exists());
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<IndicesExistsResponse> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl IndexExistsRequestBuilder<AsyncSender> {
    /**
    Send an `IndexExistsRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
    This will return a future that will resolve to the deserialised command response.

    # Examples
    
    Check whether an index called `myindex` exists:
    
    ```no_run
    # extern crate futures;
    # extern crate tokio_core;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    let future = client.index_exists(index("myindex")).send();

    future.and_then(|response| {
        assert!(response.exists());

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    */
    pub fn send(self) -> Pending {
        let req = self.inner.into_request();

        let res_future = RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()
            .and_then(|res| res.into_response());

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<Future<Item = IndicesExistsResponse, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = IndicesExistsResponse, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = IndicesExistsResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.index_exists(index("testindex")).inner.into_request();

        assert_eq!("/testindex", req.url.as_ref());
    }
}
