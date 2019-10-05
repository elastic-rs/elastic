/*!
Builders for [get document requests][docs-get].

[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
*/

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::GetResponse,
        DocumentClient,
    },
    endpoints::GetRequest,
    error::Error,
    http::sender::Sender,
    params::{
        Id,
        Index,
        Type,
    },
    types::document::{
        DocumentType,
        StaticIndex,
        StaticType,
        DEFAULT_DOC_TYPE,
    },
};

/**
A [get document request][docs-get] builder that can be configured before sending.

Call [`Client.document.get`][Client.document.get] to get a `GetRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document.get]: ../../struct.DocumentClient.html#get-document-request
*/
pub type GetRequestBuilder<TSender, TDocument> =
    RequestBuilder<TSender, GetRequestInner<TDocument>>;

#[doc(hidden)]
pub struct GetRequestInner<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    _marker: PhantomData<TDocument>,
}

impl<TDocument> RequestInner for GetRequestInner<TDocument>
where
    TDocument: DeserializeOwned + Send + 'static,
{
    type Request = GetRequest<'static>;
    type Response = GetResponse<TDocument>;

    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(GetRequest::for_index_ty_id(self.index, self.ty, self.id))
    }
}

/**
# Get document request
*/
impl<TSender, TDocument> DocumentClient<TSender, TDocument>
where
    TSender: Sender,
{
    /**
    Create a [`GetRequestBuilder`][GetRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Get a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # #[macro_use] extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<MyType>()
                         .get(1)
                         .send()?;
    # Ok(())
    # }
    ```

    For more details on document types, see the [`types`][types-mod] module.

    [GetRequestBuilder]: requests/document_get/type.GetRequestBuilder.html
    [builder-methods]: requests/document_get/type.GetRequestBuilder.html#builder-methods
    [send-sync]: requests/document_get/type.GetRequestBuilder.html#send-synchronously
    [send-async]: requests/document_get/type.GetRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn get(self, id: impl Into<Id<'static>>) -> GetRequestBuilder<TSender, TDocument>
    where
        TDocument: DeserializeOwned + DocumentType + StaticIndex + StaticType,
    {
        let index = TDocument::static_index().into();
        let ty = TDocument::static_ty().into();

        RequestBuilder::initial(
            self.inner,
            GetRequestInner {
                index: index,
                ty: ty,
                id: id.into(),
                _marker: PhantomData,
            },
        )
    }

    /**
    Create a [`GetRequestBuilder`][GetRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Get a document as a `serde_json::Value`:

    ```no_run
    # #[macro_use] extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<Value>()
                         .get_raw("myindex", 1)
                         .send()?;
    # Ok(())
    # }
    ```

    [GetRequestBuilder]: requests/document_get/type.GetRequestBuilder.html
    [builder-methods]: requests/document_get/type.GetRequestBuilder.html#builder-methods
    [send-sync]: requests/document_get/type.GetRequestBuilder.html#send-synchronously
    [send-async]: requests/document_get/type.GetRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn get_raw(
        self,
        index: impl Into<Index<'static>>,
        id: impl Into<Id<'static>>,
    ) -> GetRequestBuilder<TSender, TDocument>
    where
        TDocument: DeserializeOwned,
    {
        RequestBuilder::initial(
            self.inner,
            GetRequestInner {
                index: index.into(),
                ty: DEFAULT_DOC_TYPE.into(),
                id: id.into(),
                _marker: PhantomData,
            },
        )
    }
}

/**
# Builder methods

Configure a `GetRequestBuilder` before sending it.
*/
impl<TSender, TDocument> GetRequestBuilder<TSender, TDocument>
where
    TSender: Sender,
{
    /** Set the index for the get request. */
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.inner.index = index.into();
        self
    }

    /** Set the type for the get request. */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
        self.inner.ty = ty.into();
        self
    }
}

#[cfg(all(test, feature="sync_sender"))]
mod tests {
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };

    #[derive(Deserialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.document::<TestDoc>().get("1").inner.into_request().unwrap();

        assert_eq!("/testdoc/_doc/1", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .get("1")
            .index("new-idx")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/new-idx/_doc/1", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .get("1")
            .ty("new-ty")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/new-ty/1", req.url.as_ref());
    }
}
