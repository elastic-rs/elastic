/*!
Builders for [index requests][docs-index].

[docs-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html
*/

use serde_json;
use futures::{Future, Poll};
use serde::Serialize;

use error::{self, Error, Result};
use client::Client;
use client::sender::{AsyncSender, Sender, SyncSender};
use client::requests::RequestBuilder;
use client::requests::params::{Id, Index, Type};
use client::requests::endpoints::IndexRequest;
use client::requests::raw::RawRequestInner;
use client::responses::IndexResponse;
use types::document::DocumentType;

/** 
An [index request][docs-index] builder that can be configured before sending.

Call [`Client.document_index`][Client.document_index] to get an `IndexRequest`. 
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document_index]: ../../struct.Client.html#index-document
*/
pub type IndexRequestBuilder<TSender, TDocument> = RequestBuilder<TSender, IndexRequestInner<TDocument>>;

#[doc(hidden)]
pub struct IndexRequestInner<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Option<Id<'static>>,
    doc: TDocument,
}

/**
# Index document
*/
impl<TSender> Client<TSender>
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
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
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
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    let doc_id = doc.id;
    let response = client.document_index(index("myindex"), doc)
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
    pub fn document_index<TDocument>(&self, index: Index<'static>, doc: TDocument) -> IndexRequestBuilder<TSender, TDocument>
    where
        TDocument: Serialize + DocumentType,
    {
        let ty = TDocument::name().into();

        RequestBuilder::initial(
            self.clone(),
            IndexRequestInner {
                index: index,
                ty: ty,
                id: None,
                doc: doc,
            },
        )
    }
}

impl<TDocument> IndexRequestInner<TDocument>
where
    TDocument: Serialize,
{
    fn into_request(self) -> Result<IndexRequest<'static, Vec<u8>>> {
        let body = serde_json::to_vec(&self.doc).map_err(error::request)?;

        let request = match self.id {
            Some(id) => IndexRequest::for_index_ty_id(self.index, self.ty, id, body),
            None => IndexRequest::for_index_ty(self.index, self.ty, body),
        };

        Ok(request)
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
    /** Set the type for the index request. */
    pub fn ty<I>(mut self, ty: I) -> Self
    where
        I: Into<Type<'static>>,
    {
        self.inner.ty = ty.into();
        self
    }

    /** Set the id for the index request. */
    pub fn id<I>(mut self, id: I) -> Self
    where
        I: Into<Id<'static>>
    {
        self.inner.id = Some(id.into());
        self
    }
}

/**
# Send synchronously
*/
impl<TDocument> IndexRequestBuilder<SyncSender, TDocument>
where
    TDocument: Serialize,
{
    /**
    Send a `IndexRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Index a document with an id of `1`:

    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
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
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    let doc_id = doc.id;
    let response = client.document_index(index("myindex"), doc)
                         .id(doc_id)
                         .send()?;
    
    assert!(response.created());
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<IndexResponse> {
        let req = self.inner.into_request()?;

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TDocument> IndexRequestBuilder<AsyncSender, TDocument>
where
    TDocument: Serialize + Send + 'static,
{
    /**
    Send a `IndexRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
    This will return a future that will resolve to the deserialised index response.

    # Examples

    Index a document with an id of `1`:

    ```no_run
    # extern crate serde;
    # extern crate futures;
    # extern crate tokio_core;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    let doc_id = doc.id;
    let future = client.document_index(index("myindex"), doc)
                       .id(doc_id)
                       .send();
    
    future.and_then(|response| {
        assert!(response.created());

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    */
    pub fn send(self) -> Pending {
        let (client, params_builder, inner) = (self.client, self.params_builder, self.inner);

        let req_future = client.sender.maybe_async(move || inner.into_request());

        let res_future = req_future.and_then(move |req| {
            RequestBuilder::new(client, params_builder, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response())
        });

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<Future<Item = IndexResponse, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = IndexResponse, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = IndexResponse;
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
            .document_index(index("test-idx"), Value::Null)
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/value", req.url.as_ref());
        assert_eq!("null".as_bytes().to_vec(), req.body);
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_index(index("test-idx"), Value::Null)
            .ty("new-ty")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/new-ty", req.url.as_ref());
    }

    #[test]
    fn specify_id() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_index(index("test-idx"), Value::Null)
            .id(1)
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
    }

    #[test]
    fn document_borrow() {
        let client = SyncClientBuilder::new().build().unwrap();

        let doc = Value::Null;
        let req = client
            .document_index(index("test-idx"), &doc)
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/value", req.url.as_ref());
    }
}
