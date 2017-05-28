use std::marker::PhantomData;
use serde::de::DeserializeOwned;

use error::*;
use client::{into_response, Client};
use client::requests::{empty_body, DefaultBody, IntoBody, Index, Type, SearchRequest,
                       RequestBuilder};
use client::responses::SearchResponse;

/** A builder for a search request. **/
pub struct SearchRequestBuilder<TDocument, TBody> {
    index: Option<Index<'static>>,
    ty: Option<Type<'static>>,
    body: TBody,
    _marker: PhantomData<TDocument>,
}

impl Client {
    /** Create a `RequestBuilder` for a search request. **/
    pub fn search<'a, TDocument>
        (&'a self)
         -> RequestBuilder<'a, SearchRequestBuilder<TDocument, DefaultBody>, DefaultBody>
        where TDocument: DeserializeOwned
    {
        RequestBuilder::new(&self, None, SearchRequestBuilder::new(empty_body()))
    }
}

impl<TDocument, TBody> SearchRequestBuilder<TDocument, TBody>
    where TDocument: DeserializeOwned,
          TBody: IntoBody
{
    fn new(body: TBody) -> Self {
        SearchRequestBuilder {
            index: None,
            ty: None,
            body: body,
            _marker: PhantomData,
        }
    }

    fn into_request(self) -> SearchRequest<'static, TBody> {
        let index = self.index.unwrap_or("_all".into());

        match self.ty {
            Some(ty) => SearchRequest::for_index_ty(index, ty, self.body),
            None => SearchRequest::for_index(index, self.body),
        }
    }
}

impl<'a, TDocument, TBody> RequestBuilder<'a, SearchRequestBuilder<TDocument, TBody>, TBody>
    where TDocument: DeserializeOwned,
          TBody: IntoBody
{
    /**
    Set the indices for the search request.
    
    If no index is specified then `_all` will be used.
    **/
    pub fn index<I>(mut self, index: I) -> Self
        where I: Into<Index<'static>>
    {
        self.req.index = Some(index.into());
        self
    }

    /** Set the types for the search request. **/
    pub fn ty<I>(mut self, ty: Option<I>) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.map(Into::into);
        self
    }

    /**
    Set the body for the search request.
    
    If no body is specified then an empty query will be used.
    **/
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

    /** Send the search request. **/
    pub fn send(self) -> Result<SearchResponse<TDocument>> {
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

        let req = client.search::<Value>().req.into_request();

        assert_eq!("/_all/_search", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.search::<Value>().index("new-idx").req.into_request();

        assert_eq!("/new-idx/_search", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.search::<Value>().ty(Some("new-ty")).req.into_request();

        assert_eq!("/_all/new-ty/_search", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.search::<Value>().body("{}").req.into_request();

        assert_eq!("{}", req.body);
    }
}
