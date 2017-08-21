use serde_json;
use futures::{Future, IntoFuture};
use serde::Serialize;

use error::{self, Result, Error};
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{Index, Type, Id, IndexRequest, RequestBuilder};
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
[Client.document_index]: ../struct.Client.html#index-request
*/
pub type DocumentIndexRequestBuilder<TSender, TDocument> = RequestBuilder<TSender, DocumentIndexRequestInner<TDocument>>;

#[doc(hidden)]
pub struct DocumentIndexRequestInner<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    doc: TDocument,
}

/**
# Index request
*/
impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /**
    Create a [`DocumentIndexRequestBuilder`][DocumentIndexRequestBuilder] with this `Client` that can be configured before sending.

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
    # fn main() {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateFormat>
    # }
    # let client = ClientBuilder::new().build()?;
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    let response = client.document_index(index("myindex"), id(doc.id), doc)
                         .send()?;
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.
    
    [DocumentIndexRequestBuilder]: requests/type.DocumentIndexRequestBuilder.html
    [builder-methods]: requests/type.DocumentIndexRequestBuilder.html#builder-methods
    [send-sync]: requests/type.DocumentIndexRequestBuilder.html#send-synchronously
    [send-async]: requests/type.DocumentIndexRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn document_index<TDocument>(&self,
                                         index: Index<'static>,
                                         id: Id<'static>,
                                         doc: TDocument)
                                         -> DocumentIndexRequestBuilder<TSender, TDocument>
        where TDocument: Serialize + DocumentType
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(self.clone(),
                            None,
                            DocumentIndexRequestInner {
                                index: index,
                                ty: ty,
                                id: id,
                                doc: doc,
                            })
    }
}

impl<TDocument> DocumentIndexRequestInner<TDocument>
    where TDocument: Serialize
{
    fn into_request(self) -> Result<IndexRequest<'static, Vec<u8>>> {
        let body = serde_json::to_vec(&self.doc).map_err(|e| error::request(e))?;

        Ok(IndexRequest::for_index_ty_id(self.index, self.ty, self.id, body))
    }
}

/**
# Builder methods

Configure a `DocumentIndexRequestBuilder` before sending it.
*/
impl<TSender, TDocument> DocumentIndexRequestBuilder<TSender, TDocument> 
    where TSender: Sender
{
    /** Set the type for the index request. */
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.inner.ty = ty.into();
        self
    }
}

/**
# Send synchronously
*/
impl<TDocument> DocumentIndexRequestBuilder<SyncSender, TDocument>
    where TDocument: Serialize
{
    /**
    Send a `DocumentIndexRequestBuilder` synchronously using a [`SyncClient`]().

    This will block the current thread until a response arrives and is deserialised.
    */
    pub fn send(self) -> Result<IndexResponse> {
        let req = self.inner.into_request()?;

        RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TDocument> DocumentIndexRequestBuilder<AsyncSender, TDocument>
    where TDocument: Serialize + Send + 'static
{
    /**
    Send a `DocumentIndexRequestBuilder` asynchronously using an [`AsyncClient`]().
    
    This will return a future that will resolve to the deserialised index response.
    */
    pub fn send(self) -> Box<Future<Item = IndexResponse, Error = Error>> {
        let (client, params) = (self.client, self.params);

        let req_future = self.inner.into_request().into_future();

        let res_future = req_future.and_then(move |req| {
            RequestBuilder::new(client, params, RawRequestInner::new(req))
            .send()
            .and_then(|res| res.into_response())
        });

        Box::new(res_future)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use prelude::*;

    #[test]
    fn default_request() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .document_index(index("test-idx"), id("1"), Value::Null)
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
        assert_eq!("null".as_bytes().to_vec(), req.body);
    }

    #[test]
    fn specify_ty() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .document_index(index("test-idx"), id("1"), Value::Null)
            .ty("new-ty")
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/new-ty/1", req.url.as_ref());
    }

    #[test]
    fn document_borrow() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let doc = Value::Null;
        let req = client
            .document_index(index("test-idx"), id("1"), &doc)
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
    }
}
