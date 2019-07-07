/*!
Builders for [close index requests][docs-close-index].

[docs-close-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-open-close.html
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
    endpoints::IndicesCloseRequest,
    error::Error,
    http::{
        empty_body,
        sender::{
            AsyncSender,
            Sender,
            SyncSender,
        },
        DefaultBody,
    },
    params::Index,
};

/**
A [close index request][docs-close-index] builder that can be configured before sending.

Call [`Client.index_close`][Client.index_close] to get an `IndexCloseRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was closed from.

[docs-close-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-open-close.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.index_close]: ../../struct.Client.html#close-index-request
*/
pub type IndexCloseRequestBuilder<TSender> = RequestBuilder<TSender, IndexCloseRequestInner>;

#[doc(hidden)]
pub struct IndexCloseRequestInner {
    index: Index<'static>,
}

/**
# Close index request
*/
impl<TSender> IndexClient<TSender>
where
    TSender: Sender,
{
    /**
    Create an [`IndexCloseRequestBuilder`][IndexCloseRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Close an index called `myindex`:

    ```no_run
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index("myindex").close().send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [IndexCloseRequestBuilder]: requests/index_close/type.IndexCloseRequestBuilder.html
    [builder-methods]: requests/index_close/type.IndexCloseRequestBuilder.html#builder-methods
    [send-sync]: requests/index_close/type.IndexCloseRequestBuilder.html#send-synchronously
    [send-async]: requests/index_close/type.IndexCloseRequestBuilder.html#send-asynchronously
    */
    pub fn close(self) -> IndexCloseRequestBuilder<TSender> {
        RequestBuilder::initial(self.inner, IndexCloseRequestInner { index: self.index })
    }
}

impl IndexCloseRequestInner {
    fn into_request(self) -> IndicesCloseRequest<'static, DefaultBody> {
        IndicesCloseRequest::for_index(self.index, empty_body())
    }
}

/**
# Send synchronously
*/
impl IndexCloseRequestBuilder<SyncSender> {
    /**
    Send an `IndexCloseRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Close an index called `myindex`:

    ```no_run
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index("myindex").close().send()?;

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
impl IndexCloseRequestBuilder<AsyncSender> {
    /**
    Send an `IndexCloseRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].

    This will return a future that will resolve to the deserialised command response.

    # Examples

    Close an index called `myindex`:

    ```no_run
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.index("myindex").close().send();

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

        let req = client.index("testindex").close().inner.into_request();

        assert_eq!("/testindex/_close", req.url.as_ref());
    }
}
