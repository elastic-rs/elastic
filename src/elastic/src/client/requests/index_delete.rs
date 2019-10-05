/*!
Builders for [delete index requests][docs-delete-index].

[docs-delete-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-delete-index.html
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
    endpoints::IndicesDeleteRequest,
    error::Error,
    http::sender::Sender,
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

impl RequestInner for IndexDeleteRequestInner {
    type Request = IndicesDeleteRequest<'static>;
    type Response = CommandResponse;
    
    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(IndicesDeleteRequest::for_index(self.index))
    }
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

#[cfg(test)]
mod tests {
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.index("testindex").delete().inner.into_request().unwrap();

        assert_eq!("/testindex", req.url.as_ref());
    }
}
