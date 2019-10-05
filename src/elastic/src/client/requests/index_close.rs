/*!
Builders for [close index requests][docs-close-index].

[docs-close-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-open-close.html
*/

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::CommandResponse,
        IndexClient,
    },
    endpoints::IndicesCloseRequest,
    error::Error,
    http::{
        empty_body,
        sender::Sender,
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

impl RequestInner for IndexCloseRequestInner {
    type Request = IndicesCloseRequest<'static, DefaultBody>;
    type Response = CommandResponse;

    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(IndicesCloseRequest::for_index(self.index, empty_body()))
    }
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

#[cfg(all(test, feature="sync_sender"))]
mod tests {
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.index("testindex").close().inner.into_request().unwrap();

        assert_eq!("/testindex/_close", req.url.as_ref());
    }
}
