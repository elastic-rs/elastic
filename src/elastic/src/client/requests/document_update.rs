/*!
Builders for [update document requests][docs-update].

[docs-update]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update.html
*/

use serde::ser::Serialize;
use serde_json;
use std::marker::PhantomData;

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::UpdateResponse,
        DocumentClient,
    },
    endpoints::UpdateRequest,
    error::{
        self,
        Error,
    },
    http::sender::Sender,
    params::{
        Id,
        Index,
        Type,
    },
    types::document::{
        DocumentType,
        StaticIndex,
        StaticType,
        DEFAULT_DOC_TYPE,
    },
};

pub use crate::client::requests::common::{
    DefaultParams,
    Doc,
    Script,
    ScriptBuilder,
};

/**
An [update document request][docs-update] builder that can be configured before sending.

Call [`Client.document.update`][Client.document.update] to get an `UpdateRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-update]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document.update]: ../../struct.DocumentClient.html#update-document-request
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

impl<TBody> RequestInner for UpdateRequestInner<TBody>
where
    TBody: Serialize,
{
    type Request = UpdateRequest<'static, Vec<u8>>;
    type Response = UpdateResponse;

    fn into_request(self) -> Result<UpdateRequest<'static, Vec<u8>>, Error> {
        let body = serde_json::to_vec(&self.body).map_err(error::request)?;

        Ok(UpdateRequest::for_index_ty_id(
            self.index, self.ty, self.id, body,
        ))
    }
}

/**
# Update document request
*/
impl<TSender, TDocument> DocumentClient<TSender, TDocument>
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: "1".to_owned(), title: String::new(), timestamp: Date::now() };
    let response = client.document::<MyType>()
                         .update(1)
                         .doc(new_doc)
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    The document doesn't necessarily need to be of the same type so partial updates can be made.
    Be careful though because this can lead to unexpected results or runtime errors if the document types don't align:

    ```no_run
    # #[macro_use] extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<MyType>()
                         .update(1)
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: "1".to_owned(), title: String::new(), timestamp: Date::now() };
    let response = client.document::<MyType>()
                         .update(1)
                         .script(r#"ctx._source.title = "New Title""#)
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    Scripts can be configured with parameters:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: "1".to_owned(), title: String::new(), timestamp: Date::now() };
    let response = client.document::<MyType>()
                         .update(1)
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
    pub fn update(self, id: impl Into<Id<'static>>) -> UpdateRequestBuilder<TSender, Doc<TDocument>>
    where
        TDocument: DocumentType + StaticIndex + StaticType,
    {
        let index = TDocument::static_index().into();
        let ty = TDocument::static_ty().into();

        RequestBuilder::initial(
            self.inner,
            UpdateRequestInner {
                index: index,
                ty: ty,
                id: id.into(),
                body: Doc::empty(),
                _marker: PhantomData,
            },
        )
    }
}

impl<TSender> DocumentClient<TSender, ()>
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

    Update a document with an id of `1` using a new document value:

    ```no_run
    # #[macro_use] extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document()
                         .update_raw("myindex", 1)
                         .doc(json!({
                             "title": "New Title"
                         }))
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```
    */
    pub fn update_raw(
        self,
        index: impl Into<Index<'static>>,
        id: impl Into<Id<'static>>,
    ) -> UpdateRequestBuilder<TSender, Doc<()>> {
        RequestBuilder::initial(
            self.inner,
            UpdateRequestInner {
                index: index.into(),
                ty: DEFAULT_DOC_TYPE.into(),
                id: id.into(),
                body: Doc::empty(),
                _marker: PhantomData,
            },
        )
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
    /** Set the index for the update request. */
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.inner.index = index.into();
        self
    }

    /** Set the type for the update request. */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
        self.inner.ty = ty.into();
        self
    }

    /**
    Update the source using a document.

    # Examples

    Update a [`DocumentType`][documents-mod] called `MyType` with an id of `1` using a new document value:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: "1".to_owned(), title: String::new(), timestamp: Date::now() };
    let response = client.document::<MyType>()
                         .update(1)
                         .doc(new_doc)
                         .send();
    # Ok(())
    # }
    ```

    The document doesn't necessarily need to be of the same type so partial updates can be made.
    Be careful though because this can lead to unexpected results or runtime errors if the document types don't align:

    ```no_run
    # #[macro_use] extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<MyType>()
                         .update(1)
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
        TDocument: Serialize,
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<MyType>()
                         .update(1)
                         .script(r#"ctx._source.title = "New Title""#)
                         .send();
    # Ok(())
    # }
    ```

     Update the `title` property of a document using a parameterised script:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::client::requests::document_update::ScriptBuilder;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let script = ScriptBuilder::new("ctx._source.title = params.newTitle")
        .param("newTitle", "New Title");

    let response = client.document::<MyType>()
                         .update(1)
                         .script(script)
                         .send()?;

    assert!(response.updated());
    # Ok(())
    # }
    ```

    [painless-lang]: https://www.elastic.co/guide/en/elasticsearch/reference/current/modules-scripting-painless.html
     */
    pub fn script<TScript, TParams>(
        self,
        builder: TScript,
    ) -> UpdateRequestBuilder<TSender, Script<TParams>>
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: "1".to_owned(), title: String::new(), timestamp: Date::now() };
    let response = client.document::<MyType>()
                         .update(1)
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
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    # let new_doc = MyType { id: "1".to_owned(), title: String::new(), timestamp: Date::now() };
    #[derive(Serialize)]
    struct MyParams {
        title: &'static str
    }

    let response = client.document::<MyType>()
                         .update(1)
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
    pub fn script_fluent<TScript, TParams>(
        self,
        source: TScript,
        builder: impl FnOnce(ScriptBuilder<DefaultParams>) -> ScriptBuilder<TParams>,
    ) -> UpdateRequestBuilder<TSender, Script<TParams>>
    where
        TScript: ToString,
    {
        let builder = builder(ScriptBuilder::new(source));

        self.script(builder)
    }
}

#[cfg(all(test, feature="sync_sender"))]
mod tests {
    use super::ScriptBuilder;
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };
    use serde_json::{
        self,
        Value,
    };

    #[derive(Serialize, ElasticType)]
    #[elastic(crate_root = "crate::types")]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .update("1")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/_doc/1/_update", req.url.as_ref());

        let expected_body = json!({
            "doc": {}
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .update("1")
            .index("new-idx")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/new-idx/_doc/1/_update", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .update("1")
            .ty("new-ty")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/new-ty/1/_update", req.url.as_ref());
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
            .document::<TestDoc>()
            .update("1")
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
            .document::<TestDoc>()
            .update("1")
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
            .document::<TestDoc>()
            .update("1")
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
            .document::<TestDoc>()
            .update("1")
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
            .document::<TestDoc>()
            .update("1")
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
