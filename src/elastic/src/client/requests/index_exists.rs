/*!
Builders for [index exists requests][docs-index-exists].

[docs-index-exists]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-exists.html
*/

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::IndicesExistsResponse,
        IndexClient,
    },
    endpoints::IndicesExistsRequest,
    error::Error,
    http::sender::Sender,
    params::Index,
};

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

impl RequestInner for IndexExistsRequestInner {
    type Request = IndicesExistsRequest<'static>;
    type Response = IndicesExistsResponse;

    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(IndicesExistsRequest::for_index(self.index))
    }
}

/**
# Index exists request
*/
impl<TSender> IndexClient<TSender>
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
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index("myindex").exists().send()?;

    assert!(response.exists());
    # Ok(())
    # }
    ```

    [IndexExistsRequestBuilder]: requests/index_exists/type.IndexExistsRequestBuilder.html
    [builder-methods]: requests/index_exists/type.IndexExistsRequestBuilder.html#builder-methods
    [send-sync]: requests/index_exists/type.IndexExistsRequestBuilder.html#send-synchronously
    [send-async]: requests/index_exists/type.IndexExistsRequestBuilder.html#send-asynchronously
    */
    pub fn exists(self) -> IndexExistsRequestBuilder<TSender> {
        RequestBuilder::initial(self.inner, IndexExistsRequestInner { index: self.index })
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

        let req = client.index("testindex").exists().inner.into_request().unwrap();

        assert_eq!("/testindex", req.url.as_ref());
    }
}
