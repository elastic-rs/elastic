/*!
Builders for [get document requests][docs-get].

[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
*/

use std::marker::PhantomData;
use futures::{Future, Poll};
use serde::de::DeserializeOwned;

use error::{Error, Result};
use client::{AsyncSender, Client, Sender, SyncSender};
use client::requests::RequestBuilder;
use client::requests::params::{Id, Index, Type};
use client::requests::endpoints::GetRequest;
use client::requests::raw::RawRequestInner;
use client::responses::GetResponse;
use types::document::DocumentType;

/** 
A [get document request][docs-get] builder that can be configured before sending.

Call [`Client.document_get`][Client.document_get] to get a `GetRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document_get]: ../../struct.Client.html#get-document
*/
pub type GetRequestBuilder<TSender, TDocument> = RequestBuilder<TSender, GetRequestInner<TDocument>>;

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
impl<TSender> Client<TSender>
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
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document_get::<MyType>(index("myindex"), id(1))
                         .send()?;

    if let Some(doc) = response.into_document() {
        println!("id: {}", doc.id);
    }
    # Ok(())
    # }
    ```

    For more details on document types, see the [`types`][types-mod] module.

    Get the same document as a `serde_json::Value`:

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
    let response = client.document_get::<Value>(index("myindex"), id(1))
                         .ty("mytype")
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
    pub fn document_get<TDocument>(&self, index: Index<'static>, id: Id<'static>) -> GetRequestBuilder<TSender, TDocument>
    where
        TDocument: DeserializeOwned + DocumentType,
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(
            self.clone(),
            None,
            GetRequestInner {
                index: index,
                ty: ty,
                id: id,
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
    /** Set the type for the get request. */
    pub fn ty<I>(mut self, ty: I) -> Self
    where
        I: Into<Type<'static>>,
    {
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

    Get a document from an index called `myindex` with an id of `1`:

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
    let response = client.document_get::<Value>(index("myindex"), id(1))
                         .ty("mytype")
                         .send()?;
    
    if let Some(doc) = response.into_document() {
        println!("{:?}", doc);
    }
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<GetResponse<TDocument>> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
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

    Get a document from an index called `myindex` with an id of `1`:

    ```no_run
    # extern crate futures;
    # extern crate tokio_core;
    # extern crate serde;
    # extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use serde_json::Value;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    let future = client.document_get::<Value>(index("myindex"), id(1))
                       .ty("mytype")
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
    */
    pub fn send(self) -> Pending<TDocument> {
        let req = self.inner.into_request();

        let res_future = RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
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
    use serde_json::Value;
    use prelude::*;

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_get::<Value>(index("test-idx"), id("1"))
            .inner
            .into_request();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_get::<Value>(index("test-idx"), id("1"))
            .ty("new-ty")
            .inner
            .into_request();

        assert_eq!("/test-idx/new-ty/1", req.url.as_ref());
    }
}
