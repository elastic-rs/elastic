/*!
Builders for [update document requests][docs-update].

[docs-update]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update.html
*/

use std::marker::PhantomData;
use futures::{Future, Poll};
use serde_json;
use serde::ser::Serialize;

use error::{self, Error};
use client::Client;
use client::sender::{AsyncSender, Sender, SyncSender};
use client::requests::RequestBuilder;
use client::requests::params::{Id, Index, Type};
use client::requests::endpoints::UpdateRequest;
use client::requests::raw::RawRequestInner;
use client::responses::UpdateResponse;
use types::document::DocumentType;

pub use client::requests::common::{Doc, Script, ScriptBuilder, DefaultParams};

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
# Update document request
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
                         .script(r#"ctx._source.title = "New Title""#)
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
                         .script_fluent("ctx._source.title = params.newTitle", |script| script
                            .param("newTitle", "New Title"))
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    [UpdateRequestBuilder]: requests/document_update/type.UpdateRequestBuilder.html
    [builder-methods]: requests/document_update/type.UpdateRequestBuilder.html#builder-methods
    [send-sync]: requests/document_update/type.UpdateRequestBuilder.html#send-synchronously
    [send-async]: requests/document_update/type.UpdateRequestBuilder.html#send-asynchronously
    [documents-mod]: ../types/document/index.html
    */
    pub fn document_update<TDocument>(&self, index: Index<'static>, id: Id<'static>) -> UpdateRequestBuilder<TSender, Doc<TDocument>>
    where
        TDocument: DocumentType,
    {
        let ty = TDocument::name().into();

        RequestBuilder::initial(
            self.clone(),
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
    fn into_request(self) -> Result<UpdateRequest<'static, Vec<u8>>, Error> {
        let body = serde_json::to_vec(&self.body).map_err(error::request)?;

        Ok(UpdateRequest::for_index_ty_id(
            self.index,
            self.ty,
            self.id,
            body,
        ))
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

    [documents-mod]: ../../types/document/index.html
    */
    pub fn doc<TDocument>(self, doc: TDocument) -> UpdateRequestBuilder<TSender, Doc<TDocument>>
    where
        TDocument: Serialize + DocumentType,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
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
    Update the source using [an inline script][painless-lang].
    
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
                         .script(r#"ctx._source.title = "New Title""#)
                         .send();
    # Ok(())
    # }
    ```

     Update the `title` property of a document using a parameterised script:
    
    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::client::requests::document_update::ScriptBuilder;
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
    let script = ScriptBuilder::new("ctx._source.title = params.newTitle")
        .param("newTitle", "New Title");

    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .script(script)
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    [painless-lang]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-painless.html
     */
    pub fn script<TScript, TParams>(self, builder: TScript) -> UpdateRequestBuilder<TSender, Script<TParams>>
    where
        TScript: Into<ScriptBuilder<TParams>>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            UpdateRequestInner {
                body: builder.into().build(),
                index: self.inner.index,
                ty: self.inner.ty,
                id: self.inner.id,
                _marker: PhantomData,
            },
        )
    }

    /**
    Update the source using [a script][painless-lang] configured by a fluent closure API.

    The fluent API can be more ergonomic to work with than constructing builders directly.
    Susequent calls to `script` will override any previous script properties.

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
                         .script_fluent("ctx._source.title = params.newTitle", |script| script
                            .param("newTitle", "New Title"))
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    Script parameters can also be strongly typed, so long as they implement the `Serialize` trait.
    If the parameters don't serialize to an object then Elasticsearch will fail to parse them:

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
    #[derive(Serialize)]
    struct MyParams {
        title: &'static str
    }

    let response = client.document_update::<MyType>(index("myindex"), id(1))
                         .script_fluent("ctx._source.title = params.title", |script| script
                            .params(MyParams {
                                title: "New Title",
                            }))
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    [painless-lang]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-painless.html
    */
    pub fn script_fluent<TScript, TBuilder, TParams>(self, source: TScript, builder: TBuilder) -> UpdateRequestBuilder<TSender, Script<TParams>>
    where
        TScript: ToString,
        TBuilder: Fn(ScriptBuilder<DefaultParams>) -> ScriptBuilder<TParams>,
    {
        let builder = builder(ScriptBuilder::new(source));

        self.script(builder)
    }
}

/**
# Send synchronously
*/
impl<TBody> UpdateRequestBuilder<SyncSender, TBody>
where
    TBody: Serialize,
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
        let req = self.inner.into_request()?;

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
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
        let (client, params_builder, inner) = (self.client, self.params_builder, self.inner);

        let req_future = client.sender.maybe_async(move || inner.into_request());

        let res_future = req_future.and_then(move |req| {
            RequestBuilder::new(client, params_builder, RawRequestInner::new(req))
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

#[cfg(test)]
mod tests {
    use serde_json::{self, Value};
    use super::ScriptBuilder;
    use prelude::*;

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .inner
            .into_request()
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
            .into_request()
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

        let expected_body = json!({ "doc": doc });

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .doc(doc)
            .inner
            .into_request()
            .unwrap();

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_inline_script() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .script("ctx._source.a = params.str")
            .inner
            .into_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "inline": "ctx._source.a = params.str"
            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_script_value() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .script(ScriptBuilder::new("ctx._source.a = params.str"))
            .inner
            .into_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "inline": "ctx._source.a = params.str"
            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_script_fluent() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .script_fluent("ctx._source.a = params.str", |script| {
                script
                    .lang(Some("painless"))
                    .param("str", "some value")
                    .param("other", "some other value")
            })
            .inner
            .into_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "inline": "ctx._source.a = params.str",
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
            b: i32,
        }

        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_update::<Value>(index("test-idx"), id("1"))
            .script_fluent("ctx._source.a = params.str", |script| {
                script.params(MyParams {
                    a: "some value",
                    b: 42,
                })
            })
            .inner
            .into_request()
            .unwrap();

        let expected_body = json!({
            "script": {
                "inline": "ctx._source.a = params.str",
                "params": {
                    "a": "some value",
                    "b": 42
                }
            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }
}
