use std::marker::PhantomData;
use serde_json;
use futures::{Future, IntoFuture};
use serde::Serialize;

use error::{self, Result, Error};
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{Index, Type, IndicesPutMappingRequest, RequestBuilder};
use client::requests::raw::RawRequestInner;
use client::responses::CommandResponse;
use types::document::{FieldType, DocumentType, IndexDocumentMapping};

/** 
A [put mapping request][docs-mapping] builder that can be configured before sending.

Call [`Client.put_mapping`][Client.put_mapping] to get a `PutMappingRequestBuilder`. 
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.put_mapping]: ../struct.Client.html#put-mapping-request
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
    # fn main() {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = ClientBuilder::new().build().unwrap();
    client.put_mapping::<MyType>(index("myindex"))
          .send()
          .unwrap();
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
    pub fn put_mapping<TDocument>(&self,
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
    fn into_request(self) -> Result<IndicesPutMappingRequest<'static, Vec<u8>>> {
        let body = serde_json::to_vec(&IndexDocumentMapping::from(TDocument::mapping())).map_err(|e| error::request(e))?;

        Ok(IndicesPutMappingRequest::for_index_ty(self.index, self.ty, body))
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
    Send a `PutMappingRequestBuilder` synchronously using a [`SyncClient`]().

    This will block the current thread until a response arrives and is deserialised.
    */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.inner.into_request()?;

        RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TDocument> PutMappingRequestBuilder<AsyncSender, TDocument>
    where TDocument: DocumentType + 'static
{
    /**
    Send a `PutMappingRequestBuilder` asynchronously using an [`AsyncClient`]().
    
    This will return a future that will resolve to the deserialised command response.
    */
    pub fn send(self) -> Box<Future<Item = CommandResponse, Error = Error>> {
        let (client, params) = (self.client, self.params);

        let req_future = self.inner.into_request().into_future();

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
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .put_mapping::<Value>(index("test-idx"))
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/_mappings/value", req.url.as_ref());
        assert_eq!(r#"{"properties":{}}"#.as_bytes().to_vec(), req.body);
    }

    #[test]
    fn specify_ty() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .put_mapping::<Value>(index("test-idx"))
            .ty("new-ty")
            .req
            .into_request()
            .unwrap();

        assert_eq!("/test-idx/_mappings/new-ty", req.url.as_ref());
    }
}
