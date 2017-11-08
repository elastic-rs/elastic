/*!
Builders for [delete index requests][docs-delete-index].

[docs-delete-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-delete-index.html
*/

use futures::{Future, Poll};

use error::*;
use client::Client;
use client::sender::{AsyncSender, Sender, SyncSender};
use client::requests::RequestBuilder;
use client::requests::params::Index;
use client::requests::endpoints::IndicesDeleteRequest;
use client::requests::raw::RawRequestInner;
use client::responses::CommandResponse;

/** 
A [delete index request][docs-delete-index] builder that can be configured before sending. 

Call [`Client.index_delete`][Client.index_delete] to get an `IndexDeleteRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was deleted from.

[docs-delete-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-delete-index.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.index_delete]: ../../struct.Client.html#delete-index-request
*/
pub type IndexDeleteRequestBuilder<TSender> = RequestBuilder<TSender, IndexDeleteRequestInner>;

#[doc(hidden)]
pub struct IndexDeleteRequestInner {
    index: Index<'static>,
}

/**
# Delete index request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /** 
    Delete a [`IndexDeleteRequestBuilder`][IndexDeleteRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [send synchronously][send-sync]
    - [send asynchronously][send-async]
    
    # Examples
    
    Delete an index called `myindex`:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let my_index = index("myindex");

    let response = client.index_delete(my_index).send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [IndexDeleteRequestBuilder]: requests/index_delete/type.IndexDeleteRequestBuilder.html
    [builder-methods]: requests/index_delete/type.IndexDeleteRequestBuilder.html#builder-methods
    [send-sync]: requests/index_delete/type.IndexDeleteRequestBuilder.html#send-synchronously
    [send-async]: requests/index_delete/type.IndexDeleteRequestBuilder.html#send-asynchronously
    */
    pub fn index_delete(&self, index: Index<'static>) -> IndexDeleteRequestBuilder<TSender> {
        RequestBuilder::new(self.clone(), None, IndexDeleteRequestInner { index: index })
    }
}

impl IndexDeleteRequestInner {
    fn into_request(self) -> IndicesDeleteRequest<'static> {
        IndicesDeleteRequest::for_index(self.index)
    }
}

/**
# Send synchronously
*/
impl IndexDeleteRequestBuilder<SyncSender> {
    /**
    Send a `IndexDeleteRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples
    
    Delete an index called `myindex`:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let my_index = index("myindex");

    let response = client.index_delete(my_index).send()?;

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
impl IndexDeleteRequestBuilder<AsyncSender> {
    /**
    Send a `IndexDeleteRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
    This will return a future that will resolve to the deserialised command response.

    # Examples
    
    Delete an index called `myindex`:
    
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

    let future = client.index_delete(my_index).send();

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

        let req = client.index_delete(index("testindex")).inner.into_request();

        assert_eq!("/testindex", req.url.as_ref());
    }
}
