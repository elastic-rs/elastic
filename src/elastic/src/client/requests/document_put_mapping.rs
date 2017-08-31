use std::marker::PhantomData;
use serde_json;
use futures::{Future, IntoFuture};
use futures_cpupool::CpuPool;
use serde::Serialize;

use error::{self, Result, Error};
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{Index, Type, IndicesPutMappingRequest, RequestBuilder};
use client::requests::raw::RawRequestInner;
use client::responses::CommandResponse;
use types::document::{FieldType, DocumentType, IndexDocumentMapping};

/** 
A [put mapping request][docs-mapping] builder that can be configured before sending.

Call [`Client.document_put_mapping`][Client.document_put_mapping] to get a `PutMappingRequestBuilder`. 
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.document_put_mapping]: ../struct.Client.html#put-mapping-request
*/
pub type PutMappingRequestBuilder<TSender, TDocument> = RequestBuilder<TSender, PutMappingRequestInner<TDocument>>;

pub struct PutMappingRequestInner<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    _marker: PhantomData<TDocument>,
}

/**
# Put mapping request
*/
impl<TSender> Client<TSender> 
    where TSender: Sender
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
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document_put_mapping::<MyType>(index("myindex"))
                         .send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [PutMappingRequestBuilder]: requests/type.PutMappingRequestBuilder.html
    [builder-methods]: requests/type.PutMappingRequestBuilder.html#builder-methods
    [send-sync]: requests/type.PutMappingRequestBuilder.html#send-synchronously
    [send-async]: requests/type.PutMappingRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn document_put_mapping<TDocument>(&self,
                                      index: Index<'static>)
                                      -> PutMappingRequestBuilder<TSender, TDocument>
        where TDocument: Serialize + DocumentType
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(self.clone(),
                            None,
                            PutMappingRequestInner {
                                index: index,
                                ty: ty,
                                _marker: PhantomData,
                            })
    }
}

impl<TDocument> PutMappingRequestInner<TDocument>
    where TDocument: DocumentType
{
    fn into_sync_request(self) -> Result<IndicesPutMappingRequest<'static, Vec<u8>>> {
        let body = serde_json::to_vec(&IndexDocumentMapping::from(TDocument::mapping())).map_err(error::request)?;

        Ok(IndicesPutMappingRequest::for_index_ty(self.index, self.ty, body))
    }
}

impl<TDocument> PutMappingRequestInner<TDocument>
    where TDocument: DocumentType + Send + 'static
{
    fn into_async_request(self, ser_pool: Option<CpuPool>) -> Box<Future<Item = IndicesPutMappingRequest<'static, Vec<u8>>, Error = Error>> {
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

Configure a `PutMappingRequestBuilder` before sending it.
*/
impl<TSender, TDocument> PutMappingRequestBuilder<TSender, TDocument>
    where TSender: Sender
{
    /** Set the type for the put mapping request. */
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.inner.ty = ty.into();
        self
    }
}

/**
# Send synchronously
*/
impl<TDocument> PutMappingRequestBuilder<SyncSender, TDocument>
    where TDocument: DocumentType
{
    /**
    Send a `PutMappingRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples

    Put the mapping for a document type called `MyType`:

    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.document_put_mapping::<MyType>(index("myindex"))
                         .send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [SyncClient]: ../type.SyncClient.html
    */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.inner.into_sync_request()?;

        RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TDocument> PutMappingRequestBuilder<AsyncSender, TDocument>
    where TDocument: DocumentType + Send + 'static
{
    /**
    Send a `PutMappingRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
    This will return a future that will resolve to the deserialised command response.

    # Examples

    Put the mapping for a document type called `MyType`:

    ```no_run
    # extern crate futures;
    # extern crate tokio_core;
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    let future = client.document_put_mapping::<MyType>(index("myindex"))
                       .send();

    future.and_then(|response| {
        assert!(response.acknowledged());

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: type.AsyncClient.html
    */
    pub fn send(self) -> Box<Future<Item = CommandResponse, Error = Error>> {
        let (client, params) = (self.client, self.params);

        let ser_pool = client.sender.serde_pool.clone();
        let req_future = self.inner.into_async_request(ser_pool);

        let res_future = req_future.and_then(move |req| {
            RequestBuilder::new(client, params, RawRequestInner::new(req))
            .send()
            .and_then(|res| res.into_response())
        });

        Box::new(res_future)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use prelude::*;

    #[test]
    fn default_request() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_put_mapping::<Value>(index("test-idx"))
            .inner.into_sync_request()
            .unwrap();

        assert_eq!("/test-idx/_mappings/value", req.url.as_ref());
        assert_eq!(r#"{"properties":{}}"#.as_bytes().to_vec(), req.body);
    }

    #[test]
    fn specify_ty() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .document_put_mapping::<Value>(index("test-idx"))
            .ty("new-ty")
            .inner.into_sync_request()
            .unwrap();

        assert_eq!("/test-idx/_mappings/new-ty", req.url.as_ref());
    }
}
