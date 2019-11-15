/*!
Builders for [put mapping requests][docs-mapping].

[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
*/

use serde_json;
use std::marker::PhantomData;

use crate::{
    client::{
        requests::{
            RequestInner,
            RequestBuilder,
        },
        responses::CommandResponse,
        DocumentClient,
    },
    endpoints::IndicesPutMappingRequest,
    error::{
        self,
        Error,
    },
    http::sender::Sender,
    params::{
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

/**
A [put mapping request][docs-mapping] builder that can be configured before sending.

Call [`Client.document.put_mapping`][Client.document.put_mapping] to get a `PutMappingRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document.put_mapping]: ../../struct.DocumentClient.html#put-mapping-request
*/
pub type PutMappingRequestBuilder<TSender, TDocument> =
    RequestBuilder<TSender, PutMappingRequestInner<TDocument>>;

#[doc(hidden)]
pub struct PutMappingRequestInner<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    _marker: PhantomData<TDocument>,
}

impl<TDocument> RequestInner for PutMappingRequestInner<TDocument>
where
    TDocument: DocumentType,
{
    type Request = IndicesPutMappingRequest<'static, Vec<u8>>;
    type Response = CommandResponse;

    fn into_request(self) -> Result<Self::Request, Error> {
        let body = serde_json::to_vec(&TDocument::index_mapping()).map_err(error::request)?;

        if &self.ty[..] == DEFAULT_DOC_TYPE {
            Ok(IndicesPutMappingRequest::for_index(self.index, body))
        } else {
            Ok(IndicesPutMappingRequest::for_index_ty(
                self.index, self.ty, body,
            ))
        }
    }
}

/**
# Put mapping request
*/
impl<TSender, TDocument> DocumentClient<TSender, TDocument>
where
    TSender: Sender,
{
    /**
    Create a [`PutMappingRequestBuilder`][PutMappingRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]

    # Examples

    Put the document mapping for a [`DocumentType`][documents-mod] called `MyType`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document::<MyType>()
                         .put_mapping()
                         .send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [PutMappingRequestBuilder]: requests/document_put_mapping/type.PutMappingRequestBuilder.html
    [builder-methods]: requests/document_put_mapping/type.PutMappingRequestBuilder.html#builder-methods
    [send-sync]: requests/document_put_mapping/type.PutMappingRequestBuilder.html#send-synchronously
    [send-async]: requests/document_put_mapping/type.PutMappingRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn put_mapping(self) -> PutMappingRequestBuilder<TSender, TDocument>
    where
        TDocument: DocumentType + StaticIndex + StaticType,
    {
        let index = TDocument::static_index();
        let ty = TDocument::static_ty();

        RequestBuilder::initial(
            self.inner,
            PutMappingRequestInner {
                index,
                ty,
                _marker: PhantomData,
            },
        )
    }
}

/**
# Builder methods

Configure a `PutMappingRequestBuilder` before sending it.
*/
impl<TSender, TDocument> PutMappingRequestBuilder<TSender, TDocument>
where
    TSender: Sender,
{
    /** Set the index for the put mapping request. */
    pub fn index(mut self, index: impl Into<Index<'static>>) -> Self {
        self.inner.index = index.into();
        self
    }

    /** Set the type for the put mapping request. */
    pub fn ty(mut self, ty: impl Into<Type<'static>>) -> Self {
        self.inner.ty = ty.into();
        self
    }
}

#[cfg(all(test, feature="sync_sender"))]
mod tests {
    use crate::{
        client::requests::RequestInner,
        prelude::*,
    };
    use serde_json::{
        self,
        Value,
    };

    #[derive(ElasticType)]
    #[elastic(crate_root = "crate::types")]
    struct TestDoc {}

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .put_mapping()
            .inner
            .into_request()
            .unwrap();

        let expected_body = json!({
            "properties": {

            }
        });

        let actual_body: Value = serde_json::from_slice(&req.body).unwrap();

        assert_eq!("/testdoc/_mapping", req.url.as_ref());
        assert_eq!(expected_body.to_string(), actual_body.to_string());
    }

    #[test]
    fn specify_index() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .put_mapping()
            .index("new-idx")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/new-idx/_mapping", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document::<TestDoc>()
            .put_mapping()
            .ty("new-ty")
            .inner
            .into_request()
            .unwrap();

        assert_eq!("/testdoc/_mappings/new-ty", req.url.as_ref());
    }
}
