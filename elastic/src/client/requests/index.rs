use serde_json::Value;
use serde::Serialize;

use error::*;
use client::{into_response, Client};
use client::requests::{Document, Index, Type, Id, IndexRequest, RequestBuilder};
use types::document::DocumentType;

/// A builder for a index request.
pub struct IndexRequestBuilder<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    doc: Document<TDocument>,
}

impl Client {
    /// Create a `RequestBuilder` for a index request.
    pub fn index<'a, TDocument>(&'a self,
                              index: Index<'static>,
                              id: Id<'static>,
                              doc: TDocument)
                              -> RequestBuilder<'a, IndexRequestBuilder<TDocument>, Document<TDocument>>
        where TDocument: Serialize + DocumentType
    {
        let ty = TDocument::name().into();
        let doc = Document::from(doc);

        RequestBuilder::new(&self,
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
    fn into_request(self) -> IndexRequest<'static, Document<TDocument>> {
        IndexRequest::for_index_ty_id(self.index, self.ty, self.id, self.doc)
    }
}

impl<'a, TDocument> RequestBuilder<'a, IndexRequestBuilder<TDocument>, Document<TDocument>>
    where TDocument: Serialize + DocumentType
{
    /// Set the index for the index request.
    pub fn index<I>(mut self, index: I) -> Self
        where I: Into<Index<'static>>
    {
        self.req.index = index.into();
        self
    }

    /// Set the type for the index request.
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.into();
        self
    }

    /// Set the id for the index request.
    pub fn id<I>(mut self, id: I) -> Self
        where I: Into<Id<'static>>
    {
        self.req.id = id.into();
        self
    }

    /// Send the index request.
    pub fn send(self) -> Result<Value> {
        let req = self.req.into_request();

        RequestBuilder::new(self.client, self.params, req).send_raw().and_then(into_response)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use prelude::*;

    #[test]
    fn default_request() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.index::<Value>(index("test-idx"), id("1"), Value::Null).req.into_request();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
        assert_eq!(Document::from(Value::Null), req.body);
    }

    #[test]
    fn specify_index() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req =
            client.index::<Value>(index("test-idx"), id("1"), Value::Null).index("new-idx").req.into_request();

        assert_eq!("/new-idx/value/1", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.index::<Value>(index("test-idx"), id("1"), Value::Null).ty("new-ty").req.into_request();

        assert_eq!("/test-idx/new-ty/1", req.url.as_ref());
    }

    #[test]
    fn specify_id() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.index::<Value>(index("test-idx"), id("1"), Value::Null).id("new-id").req.into_request();

        assert_eq!("/test-idx/value/new-id", req.url.as_ref());
    }
}
