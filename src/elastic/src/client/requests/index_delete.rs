/*!
Builders for [delete index requests][docs-delete-index].

[docs-delete-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-delete-index.html
*/

use futures::{
    Future,
    Poll,
};

use crate::{
    client::{
        requests::{
            raw::RawRequestInner,
            RequestBuilder,
        },
        responses::CommandResponse,
        IndexClient,
    },
    endpoints::IndicesDeleteRequest,
    error::Error,
    http::sender::{
        AsyncSender,
        Sender,
        SyncSender,
    },
    params::Index,
};

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
impl<TSender> IndexClient<TSender>
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
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index("myindex").delete().send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [IndexDeleteRequestBuilder]: requests/index_delete/type.IndexDeleteRequestBuilder.html
    [builder-methods]: requests/index_delete/type.IndexDeleteRequestBuilder.html#builder-methods
    [send-sync]: requests/index_delete/type.IndexDeleteRequestBuilder.html#send-synchronously
    [send-async]: requests/index_delete/type.IndexDeleteRequestBuilder.html#send-asynchronously
    */
    pub fn delete(self) -> IndexDeleteRequestBuilder<TSender> {
        RequestBuilder::initial(self.inner, IndexDeleteRequestInner { index: self.index })
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
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index("myindex").delete().send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<CommandResponse, Error> {
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
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.index("myindex").delete().send();

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

        let res_future =
            RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response());

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<dyn Future<Item = CommandResponse, Error = Error> + Send>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = CommandResponse, Error = Error> + Send + 'static,
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

        let req = client.index("testindex").delete().inner.into_request();

        assert_eq!("/testindex", req.url.as_ref());
    }
}
