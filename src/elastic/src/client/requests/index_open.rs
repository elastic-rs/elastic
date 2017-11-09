/*!
Builders for [open index requests][docs-open-index].

[docs-open-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-open-close.html
*/

use futures::{Future, Poll};

use error::*;
use client::Client;
use client::sender::{AsyncSender, Sender, SyncSender};
use client::requests::{empty_body, DefaultBody, RequestBuilder};
use client::requests::params::Index;
use client::requests::endpoints::IndicesOpenRequest;
use client::requests::raw::RawRequestInner;
use client::responses::CommandResponse;

/** 
An [open index request][docs-open-index] builder that can be configured before sending. 

Call [`Client.index_open`][Client.index_open] to get an `IndexOpenRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was opend from.

[docs-open-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-open-close.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.index_open]: ../../struct.Client.html#open-index-request
*/
pub type IndexOpenRequestBuilder<TSender> = RequestBuilder<TSender, IndexOpenRequestInner>;

#[doc(hidden)]
pub struct IndexOpenRequestInner {
    index: Index<'static>,
}

/**
# Open index request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /** 
    Open an [`IndexOpenRequestBuilder`][IndexOpenRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [send synchronously][send-sync]
    - [send asynchronously][send-async]
    
    # Examples
    
    Open an index called `myindex`:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let my_index = index("myindex");

    let response = client.index_open(my_index).send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [IndexOpenRequestBuilder]: requests/index_open/type.IndexOpenRequestBuilder.html
    [builder-methods]: requests/index_open/type.IndexOpenRequestBuilder.html#builder-methods
    [send-sync]: requests/index_open/type.IndexOpenRequestBuilder.html#send-synchronously
    [send-async]: requests/index_open/type.IndexOpenRequestBuilder.html#send-asynchronously
    */
    pub fn index_open(&self, index: Index<'static>) -> IndexOpenRequestBuilder<TSender> {
        RequestBuilder::new(self.clone(), None, IndexOpenRequestInner { index: index })
    }
}

impl IndexOpenRequestInner {
    fn into_request(self) -> IndicesOpenRequest<'static, DefaultBody> {
        IndicesOpenRequest::for_index(self.index, empty_body())
    }
}

/**
# Send synchronously
*/
impl IndexOpenRequestBuilder<SyncSender> {
    /**
    Send an `IndexOpenRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples
    
    Open an index called `myindex`:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let my_index = index("myindex");

    let response = client.index_open(my_index).send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl IndexOpenRequestBuilder<AsyncSender> {
    /**
    Send an `IndexOpenRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
    This will return a future that will resolve to the deserialised command response.

    # Examples
    
    Open an index called `myindex`:
    
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
    let my_index = index("myindex");

    let future = client.index_open(my_index).send();

    future.and_then(|response| {
        assert!(response.acknowledged());

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
    inner: Box<Future<Item = CommandResponse, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = CommandResponse, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = CommandResponse;
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

        let req = client.index_open(index("testindex")).inner.into_request();

        assert_eq!("/testindex/_open", req.url.as_ref());
    }
}
