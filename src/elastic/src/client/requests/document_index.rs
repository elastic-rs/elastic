/*!
Builders for [index requests][docs-index].

[docs-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html
*/

use serde::Serialize;
use serde_json;

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::IndexResponse,
        DocumentClient,
    },
    endpoints::IndexRequest,
    error::{
        self,
        Error,
    },
    http::sender::Sender,
    params::{
        Id,
        Index,
        Type,
    },
    types::document::{
        DocumentType,
        DEFAULT_DOC_TYPE,
    },
};

/**
An [index request][docs-index] builder that can be configured before sending.

Call [`Client.document.index`][Client.document.index] to get an `IndexRequest`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document.index]: ../../struct.DocumentClient.html#index-document-request
*/
pub type IndexRequestBuilder<TSender, TDocument> =
    RequestBuilder<TSender, IndexRequestInner<TDocument>>;

#[doc(hidden)]
pub struct IndexRequestInner<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Option<Id<'static>>,
    doc: TDocument,
}

impl<TDocument> RequestInner for IndexRequestInner<TDocument>
where
    TDocument: Serialize,
{
    type Request = IndexRequest<'static, Vec<u8>>;
    type Response = IndexResponse;

    fn into_request(self) -> Result<Self::Request, Error> {
        let body = serde_json::to_vec(&self.doc).map_err(error::request)?;

        let request = match self.id {
            Some(id) => IndexRequest::for_index_ty_id(self.index, self.ty, id, body),
            None => IndexRequest::for_index_ty(self.index, self.ty, body),
        };

        Ok(request)
    }
}

/**
# Index document request
*/
impl<TSender, TDocument> DocumentClient<TSender, TDocument>
where
    TSender: Sender,
{
    /**
    Create a [`IndexRequestBuilder`][IndexRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Index a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    #[derive(Serialize, Deserialize, ElasticType)]
    struct MyType {
        #[elastic(id)]
         pub id: String,
         pub title: String,
         pub timestamp: Date<DefaultDateMapping>
    }

    let doc = MyType {
        id: "1".to_owned(),
        title: "A title".to_owned(),
        timestamp: Date::now()
    };

    let response = client.document()
                         .index(doc)
                         .send()?;

    assert!(response.created());
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [IndexRequestBuilder]: requests/document_index/type.IndexRequestBuilder.html
    [builder-methods]: requests/document_index/type.IndexRequestBuilder.html#builder-methods
    [send-sync]: requests/document_index/type.IndexRequestBuilder.html#send-synchronously
    [send-async]: requests/document_index/type.IndexRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn index(self, doc: TDocument) -> IndexRequestBuilder<TSender, TDocument>
    where
        TDocument: Serialize + DocumentType,
    {
        RequestBuilder::initial(
            self.inner,
            IndexRequestInner {
                index: doc.index().to_owned(),
                ty: doc.ty().to_owned(),
                id: doc.partial_id().map(|id| id.to_owned()),
                doc,
            },
        )
    }

    /**
    Create a [`IndexRequestBuilder`][IndexRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Index a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let doc_id = 123;
    let doc = json!({
        "id": doc_id,
        "title": "A document"
    });

    let response = client.document()
                         .index_raw("myindex", doc)
                         .id(doc_id)
                         .send()?;

    assert!(response.created());
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [IndexRequestBuilder]: requests/document_index/type.IndexRequestBuilder.html
    [builder-methods]: requests/document_index/type.IndexRequestBuilder.html#builder-methods
    [send-sync]: requests/document_index/type.IndexRequestBuilder.html#send-synchronously
    [send-async]: requests/document_index/type.IndexRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn index_raw(
        self,
        index: impl Into<Index<'static>>,
        doc: TDocument,
    ) -> IndexRequestBuilder<TSender, TDocument>
    where
        TDocument: Serialize,
    {
        RequestBuilder::initial(
            self.inner,
            IndexRequestInner {
                index: index.into(),
                ty: DEFAULT_DOC_TYPE.into(),
                id: None,
                doc,
            },
        )
    }
}

/**
# Builder methods

Configure a `IndexRequestBuilder` before sending it.
*/
impl<TSender, TDocument> IndexRequestBuilder<TSender, TDocument>
where
    TSender: Sender,
{
    /** Set the index for the index request. */
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.inner.index = index.into();
        self
    }

    /** Set the type for the index request. */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
        self.inner.ty = ty.into();
        self
    }

    /** Set the id for the index request. */
    pub fn id(mut self, id: impl Into<Id<'static>>) -> Self {
        self.inner.id = Some(id.into());
        self
    }
}

#[cfg(all(test, feature="sync_sender"))]
mod tests {
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };

    #[derive(Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .index(TestDoc {})
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/_doc", req.url.as_ref());
        assert_eq!(b"{}".to_vec(), req.body);
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .index(TestDoc {})
            .index("new-idx")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/new-idx/_doc", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .index(TestDoc {})
            .ty("new-ty")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/new-ty", req.url.as_ref());
    }

    #[test]
    fn specify_id() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .index(TestDoc {})
            .id(1)
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/_doc/1", req.url.as_ref());
    }
}
