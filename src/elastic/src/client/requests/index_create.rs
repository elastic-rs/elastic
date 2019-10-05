/*!
Builders for [create index requests][docs-create-index].

[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
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
    endpoints::IndicesCreateRequest,
    error::Error,
    http::{
        empty_body,
        sender::Sender,
        DefaultBody,
    },
    params::Index,
};

/**
A [create index request][docs-create-index] builder that can be configured before sending.

Call [`Client.index_create`][Client.index_create] to get an `IndexCreateRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.index_create]: ../../struct.Client.html#create-index-request
*/
pub type IndexCreateRequestBuilder<TSender, TBody> =
    RequestBuilder<TSender, IndexCreateRequestInner<TBody>>;

#[doc(hidden)]
pub struct IndexCreateRequestInner<TBody> {
    index: Index<'static>,
    body: TBody,
}

impl<TBody> RequestInner for IndexCreateRequestInner<TBody>
where
    TBody: Send + 'static
{
    type Request = IndicesCreateRequest<'static, TBody>;
    type Response = CommandResponse;

    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(IndicesCreateRequest::for_index(self.index, self.body))
    }
}

/**
# Create index request
*/
impl<TSender> IndexClient<TSender>
where
    TSender: Sender,
{
    /**
    Create an [`IndexCreateRequestBuilder`][IndexCreateRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Create an index called `myindex`:

    ```no_run
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let my_index = index("myindex");

    let response = client.index(my_index).create().send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    Create an index with settings and document mappings for a [`DocumentType`][documents-mod] called `MyType`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # use elastic::prelude::*;
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let body = json!({
        "settings": {
            "index": {
                "number_of_shards": 3,
                "number_of_replicas": 2
            }
        },
        "mappings": {
            MyType::static_index(): MyType::index_mapping()
        }
    });

    let response = client.index("myindex")
                         .create()
                         .body(body.to_string())
                         .send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [IndexCreateRequestBuilder]: requests/index_create/type.IndexCreateRequestBuilder.html
    [builder-methods]: requests/index_create/type.IndexCreateRequestBuilder.html#builder-methods
    [send-sync]: requests/index_create/type.IndexCreateRequestBuilder.html#send-synchronously
    [send-async]: requests/index_create/type.IndexCreateRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn create(self) -> IndexCreateRequestBuilder<TSender, DefaultBody> {
        RequestBuilder::initial(
            self.inner,
            IndexCreateRequestInner {
                index: self.index,
                body: empty_body(),
            },
        )
    }
}

/**
# Builder methods

Configure an `IndexCreateRequestBuilder` before sending it.
*/
impl<TSender, TBody> IndexCreateRequestBuilder<TSender, TBody>
where
    TSender: Sender,
    TBody: Into<TSender::Body>,
{
    /**
    Set the body for the create index request.

    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self, body: TNewBody) -> IndexCreateRequestBuilder<TSender, TNewBody>
    where
        TNewBody: Into<TSender::Body>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            IndexCreateRequestInner {
                index: self.inner.index,
                body: body,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        client::requests::RequestInner,
    };

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.index("testindex").create().inner.into_request().unwrap();

        assert_eq!("/testindex", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .index("testindex")
            .create()
            .body("{}")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("{}", req.body);
    }
}
