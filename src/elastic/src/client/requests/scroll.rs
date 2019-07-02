/*!
Builders for [search requests][docs-search].

[docs-search]: https://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html
*/

use futures::{
    Future,
    Poll,
};
use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    marker::PhantomData,
    borrow::Cow,
    mem,
};

use crate::{
    client::{
        requests::{
            raw::RawRequestInner,
            RequestBuilder,
        },
        responses::SearchResponse,
        Client,
        DocumentClient,
    },
    endpoints::{
        SearchRequest,
        ScrollRequest,
    },
    error::{self, Error},
    http::{
        empty_body,
        sender::{
            AsyncSender,
            Sender,
            SyncSender,
            RequestParams,
        },
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

Call [`Client.search`][Client.search] to get a `ScrollRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-search]: https://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.search]: ../../struct.Client.html#search-request
*/
pub type ScrollRequestBuilder<TSender, TDocument, TBody> =
    RequestBuilder<TSender, ScrollRequestInner<TDocument, TBody>>;

#[doc(hidden)]
pub struct ScrollRequestInner<TDocument, TBody> {
    scroll: Option<Scroll<'static>>,
    index: Option<Index<'static>>,
    ty: Option<Type<'static>>,
    body: TBody,
    _marker: PhantomData<TDocument>,
}

/**
The duration to keep the scroll context alive for.
*/
#[derive(Clone)]
pub struct Scroll<'a>(Cow<'a, str>);

impl<'a> From<&'a str> for Scroll<'a> {
    fn from(value: &'a str) -> Self {
        Scroll(value.into())
    }
}

impl<'a> From<String> for Scroll<'a> {
    fn from(value: String) -> Self {
        Scroll(value.into())
    }
}

impl<'a> Default for Scroll<'a> {
    fn default() -> Self {
        Scroll(Cow::Borrowed("1m"))
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
    Create a [`ScrollRequestBuilder`][ScrollRequestBuilder] with this `Client` that can be configured before sending.

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
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
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
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.search::<Value>()
                         .index("myindex")
                         .ty("my-type")
                         .send()?;
    # Ok(())
    # }
    ```

    [ScrollRequestBuilder]: requests/search/type.ScrollRequestBuilder.html
    [builder-methods]: requests/search/type.ScrollRequestBuilder.html#builder-methods
    [send-sync]: requests/search/type.ScrollRequestBuilder.html#send-synchronously
    [send-async]: requests/search/type.ScrollRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/master/query-dsl-query-string-query.html
    */
    pub fn scroll<TDocument>(&self) -> ScrollRequestBuilder<TSender, TDocument, DefaultBody>
    where
        TDocument: DeserializeOwned,
    {
        RequestBuilder::initial(self.clone(), ScrollRequestInner::new(empty_body()))
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
    Create a [`ScrollRequestBuilder`][ScrollRequestBuilder] with this `Client` that can be configured before sending.

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
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
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

    [ScrollRequestBuilder]: requests/search/type.ScrollRequestBuilder.html
    [builder-methods]: requests/search/type.ScrollRequestBuilder.html#builder-methods
    [send-sync]: requests/search/type.ScrollRequestBuilder.html#send-synchronously
    [send-async]: requests/search/type.ScrollRequestBuilder.html#send-asynchronously
    [types-mod]: ../../types/index.html
    [documents-mod]: ../../types/document/index.html
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/master/query-dsl-query-string-query.html
    */
    pub fn scroll(self) -> ScrollRequestBuilder<TSender, TDocument, DefaultBody>
    where
        TDocument: DeserializeOwned + DocumentType,
    {
        let index = TDocument::partial_static_index().map(|idx| idx.to_owned());
        let ty = TDocument::partial_static_ty().map(|ty| ty.to_owned());

        RequestBuilder::initial(
            self.inner,
            ScrollRequestInner {
                scroll: None,
                index,
                ty,
                body: empty_body(),
                _marker: PhantomData,
            },
        )
    }
}

impl<TDocument, TBody> ScrollRequestInner<TDocument, TBody>
where
    TDocument: DeserializeOwned,
{
    fn new(body: TBody) -> Self {
        ScrollRequestInner {
            scroll: None,
            index: None,
            ty: None,
            body: body,
            _marker: PhantomData,
        }
    }

    fn into_request(self) -> (Scroll<'static>, SearchRequest<'static, TBody>) {
        let scroll = self.scroll.unwrap_or_else(Default::default);
        let index = self.index.unwrap_or_else(|| "_all".into());

        let req = match self.ty {
            Some(ty) => SearchRequest::for_index_ty(index, ty, self.body),
            None => SearchRequest::for_index(index, self.body),
        };

        (scroll, req)
    }
}

/**
# Builder methods

Configure a `ScrollRequestBuilder` before sending it.
*/
impl<TSender, TDocument, TBody> ScrollRequestBuilder<TSender, TDocument, TBody>
where
    TSender: Sender,
{
    /**
    Set the duration to keep the scroll alive.
    */
    pub fn retain_context(mut self, keep_alive: impl Into<Scroll<'static>>) -> Self {
        self.inner.scroll = Some(keep_alive.into());

        self
    }

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
    ) -> ScrollRequestBuilder<TSender, TDocument, TNewBody>
    where
        TNewBody: Into<TSender::Body>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            ScrollRequestInner {
                scroll: None,
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
impl<TDocument, TBody> ScrollRequestBuilder<SyncSender, TDocument, TBody>
where
    TDocument: DeserializeOwned,
    TBody: Into<<SyncSender as Sender>::Body> + Send + 'static,
{
    /**
    Send a `ScrollRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Run a simple [Query String][docs-querystring] query for a [`DocumentType`][documents-mod] called `MyType`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
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
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/master/query-dsl-query-string-query.html
    */
    pub fn send(mut self) -> Result<SearchResponse<TDocument>, Error> {
        let (scroll, req) = self.inner.into_request();

        self.params_builder = self.params_builder
            .fluent(move |p| p
                .url_param("scroll", scroll.0))
            .shared();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

// TODO: Build an iterator that actually performs requests
// TODO: Separate initial request from subsequent

pub struct ScrollIter<TDocument, TBody> {
    req: ScrollIterReq<TBody>,
    _marker: PhantomData<TDocument>,
}

enum ScrollIterReq<TBody> {
    Poisoned,
    Initial(SenderRequestTemplate<SyncSender, SearchRequest<'static, TBody>>),
    Continue(SenderRequestTemplate<SyncSender, ScrollRequest<'static, Value>>),
}

struct SenderRequestTemplate<TSender, TRequest> {
    client: Client<TSender>,
    scroll: Scroll<'static>,
    params: RequestParams,
    request: TRequest,
}

impl<TDocument, TBody> ScrollIter<TDocument, TBody>
where
    TDocument: DeserializeOwned,
    TBody: Into<<SyncSender as Sender>::Body> + Send + 'static,
{
    fn scroll(&mut self) -> Result<Option<SearchResponse<TDocument>>, Error> {
        let req = mem::replace(&mut self.req, ScrollIterReq::Poisoned);

        match req {
            ScrollIterReq::Initial(template) => {
                let res = template
                    .client
                    .request(template.request)
                    .params(template.params.clone()
                        .url_param("scroll", template.scroll.0.clone()))
                    .send()?
                    .into_response::<SearchResponse<TDocument>>()?;

                let scroll_id = match res.scroll_id() {
                    Some(scroll_id) => scroll_id.to_owned(),
                    None => return Err(error::request(error::message("the scroll request didn't return a `scroll_id`")))
                };

                self.req = ScrollIterReq::Continue(SenderRequestTemplate {
                    scroll: template.scroll.clone(),
                    client: template.client,
                    params: template.params,
                    request: ScrollRequest::for_scroll_id(scroll_id, json!({
                        "scroll": template.scroll.0
                    }))
                });

                if res.hits().len() > 0 {
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            },
            ScrollIterReq::Continue(template) => {
                let res = template
                    .client
                    .request(template.request)
                    .send()?
                    .into_response::<SearchResponse<TDocument>>()?;

                let scroll_id = match res.scroll_id() {
                    Some(scroll_id) => scroll_id.to_owned(),
                    None => return Err(error::request(error::message("the scroll request didn't return a `scroll_id`")))
                };

                self.req = ScrollIterReq::Continue(SenderRequestTemplate {
                    scroll: template.scroll.clone(),
                    client: template.client,
                    params: template.params,
                    request: ScrollRequest::for_scroll_id(scroll_id, json!({
                        "scroll": template.scroll.0
                    }))
                });

                if res.hits().len() > 0 {
                    Ok(Some(res))
                } else {
                    Ok(None)
                }
            },
            ScrollIterReq::Poisoned => {
                return Err(error::request(error::message("attempt to re-use a poisoned scroll iterator")))
            }
        }
    }
}

impl<TDocument, TBody> Iterator for ScrollIter<TDocument, TBody>
where
    TDocument: DeserializeOwned,
    TBody: Into<<SyncSender as Sender>::Body> + Send + 'static,
{
    type Item = Result<SearchResponse<TDocument>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scroll().transpose()
    }
}

/**
# Send asynchronously
*/
impl<TDocument, TBody> ScrollRequestBuilder<AsyncSender, TDocument, TBody>
where
    TDocument: DeserializeOwned + Send + 'static,
    TBody: Into<<AsyncSender as Sender>::Body> + Send + 'static,
{
    /**
    Send a `ScrollRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].

    This will return a future that will resolve to the deserialised search response.

    # Examples

    Run a simple [Query String][docs-querystring] query for a [`DocumentType`][documents-mod] called `MyType`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = AsyncClientBuilder::new().build()?;
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
    [docs-querystring]: https://www.elastic.co/guide/en/elasticsearch/reference/master/query-dsl-query-string-query.html
    */
    pub fn send(mut self) -> Pending<TDocument> {
        let (scroll, req) = self.inner.into_request();

        self.params_builder = self.params_builder
            .fluent(move |p| p
                .url_param("scroll", scroll.0))
            .shared();

        let res_future =
            RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response());

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending<TDocument> {
    inner: Box<dyn Future<Item = SearchResponse<TDocument>, Error = Error> + Send>,
}

impl<TDocument> Pending<TDocument> {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = SearchResponse<TDocument>, Error = Error> + Send + 'static,
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

    use crate::{
        prelude::*,
        tests::*,
    };

    #[test]
    fn is_send() {
        assert_send::<super::Pending<TestDoc>>();
    }

    #[derive(Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    struct TestDoc {}

    #[test]
    fn default_request() {
        unimplemented!();
    }
}
