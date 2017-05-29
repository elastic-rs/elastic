use serde_json;
use serde::Serialize;

use error::*;
use client::{into_response, Client};
use client::requests::{Index, Type, Id, IndexRequest, RequestBuilder};
use client::responses::IndexResponse;
use types::document::DocumentType;

/** A builder for an [`index_document`]() request. */
pub struct IndexRequestBuilder<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    doc: TDocument,
}

impl Client {
    /** Create a `RequestBuilder` for an index request. */
    pub fn index_document<'a, TDocument>
        (&'a self,
         index: Index<'static>,
         id: Id<'static>,
         doc: TDocument)
         -> RequestBuilder<'a, IndexRequestBuilder<TDocument>, TDocument>
        where TDocument: Serialize + DocumentType
    {
        let ty = TDocument::name().into();

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
    fn into_request(self) -> Result<IndexRequest<'static, Vec<u8>>> {
        let body = serde_json::to_vec(&self.doc)?;

        Ok(IndexRequest::for_index_ty_id(self.index, self.ty, self.id, body))
    }
}

impl<'a, TDocument> RequestBuilder<'a, IndexRequestBuilder<TDocument>, TDocument>
    where TDocument: Serialize
{
    /** Set the type for the index request. */
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.into();
        self
    }

    /** Send the index request. */
    pub fn send(self) -> Result<IndexResponse> {
        let req = self.req.into_request()?;

        RequestBuilder::new(self.client, self.params, req)
            .send_raw()
            .and_then(into_response)
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
