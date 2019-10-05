/*!
Builders for [delete document requests][docs-delete].

[docs-delete]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete.html
*/

use std::marker::PhantomData;

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::DeleteResponse,
        DocumentClient,
    },
    endpoints::DeleteRequest,
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
A [delete document request][docs-delete] builder that can be configured before sending.

Call [`Client.document.delete`][Client.document.delete] to get a `DeleteRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-delete]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document.delete]: ../../struct.DocumentClient.html#delete-document-request
*/
pub type DeleteRequestBuilder<TSender, TDocument> =
    RequestBuilder<TSender, DeleteRequestInner<TDocument>>;

#[doc(hidden)]
pub struct DeleteRequestInner<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    _marker: PhantomData<TDocument>,
}

impl<TDocument> RequestInner for DeleteRequestInner<TDocument> {
    type Request = DeleteRequest<'static>;
    type Response = DeleteResponse;

    fn into_request(self) -> Result<Self::Request, Error> {
        Ok(DeleteRequest::for_index_ty_id(self.index, self.ty, self.id))
    }
}

/**
# Delete document request
*/
impl<TSender, TDocument> DocumentClient<TSender, TDocument>
where
    TSender: Sender,
{
    /**
    Create a [`DeleteRequestBuilder`][DeleteRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Delete a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<MyType>()
                         .delete(1)
                         .send()?;

    assert!(response.deleted());
    # Ok(())
    # }
    ```

    [DeleteRequestBuilder]: requests/document_delete/type.DeleteRequestBuilder.html
    [builder-methods]: requests/document_delete/type.DeleteRequestBuilder.html#builder-methods
    [send-sync]: requests/document_delete/type.DeleteRequestBuilder.html#send-synchronously
    [send-async]: requests/document_delete/type.DeleteRequestBuilder.html#send-asynchronously
    [documents-mod]: ../types/document/index.html
    */
    pub fn delete(self, id: impl Into<Id<'static>>) -> DeleteRequestBuilder<TSender, TDocument>
    where
        TDocument: DocumentType + StaticIndex + StaticType,
    {
        let index = TDocument::static_index().into();
        let ty = TDocument::static_ty().into();

        RequestBuilder::initial(
            self.inner,
            DeleteRequestInner {
                index: index,
                ty: ty,
                id: id.into(),
                _marker: PhantomData,
            },
        )
    }
}

impl<TSender> DocumentClient<TSender, ()>
where
    TSender: Sender,
{
    /**
    Create a [`DeleteRequestBuilder`][DeleteRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Delete a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document()
                         .delete_raw("myindex", 1)
                         .send()?;

    assert!(response.deleted());
    # Ok(())
    # }
    ```

    [DeleteRequestBuilder]: requests/document_delete/type.DeleteRequestBuilder.html
    [builder-methods]: requests/document_delete/type.DeleteRequestBuilder.html#builder-methods
    [send-sync]: requests/document_delete/type.DeleteRequestBuilder.html#send-synchronously
    [send-async]: requests/document_delete/type.DeleteRequestBuilder.html#send-asynchronously
    [documents-mod]: ../types/document/index.html
    */
    pub fn delete_raw(
        self,
        index: impl Into<Index<'static>>,
        id: impl Into<Id<'static>>,
    ) -> DeleteRequestBuilder<TSender, ()> {
        RequestBuilder::initial(
            self.inner,
            DeleteRequestInner {
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

Configure a `DeleteRequestBuilder` before sending it.
*/
impl<TSender, TDocument> DeleteRequestBuilder<TSender, TDocument>
where
    TSender: Sender,
{
    /** Set the index for the delete request. */
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.inner.index = index.into();
        self
    }

    /** Set the type for the delete request. */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
        self.inner.ty = ty.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };

    #[derive(ElasticType)]
    #[elastic(crate_root = "crate::types")]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .delete("1")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/_doc/1", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .delete("1")
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
            .delete("1")
            .ty("new-ty")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/new-ty/1", req.url.as_ref());
    }
}
