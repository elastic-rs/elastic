use std::marker::PhantomData;
use serde::de::DeserializeOwned;

use error::*;
use client::{into_response, Client, SearchRequestBuilder};
use client::requests::{empty_body, DefaultBody, IntoBody, Index, Type, SearchRequest, RequestBuilder};
use client::responses::SearchResponse;

impl Client {
    /// Create a `RequestBuilder` for a search request.
    pub fn search<'a, TDocument>
        (&'a self)
         -> RequestBuilder<'a, SearchRequestBuilder<TDocument, DefaultBody>, DefaultBody>
        where TDocument: DeserializeOwned
    {
        RequestBuilder::new(&self, None, SearchRequestBuilder::new())
    }
}

impl<TDocument> SearchRequestBuilder<TDocument, DefaultBody> {
    fn new() -> Self {
        SearchRequestBuilder {
            index: None,
            ty: None,
            body: empty_body(),
            _marker: PhantomData,
        }
    }
}

impl<'a, TDocument, TBody> RequestBuilder<'a, SearchRequestBuilder<TDocument, TBody>, TBody>
    where TDocument: DeserializeOwned,
          TBody: IntoBody
{
    /// Set the indices for the search request.
    ///
    /// If no index is specified then `_all` will be used.
    pub fn index<I>(mut self, index: I) -> Self
        where I: Into<Index<'static>>
    {
        self.req.index = Some(index.into());
        self
    }

    /// Set the types for the search request.
    pub fn ty<I>(mut self, ty: Option<I>) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.map(Into::into);
        self
    }

    /// Set the body for the search request.
    ///
    /// If no body is specified then an empty query will be used.
    pub fn body<TNewBody>
        (self,
         body: TNewBody)
         -> RequestBuilder<'a, SearchRequestBuilder<TDocument, TNewBody>, TNewBody>
        where TNewBody: IntoBody
    {
        RequestBuilder::new(self.client,
                            self.params,
                            SearchRequestBuilder {
                                body: body,
                                index: self.req.index,
                                ty: self.req.ty,
                                _marker: PhantomData,
                            })
    }

    /// Send the search request.
    pub fn send(self) -> Result<SearchResponse<TDocument>> {
        let req = self.req;

        let index = req.index.unwrap_or("_all".into());

        let req = match req.ty {
            Some(ty) => SearchRequest::for_index_ty(index, ty, req.body),
            None => SearchRequest::for_index(index, req.body),
        };

        RequestBuilder::new(self.client, self.params, req).send_raw().and_then(into_response)
    }
}