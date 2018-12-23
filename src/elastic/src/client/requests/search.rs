/*!
Builders for [search requests][docs-search].

[docs-search]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html
*/

use std::marker::PhantomData;
use futures::{Future, Poll};
use serde::de::DeserializeOwned;

use error::{Error, Result};
use client::{AsyncSender, Client, Sender, SyncSender};
use client::requests::{empty_body, DefaultBody, RequestBuilder};
use client::requests::params::{Index, Type};
use client::requests::endpoints::SearchRequest;
use client::requests::raw::RawRequestInner;
use client::responses::SearchResponse;

/**
A [search request][docs-search] builder that can be configured before sending. 

Call [`Client.search`][Client.search] to get a `SearchRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-search]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.search]: ../../struct.Client.html#search-request
*/
pub type SearchRequestBuilder<TSender, TDocument, TBody> = RequestBuilder<TSender, SearchRequestInner<TDocument, TBody>>;

#[doc(hidden)]
pub struct SearchRequestInner<TDocument, TBody> {
    index: Option<Index<'static>>,
    ty: Option<Type<'static>>,
    body: TBody,
    _marker: PhantomData<TDocument>,
}

/**
# Search request
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /** 
    Create a [`SearchRequestBuilder`][SearchRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Run a simple [Query String][docs-querystring] query for a [`DocumentType`][documents-mod] called `MyType`:
    
    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let query = "a query string";

    let response = client.search::<MyType>()
                         .index("myindex")
                         .body(json!({
                             "query": {
                                 "query_string": {
                                     "query": query
                                 }
                             }
                         }))
                         .send()?;

    // Iterate through the hits (of type `MyType`)
    for hit in response.hits() {
        println!("{:?}", hit);
    }
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    It's also possible to use `serde_json::Value`s as documents when searching:

    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.search::<Value>()
                         .index("myindex")
                         .ty(Some("mytype"))
                         .send()?;
    # Ok(())
    # }
    ```

    [SearchRequestBuilder]: requests/search/type.SearchRequestBuilder.html
    [builder-methods]: requests/search/type.SearchRequestBuilder.html#builder-methods
    [send-sync]: requests/search/type.SearchRequestBuilder.html#send-synchronously
    [send-async]: requests/search/type.SearchRequestBuilder.html#send-asynchronously
    [types-mod]: ../../types/index.html
    [documents-mod]: ../../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
    */
    pub fn search<TDocument>(&self) -> SearchRequestBuilder<TSender, TDocument, DefaultBody>
    where
        TDocument: DeserializeOwned,
    {
        RequestBuilder::new(self.clone(), None, SearchRequestInner::new(empty_body()))
    }
}

impl<TDocument, TBody> SearchRequestInner<TDocument, TBody>
where
    TDocument: DeserializeOwned,
{
    fn new(body: TBody) -> Self {
        SearchRequestInner {
            index: None,
            ty: None,
            body: body,
            _marker: PhantomData,
        }
    }

    fn into_request(self) -> SearchRequest<'static, TBody> {
        let index = self.index.unwrap_or_else(|| "_all".into());

        match self.ty {
            Some(ty) => SearchRequest::for_index_ty(index, ty, self.body),
            None => SearchRequest::for_index(index, self.body),
        }
    }
}

/**
# Builder methods

Configure a `SearchRequestBuilder` before sending it.
*/
impl<TSender, TDocument, TBody> SearchRequestBuilder<TSender, TDocument, TBody>
where
    TSender: Sender,
{
    /**
    Set the indices for the search request.
    
    If no index is specified then `_all` will be used.
    */
    pub fn index<I>(mut self, index: I) -> Self
    where
        I: Into<Index<'static>>,
    {
        self.inner.index = Some(index.into());
        self
    }

    /** Set the types for the search request. */
    pub fn ty<I>(mut self, ty: Option<I>) -> Self
    where
        I: Into<Type<'static>>,
    {
        self.inner.ty = ty.map(Into::into);
        self
    }

    /**
    Set the body for the search request.
    
    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self, body: TNewBody) -> SearchRequestBuilder<TSender, TDocument, TNewBody>
    where
        TNewBody: Into<TSender::Body>,
    {
        RequestBuilder::new(
            self.client,
            self.params,
            SearchRequestInner {
                body: body,
                index: self.inner.index,
                ty: self.inner.ty,
                _marker: PhantomData,
            },
        )
    }
}

/**
# Send synchronously
*/
impl<TDocument, TBody> SearchRequestBuilder<SyncSender, TDocument, TBody>
where
    TDocument: DeserializeOwned,
    TBody: Into<<SyncSender as Sender>::Body>,
{
    /**
    Send a `SearchRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Run a simple [Query String][docs-querystring] query for a [`DocumentType`][documents-mod] called `MyType`:
    
    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.search::<MyType>()
                         .index("myindex")
                         .send()?;

    // Iterate through the hits (of type `MyType`)
    for hit in response.hits() {
        println!("{:?}", hit);
    }
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    [documents-mod]: ../../../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
    */
    pub fn send(self) -> Result<SearchResponse<TDocument>> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TDocument, TBody> SearchRequestBuilder<AsyncSender, TDocument, TBody>
where
    TDocument: DeserializeOwned + Send + 'static,
    TBody: Into<<AsyncSender as Sender>::Body>,
{
    /**
    Send a `SearchRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
    This will return a future that will resolve to the deserialised search response.

    # Examples

    Run a simple [Query String][docs-querystring] query for a [`DocumentType`][documents-mod] called `MyType`:
    
    ```no_run
    # extern crate tokio;
    # extern crate futures;
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    let future = client.search::<MyType>()
                       .index("myindex")
                       .send();

    future.and_then(|response| {
        // Iterate through the hits (of type `MyType`)
        for hit in response.hits() {
            println!("{:?}", hit);
        }

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    [documents-mod]: ../../../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
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
    inner: Box<Future<Item = SearchResponse<TDocument>, Error = Error>>,
}

impl<TDocument> Pending<TDocument> {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = SearchResponse<TDocument>, Error = Error> + 'static,
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
    type Item = SearchResponse<TDocument>;
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

        let req = client.search::<Value>().inner.into_request();

        assert_eq!("/_all/_search", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .search::<Value>()
            .index("new-idx")
            .inner
            .into_request();

        assert_eq!("/new-idx/_search", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .search::<Value>()
            .ty(Some("new-ty"))
            .inner
            .into_request();

        assert_eq!("/_all/new-ty/_search", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.search::<Value>().body("{}").inner.into_request();

        assert_eq!("{}", req.body);
    }
}
