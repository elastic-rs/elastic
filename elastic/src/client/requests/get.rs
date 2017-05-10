use std::marker::PhantomData;
use serde::de::DeserializeOwned;

use error::*;
use client::{into_response, Client};
use client::requests::{DefaultBody, Index, Type, Id, GetRequest, RequestBuilder};
use client::responses::GetResponse;
use types::document::DocumentType;

/// A builder for a search request.
pub struct GetRequestBuilder<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    _marker: PhantomData<TDocument>,
}

impl Client {
    /// Create a `RequestBuilder` for a search request.
    pub fn get<'a, TDocument>(&'a self,
                              index: Index<'static>,
                              id: Id<'static>)
                              -> RequestBuilder<'a, GetRequestBuilder<TDocument>, DefaultBody>
        where TDocument: DeserializeOwned + DocumentType
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(&self,
                            None,
                            GetRequestBuilder {
                                index: index,
                                ty: ty,
                                id: id,
                                _marker: PhantomData,
                            })
    }
}

impl<TDocument> GetRequestBuilder<TDocument> {
    fn into_request(self) -> GetRequest<'static> {
        GetRequest::for_index_ty_id(self.index, self.ty, self.id)
    }
}

impl<'a, TDocument> RequestBuilder<'a, GetRequestBuilder<TDocument>, DefaultBody>
    where TDocument: DeserializeOwned + DocumentType
{
    /// Set the index for the get request.
    pub fn index<I>(mut self, index: I) -> Self
        where I: Into<Index<'static>>
    {
        self.req.index = index.into();
        self
    }

    /// Set the type for the get request.
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.into();
        self
    }

    /// Set the id for the get request.
    pub fn id<I>(mut self, id: I) -> Self
        where I: Into<Id<'static>>
    {
        self.req.id = id.into();
        self
    }

    /// Send the search request.
    pub fn send(self) -> Result<GetResponse<TDocument>> {
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

        let req = client.get::<Value>(index("test-idx"), id("1")).req.into_request();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req =
            client.get::<Value>(index("test-idx"), id("1")).index("new-idx").req.into_request();

        assert_eq!("/new-idx/value/1", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.get::<Value>(index("test-idx"), id("1")).ty("new-ty").req.into_request();

        assert_eq!("/test-idx/new-ty/1", req.url.as_ref());
    }

    #[test]
    fn specify_id() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.get::<Value>(index("test-idx"), id("1")).id("new-id").req.into_request();

        assert_eq!("/test-idx/value/new-id", req.url.as_ref());
    }
}
