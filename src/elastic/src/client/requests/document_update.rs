/*!
Builders for [update document requests][docs-update].

[docs-update]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update.html
*/

use std::marker::PhantomData;
use futures::{Future, IntoFuture, Poll};
use futures_cpupool::CpuPool;
use serde_json::{self, Value, Map};
use serde::ser::{Serialize, Serializer};

use error::{self, Error};
use client::{AsyncSender, Client, Sender, SyncSender};
use client::requests::RequestBuilder;
use client::requests::params::{Id, Index, Type};
use client::requests::endpoints::UpdateRequest;
use client::requests::raw::RawRequestInner;
use client::responses::UpdateResponse;
use types::document::DocumentType;

/** 
An [update document request][docs-update] builder that can be configured before sending.

Call [`Client.document_update`][Client.document_update] to get an `UpdateRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-update]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document_update]: ../../struct.Client.html#update-document
*/
pub type UpdateRequestBuilder<TSender, TBody> = RequestBuilder<TSender, UpdateRequestInner<TBody>>;

#[doc(hidden)]
pub struct UpdateRequestInner<TBody> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    body: TBody,
    _marker: PhantomData<TBody>,
}

/**
# Update document
*/
impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /** 
    Create an [`UpdateRequestBuilder`][UpdateRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Update a [`DocumentType`][documents-mod] called `MyType` with an id of `1` using a new document value:
    
    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .doc(new_doc)
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    The document doesn't necessarily need to be of the same type so partial updates can be made.
    Be careful though because this can lead to unexpected results or runtime errors if the document types don't align:

    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_json;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .doc(json!({
                             "title": "New Title"
                         }))
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    Documents can also be updated using a script:

    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .inline_script(r#"ctx._source.title = "New Title""#)
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    Scripts can be configured with parameters:

    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .script(|script| script
                            .source("ctx._source.title = params.newTitle")
                            .param("newTitle", "New Title"))
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```
    */
    pub fn document_update<TDocument>(&self, index: Index<'static>, id: Id<'static>) -> UpdateRequestBuilder<TSender, Doc<TDocument>>
    where
        TDocument: DocumentType,
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(
            self.clone(),
            None,
            UpdateRequestInner {
                index: index,
                ty: ty,
                id: id,
                body: Doc::empty(),
                _marker: PhantomData,
            },
        )
    }
}

impl<TBody> UpdateRequestInner<TBody>
where
    TBody: Serialize,
{
    fn into_sync_request(self) -> Result<UpdateRequest<'static, Vec<u8>>, Error> {
        let body = serde_json::to_vec(&self.body).map_err(error::request)?;

        Ok(UpdateRequest::for_index_ty_id(
            self.index,
            self.ty,
            self.id,
            body,
        ))
    }
}

impl<TBody> UpdateRequestInner<TBody>
where
    TBody: Serialize + Send + 'static,
{
    fn into_async_request(self, ser_pool: Option<CpuPool>) -> Box<Future<Item = UpdateRequest<'static, Vec<u8>>, Error = Error>> {
        if let Some(ser_pool) = ser_pool {
            let request_future = ser_pool.spawn_fn(|| self.into_sync_request());

            Box::new(request_future)
        } else {
            Box::new(self.into_sync_request().into_future())
        }
    }
}

/**
# Builder methods

Configure an `UpdateRequestBuilder` before sending it.
*/
impl<TSender, TBody> UpdateRequestBuilder<TSender, TBody>
where
    TSender: Sender,
{
    /** Set the type for the update request. */
    pub fn ty<I>(mut self, ty: I) -> Self
    where
        I: Into<Type<'static>>,
    {
        self.inner.ty = ty.into();
        self
    }

    /**
    Update the source using a document.
    
    # Examples

    Update a [`DocumentType`][documents-mod] called `MyType` with an id of `1` using a new document value:
    
    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .doc(new_doc)
                         .send();
    # Ok(())
    # }
    ```

    The document doesn't necessarily need to be of the same type so partial updates can be made.
    Be careful though because this can lead to unexpected results or runtime errors if the document types don't align:

    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_json;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .doc(json!({
                             "title": "New Title"
                         }))
                         .send();
    # Ok(())
    # }
    ```
    */
    pub fn doc<TDocument>(self, doc: TDocument) -> UpdateRequestBuilder<TSender, Doc<TDocument>>
    where
        TDocument: Serialize + DocumentType
    {
        RequestBuilder::new(
            self.client,
            self.params,
            UpdateRequestInner {
                body: Doc::value(doc),
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                _marker: PhantomData,
            },
        )
    }

    /**
    Update the source using [an inline script][painless-script].
    
    # Examples

    Update the `title` property of a document using a script:

    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .inline_script(r#"ctx._source.title = "New Title""#)
                         .send();
    # Ok(())
    # }
    ```
    */
    pub fn inline_script<TScript>(self, source: TScript) -> UpdateRequestBuilder<TSender, Script<DefaultParams>>
    where
        TScript: ToString
    {
        self.script(|script| script.source(source.to_string()))
    }

    /**
    Update the source using a script.

    If a `source` isn't specified in the builder then a default no-op script is used.

    # Examples

    Update the `title` property of a document using a parameterised script:
    
    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .script(|script| script
                            .source("ctx._source.title = params.newTitle")
                            .param("newTitle", "New Title"))
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```
    */
    pub fn script<TBuilder, TParams>(self, builder: TBuilder) -> UpdateRequestBuilder<TSender, Script<TParams>>
    where
        TBuilder: Fn(ScriptBuilder<DefaultParams>) -> ScriptBuilder<TParams>
    {
        let script = builder(ScriptBuilder::new()).build();
        
        RequestBuilder::new(
            self.client,
            self.params,
            UpdateRequestInner {
                body: script,
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                _marker: PhantomData,
            },
        )
    }
}

/**
# Send synchronously
*/
impl<TBody> UpdateRequestBuilder<SyncSender, TBody>
where TBody: Serialize,
{
    /**
    Send an `UpdateRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Update a document from an index called `myindex` with an id of `1`:

    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .doc(new_doc)
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<UpdateResponse, Error> {
        let req = self.inner.into_sync_request()?;

        RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TBody> UpdateRequestBuilder<AsyncSender, TBody> 
where
    TBody: Serialize + Send + 'static,
{
    /**
    Send an `UpdateRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
    This will return a future that will resolve to the deserialised update document response.

    # Examples

    Update a document from an index called `myindex` with an id of `1`:

    ```no_run
    # extern crate futures;
    # extern crate tokio_core;
    # extern crate serde;
    # extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use serde_json::Value;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let core = tokio_core::reactor::Core::new()?;
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    # let new_doc = MyType { id: 1, title: String::new(), timestamp: Date::now() };
    let future = client.document_update::<Value>(index("myindex"), id(1))
                       .doc(new_doc)
                       .send();

    future.and_then(|response| {
        assert!(response.updated());

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    */
    pub fn send(self) -> Pending {
        let (client, params) = (self.client, self.params);

        let ser_pool = client.sender.serde_pool.clone();
        let req_future = self.inner.into_async_request(ser_pool);

        let res_future = req_future.and_then(move |req| {
            RequestBuilder::new(client, params, RawRequestInner::new(req))
                .send()
                .and_then(|res| res.into_response())
        });

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<Future<Item = UpdateResponse, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = UpdateResponse, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = UpdateResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

/** Update an indexed document using a new document. */
#[derive(Serialize)]
pub struct Doc<TDocument> {
    doc: DocInner<TDocument>,
}

impl<TDocument> Doc<TDocument> {
    fn empty() -> Self {
        Doc {
            doc: DocInner { inner: None }
        }
    }

    fn value(doc: TDocument) -> Self {
        Doc {
            doc: DocInner { inner: Some(doc) }
        }
    }
}

struct DocInner<TDocument> {
    inner: Option<TDocument>,
}

impl<TDocument> Serialize for DocInner<TDocument>
where
    TDocument: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.inner {
            Some(ref doc) => doc.serialize(serializer),
            None => Value::Object(Map::new()).serialize(serializer),
        }
    }
}

type DefaultParams = Map<String, Value>;

/** Update an indexed document using a script. */
#[derive(Serialize)]
pub struct Script<TParams> {
    script: ScriptInner<TParams>,
}

#[derive(Serialize)]
struct ScriptInner<TParams> {
    source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<TParams>,
}

/** A builder for an update script that can be configured before sending. */
pub struct ScriptBuilder<TParams> {
    source: Option<String>,
    lang: Option<String>,
    params: Option<TParams>,
}

impl ScriptBuilder<DefaultParams> {
    fn new() -> Self {
        ScriptBuilder {
            params: None,
            source: None,
            lang: None,
        }
    }

    /** Set a script parameter. */
    pub fn param<TKey, TValue>(mut self, key: TKey, value: TValue) -> Self 
    where
        TKey: ToString,
        TValue: Into<Value>,
    {
        let mut params = self.params.unwrap_or_else(DefaultParams::new);
        params.insert(key.to_string(), value.into());

        self.params = Some(params);
        self
    }
}

impl<TParams> ScriptBuilder<TParams> {
    /** Set the source for the update script. */
    pub fn source<TSource>(mut self, source: TSource) -> Self 
        where TSource: ToString,
    {
        self.source = Some(source.to_string());
        self
    }

    /** Set the language for the update script. */
    pub fn lang<TLang>(mut self, lang: Option<TLang>) -> Self 
        where TLang: ToString,
    {
        self.lang = lang.map(|lang| lang.to_string());
        self
    }

    /** Specify a new set of parameters for the update script. */
    pub fn params<TNewParams>(self, params: TNewParams) -> ScriptBuilder<TNewParams> {
        ScriptBuilder {
            source: self.source,
            lang: self.lang,
            params: Some(params),
        }
    }

    fn build(self) -> Script<TParams> {
        let source = self.source.unwrap_or_else(|| "ctx._source".to_owned());

        Script {
            script: ScriptInner {
                source: source,
                params: self.params,
                lang: self.lang,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{self, Value};
    use prelude::*;

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .inner
            .into_sync_request()
            .unwrap();

        assert_eq!("/test-idx/value/1/_update", req.url.as_ref());

        let expected_body = json!({
            "doc": {}
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .ty("new-ty")
            .inner
            .into_sync_request()
            .unwrap();

        assert_eq!("/test-idx/new-ty/1/_update", req.url.as_ref());
    }

    #[test]
    fn specify_doc() {
        let client = SyncClientBuilder::new().build().unwrap();

        let doc = json!({
            "a": "string",
            "b": 123
        });

        let expected_body = json!({
            "doc": doc
        });

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .doc(doc)
            .inner
            .into_sync_request()
            .unwrap();

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_inline_script() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .inline_script("ctx._source.a = params.str")
            .inner
            .into_sync_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "source": "ctx._source.a = params.str"
            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_script() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .script(|script| script
                .source("ctx._source.a = params.str")
                .lang(Some("painless"))
                .param("str", "some value")
                .param("other", "some other value"))
            .inner
            .into_sync_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "source": "ctx._source.a = params.str",
                "lang": "painless",
                "params": {
                    "str": "some value",
                    "other": "some other value"
                }
            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_typed_script() {
        #[derive(Serialize)]
        struct MyParams {
            a: &'static str,
            b: i32
        }

        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .script(|script| script
                .source("ctx._source.a = params.str")
                .params(MyParams {
                    a: "some value",
                    b: 42
                }))
            .inner
            .into_sync_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "source": "ctx._source.a = params.str",
                "params": {
                    "a": "some value",
                    "b": 42
                }
            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_script_default() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .script(|script| script)
            .inner
            .into_sync_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "source": "ctx._source"
            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }
}
