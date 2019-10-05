/*!
Builders for [open index requests][docs-open-index].

[docs-open-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-open-close.html
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
    endpoints::IndicesOpenRequest,
    error::Error,
    http::{
        empty_body,
        sender::Sender,
        DefaultBody,
    },
    params::Index,
};

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

impl RequestInner for IndexOpenRequestInner {
    type Request = IndicesOpenRequest<'static, DefaultBody>;
    type Response = CommandResponse;
    
    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(IndicesOpenRequest::for_index(self.index, empty_body()))
    }
}

/**
# Open index request
*/
impl<TSender> IndexClient<TSender>
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
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index("myindex").open().send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [IndexOpenRequestBuilder]: requests/index_open/type.IndexOpenRequestBuilder.html
    [builder-methods]: requests/index_open/type.IndexOpenRequestBuilder.html#builder-methods
    [send-sync]: requests/index_open/type.IndexOpenRequestBuilder.html#send-synchronously
    [send-async]: requests/index_open/type.IndexOpenRequestBuilder.html#send-asynchronously
    */
    pub fn open(self) -> IndexOpenRequestBuilder<TSender> {
        RequestBuilder::initial(self.inner, IndexOpenRequestInner { index: self.index })
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

        let req = client.index("testindex").open().inner.into_request().unwrap();

        assert_eq!("/testindex/_open", req.url.as_ref());
    }
}
