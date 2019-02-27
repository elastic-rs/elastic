/*!
Builders for [get document requests][docs-get].

[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
*/

use futures::{
    Future,
    Poll,
};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use client::requests::endpoints::GetRequest;
use client::requests::params::{
    Id,
    Index,
    Type,
};
use client::requests::raw::RawRequestInner;
use client::requests::RequestBuilder;
use client::responses::GetResponse;
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
use types::document::{
    DocumentType,
    StaticIndex,
    StaticType,
};
use types::document::DEFAULT_DOC_TYPE;

/**
A [get document request][docs-get] builder that can be configured before sending.

Call [`Client.document_get`][Client.document_get] to get a `GetRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document_get]: ../../struct.Client.html#get-document
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
    # extern crate serde;
    # extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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
    # extern crate serde;
    # extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
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

impl<TDocument> GetRequestInner<TDocument> {
    fn into_request(self) -> GetRequest<'static> {
        GetRequest::for_index_ty_id(self.index, self.ty, self.id)
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

/**
# Send synchronously
*/
impl<TDocument> GetRequestBuilder<SyncSender, TDocument>
where
    TDocument: DeserializeOwned,
{
    /**
    Send a `GetRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Get a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

    ```no_run
    # extern crate serde;
    # extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # #[derive(Debug, ElasticType, Deserialize)]
    # struct MyType { }
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<MyType>()
                         .get(1)
                         .send()?;

    if let Some(doc) = response.into_document() {
        println!("{:?}", doc);
    }
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn send(self) -> Result<GetResponse<TDocument>> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TDocument> GetRequestBuilder<AsyncSender, TDocument>
where
    TDocument: DeserializeOwned + Send + 'static,
{
    /**
    Send a `GetRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].

    This will return a future that will resolve to the deserialised get document response.

    # Examples

    Get a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:

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
    # #[derive(Debug, ElasticType, Deserialize)]
    # struct MyType { }
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.document::<MyType>()
                       .get(1)
                       .send();

    future.and_then(|response| {
        if let Some(doc) = response.into_document() {
            println!("{:?}", doc);
        }

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn send(self) -> Pending<TDocument> {
        let req = self.inner.into_request();

        let res_future =
            RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response());

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending<TDocument> {
    inner: Box<Future<Item = GetResponse<TDocument>, Error = Error>>,
}

impl<TDocument> Pending<TDocument> {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = GetResponse<TDocument>, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl<TDocument> Future for Pending<TDocument>
where
    TDocument: DeserializeOwned + Send + 'static,
{
    type Item = GetResponse<TDocument>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[derive(Deserialize, ElasticType)]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.document::<TestDoc>().get("1").inner.into_request();

        assert_eq!("/testdoc/doc/1", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .get("1")
            .index("new-idx")
            .inner
            .into_request();

        assert_eq!("/new-idx/doc/1", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .get("1")
            .ty("new-ty")
            .inner
            .into_request();

        assert_eq!("/testdoc/new-ty/1", req.url.as_ref());
    }
}
