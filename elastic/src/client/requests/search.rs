use std::marker::PhantomData;
use serde::de::DeserializeOwned;

use error::*;
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{empty_body, DefaultBody, IntoBody, Index, Type, SearchRequest,
                       RequestBuilder, RawRequestBuilder};
use client::responses::SearchResponse;

/** 
A builder for a [`Client.search`][Client.search] request. 

[Client.search]: ../struct.Client.html#method.search
*/
pub struct SearchRequestBuilder<TDocument, TBody> {
    index: Option<Index<'static>>,
    ty: Option<Type<'static>>,
    body: TBody,
    _marker: PhantomData<TDocument>,
}

impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /** 
    Create a [`RequestBuilder` for a search request][RequestBuilder.search]. 

    # Examples

    Run a simple [Query String][docs-querystring] query for a [`DocumentType`][documents-mod] called `MyType`:
    
    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate json_str;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = ClientBuilder::new().build().unwrap();
    let response = client.search::<MyType>()
                         .index("myindex")
                         .body(json_str!({
                             query: {
                                 query_string: {
                                     query: "*"
                                 }
                             }
                         }))
                         .send()
                         .unwrap();

    // Iterate through the hits (of type `MyType`)
    for hit in response.hits() {
        println!("{:?}", hit);
    }
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    It's also possible to use `serde_json::Value`s as documents when searching:

    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # let client = ClientBuilder::new().build().unwrap();
    let response = client.search::<Value>()
                         .index("myindex")
                         .ty(Some("mytype"))
                         .send()
                         .unwrap();
    # }
    ```

    [RequestBuilder.search]: requests/struct.RequestBuilder.html#search-request-builder
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
    */
    pub fn search<TDocument>
        (&self)
         -> RequestBuilder<TSender, SearchRequestBuilder<TDocument, DefaultBody>>
        where TDocument: DeserializeOwned
    {
        RequestBuilder::new(self.clone(), None, SearchRequestBuilder::new(empty_body()))
    }
}

impl<TDocument, TBody> SearchRequestBuilder<TDocument, TBody>
    where TDocument: DeserializeOwned
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

/** 
# Search request builder

A request builder for a [Search][docs-search] query.

Call [`Client.search`][Client.search] to get a `RequestBuilder` for a search request.

[Client.search]: ../struct.Client.html#method.search
[docs-search]: http://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html
*/
impl<TSender, TDocument, TBody> RequestBuilder<TSender, SearchRequestBuilder<TDocument, TBody>>
    where TSender: Sender
{
    /**
    Set the indices for the search request.
    
    If no index is specified then `_all` will be used.
    */
    pub fn index<I>(mut self, index: I) -> Self
        where I: Into<Index<'static>>
    {
        self.req.index = Some(index.into());
        self
    }

    /** Set the types for the search request. */
    pub fn ty<I>(mut self, ty: Option<I>) -> Self
        where I: Into<Type<'static>>
    {
        self.req.ty = ty.map(Into::into);
        self
    }

    /**
    Set the body for the search request.
    
    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self,
                          body: TNewBody)
                          -> RequestBuilder<TSender, SearchRequestBuilder<TDocument, TNewBody>>
        where TNewBody: Into<TSender::Body>
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
}

impl<TDocument, TBody> RequestBuilder<SyncBody, SearchRequestBuilder<TDocument, TBody>>
    where TDocument: DeserializeOwned,
          TBody: Into<<SyncSender as Sender>::Body>
{
    /** Send the search request. */
    pub fn send(self) -> Result<SearchResponse<TDocument>> {
        let req = self.req.into_request();

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

        let req = client
            .search::<Value>()
            .ty(Some("new-ty"))
            .req
            .into_request();

        assert_eq!("/_all/new-ty/_search", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.search::<Value>().body("{}").req.into_request();

        assert_eq!("{}", req.body);
    }
}
