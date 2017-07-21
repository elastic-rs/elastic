use serde_json;
use serde::Serialize;

use error::*;
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{Index, Type, Id, IndexRequest, RequestBuilder, RawRequestBuilder};
use client::responses::IndexResponse;
use types::document::DocumentType;

/** 
A builder for an [`Client.index_document`][Client.index_document] request. 

[Client.index_document]: ../struct.Client.html#method.index_document
*/
pub struct IndexRequestBuilder<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    doc: TDocument,
}

impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /**
    Create a [`RequestBuilder` for an index request][RequestBuilder.index_document].

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
    # let client = ClientBuilder::new().build().unwrap();
    let doc = MyType {
        id: 1,
        title: String::from("A title"),
        timestamp: Date::now()
    };

    let response = client.index_document(index("myindex"), id(doc.id), doc)
                         .send()
                         .unwrap();
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.
    
    [RequestBuilder.index_document]: requests/struct.RequestBuilder.html#index-document-builder
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn index_document<TDocument>(&self,
                                         index: Index<'static>,
                                         id: Id<'static>,
                                         doc: TDocument)
                                         -> RequestBuilder<TSender, IndexRequestBuilder<TDocument>>
        where TDocument: Serialize + DocumentType
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(self.clone(),
                            None,
                            IndexRequestBuilder {
                                index: index,
                                ty: ty,
                                id: id,
                                doc: doc,
                            })
    }
}

impl<TDocument> IndexRequestBuilder<TDocument>
    where TDocument: Serialize
{
    fn into_request(self) -> Result<IndexRequest<'static, Vec<u8>>> {
        let body = serde_json::to_vec(&self.doc)?;

        Ok(IndexRequest::for_index_ty_id(self.index, self.ty, self.id, body))
    }
}

/** 
# Index document builder

A request builder for an [Index][docs-index] request.

Call [`Client.index_document`][Client.index_document] to get a `RequestBuilder` for an index request.

[Client.index_document]: ../struct.Client.html#method.index_document
[docs-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html
*/
impl<TSender, TDocument> RequestBuilder<TSender, IndexRequestBuilder<TDocument>> 
    where TSender: Sender
{
    /** Set the type for the index request. */
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.into();
        self
    }
}

impl<TDocument> RequestBuilder<SyncSender, IndexRequestBuilder<TDocument>>
    where TDocument: Serialize
{
    /** Send the index request. */
    pub fn send(self) -> Result<IndexResponse> {
        let req = self.req.into_request()?;

        RequestBuilder::new(self.client, self.params, RawRequestBuilder::new(req))
            .send_raw()?
            .into_response()
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
            .index_document(index("test-idx"), id("1"), Value::Null)
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
            .index_document(index("test-idx"), id("1"), Value::Null)
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
            .index_document(index("test-idx"), id("1"), &doc)
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
    }
}
