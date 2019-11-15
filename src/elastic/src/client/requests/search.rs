/*!
Builders for [search requests][docs-search].

[docs-search]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html
*/

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::SearchResponse,
        Client,
        DocumentClient,
    },
    endpoints::SearchRequest,
    error::Error,
    http::{
        empty_body,
        sender::Sender,
        DefaultBody,
    },
    params::{
        Index,
        Type,
    },
    types::document::DocumentType,
};

/**
A [search request][docs-search] builder that can be configured before sending.

Call [`Client.search`][Client.search] to get a `SearchRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-search]: https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.search]: ../../struct.Client.html#search-request
*/
pub type SearchRequestBuilder<TSender, TDocument, TBody> =
    RequestBuilder<TSender, SearchRequestInner<TDocument, TBody>>;

#[doc(hidden)]
pub struct SearchRequestInner<TDocument, TBody> {
    index: Option<Index<'static>>,
    ty: Option<Type<'static>>,
    body: TBody,
    _marker: PhantomData<TDocument>,
}

impl<TDocument, TBody> RequestInner for SearchRequestInner<TDocument, TBody>
where
    TBody: Send + 'static,
    TDocument: DeserializeOwned + Send + 'static,
{
    type Request = SearchRequest<'static, TBody>;
    type Response = SearchResponse<TDocument>;

    fn into_request(self) -> Result<Self::Request, Error> {
        let index = self.index.unwrap_or_else(|| "_all".into());

        Ok(match self.ty {
            Some(ty) => SearchRequest::for_index_ty(index, ty, self.body),
            None => SearchRequest::for_index(index, self.body),
        })
    }
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
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
    # #[macro_use] extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.search::<Value>()
                         .index("myindex")
                         .ty("my-type")
                         .send()?;
    # Ok(())
    # }
    ```

    [SearchRequestBuilder]: requests/search/type.SearchRequestBuilder.html
    [builder-methods]: requests/search/type.SearchRequestBuilder.html#builder-methods
    [send-sync]: requests/search/type.SearchRequestBuilder.html#send-synchronously
    [send-async]: requests/search/type.SearchRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
    */
    pub fn search<TDocument>(&self) -> SearchRequestBuilder<TSender, TDocument, DefaultBody>
    where
        TDocument: DeserializeOwned,
    {
        RequestBuilder::initial(self.clone(), SearchRequestInner::new(empty_body()))
    }
}

/**
# Search request
*/
impl<TSender, TDocument> DocumentClient<TSender, TDocument>
where
    TSender: Sender,
{
    /**
    Create a [`SearchRequestBuilder`][SearchRequestBuilder] with this `Client` that can be configured before sending.

    The index and type parameters will be inferred from the document type.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Run a simple [Query String][docs-querystring] query for a [`DocumentType`][documents-mod] called `MyType`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let query = "a query string";

    let response = client.document::<MyType>()
                         .search()
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

    [SearchRequestBuilder]: requests/search/type.SearchRequestBuilder.html
    [builder-methods]: requests/search/type.SearchRequestBuilder.html#builder-methods
    [send-sync]: requests/search/type.SearchRequestBuilder.html#send-synchronously
    [send-async]: requests/search/type.SearchRequestBuilder.html#send-asynchronously
    [types-mod]: ../../types/index.html
    [documents-mod]: ../../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-query-string-query.html
    */
    pub fn search(self) -> SearchRequestBuilder<TSender, TDocument, DefaultBody>
    where
        TDocument: DeserializeOwned + DocumentType,
    {
        let index = TDocument::partial_static_index().map(|idx| idx.to_owned());
        let ty = TDocument::partial_static_ty().map(|ty| ty.to_owned());

        RequestBuilder::initial(
            self.inner,
            SearchRequestInner {
                index,
                ty,
                body: empty_body(),
                _marker: PhantomData,
            },
        )
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
            body,
            _marker: PhantomData,
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
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.inner.index = Some(index.into());
        self
    }

    /** Set the types for the search request. */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
        self.inner.ty = Some(ty.into());
        self
    }

    /**
    Set the body for the search request.

    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(
        self,
        body: TNewBody,
    ) -> SearchRequestBuilder<TSender, TDocument, TNewBody>
    where
        TNewBody: Into<TSender::Body>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            SearchRequestInner {
                body,
                index: self.inner.index,
                ty: self.inner.ty,
                _marker: PhantomData,
            },
        )
    }
}

#[cfg(all(test, feature="sync_sender"))]
mod tests {
    use serde_json::Value;

    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };

    #[derive(Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.search::<Value>().inner.into_request().unwrap();

        assert_eq!("/_all/_search", req.url.as_ref());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .search::<Value>()
            .index("new-idx")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/new-idx/_search", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.search::<Value>().ty("new-ty").inner.into_request().unwrap();

        assert_eq!("/_all/new-ty/_search", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.search::<Value>().body("{}").inner.into_request().unwrap();

        assert_eq!("{}", req.body);
    }
}
