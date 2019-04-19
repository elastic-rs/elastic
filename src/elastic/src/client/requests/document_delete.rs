/*!
Builders for [delete document requests][docs-delete].

[docs-delete]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-delete.html
*/

use futures::{
    Future,
    Poll,
};
use std::marker::PhantomData;

use client::requests::endpoints::DeleteRequest;
use client::requests::params::{
    Id,
    Index,
    Type,
};
use client::requests::raw::RawRequestInner;
use client::requests::RequestBuilder;
use client::responses::DeleteResponse;
use client::sender::{
    AsyncSender,
    Sender,
    SyncSender,
};
use client::DocumentClient;
use error::{
    Error,
    Result,
};
use types::document::DEFAULT_DOC_TYPE;
use types::document::{
    DocumentType,
    StaticIndex,
    StaticType,
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
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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

impl<TDocument> DeleteRequestInner<TDocument> {
    fn into_request(self) -> DeleteRequest<'static> {
        DeleteRequest::for_index_ty_id(self.index, self.ty, self.id)
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

/**
# Send synchronously
*/
impl<TDocument> DeleteRequestBuilder<SyncSender, TDocument> {
    /**
    Send a `DeleteRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Delete a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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

    [SyncClient]: ../../type.SyncClient.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn send(self) -> Result<DeleteResponse> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TDocument> DeleteRequestBuilder<AsyncSender, TDocument> {
    /**
    Send a `DeleteRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].

    This will return a future that will resolve to the deserialised delete document response.

    # Examples

    Delete a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # extern crate futures;
    # extern crate tokio;
    # extern crate serde;
    # extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use serde_json::Value;
    # use futures::Future;
    # use elastic::prelude::*;
    # #[derive(ElasticType)]
    # struct MyType { }
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.document::<MyType>()
                       .delete(1)
                       .ty("mytype")
                       .send();

    future.and_then(|response| {
        assert!(response.deleted());

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    [documents-mod]: ../types/document/index.html
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
    inner: Box<Future<Item = DeleteResponse, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = DeleteResponse, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = DeleteResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[derive(ElasticType)]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .delete("1")
            .inner
            .into_request();

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
            .into_request();

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
            .into_request();

        assert_eq!("/testdoc/new-ty/1", req.url.as_ref());
    }
}
