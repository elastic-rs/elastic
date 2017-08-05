use std::marker::PhantomData;
use serde::de::DeserializeOwned;

use error::*;
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{Index, Type, Id, GetRequest, RequestBuilder, RawRequestBuilder};
use client::responses::GetResponse;
use types::document::DocumentType;

/** 

A builder for a [`Client.get_document`][Client.get_document] request. 

[Client.get_document]: ../struct.Client.html#method.get_document
*/
pub struct GetRequestBuilder<TDocument> {
    index: Index<'static>,
    ty: Type<'static>,
    id: Id<'static>,
    _marker: PhantomData<TDocument>,
}

impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /** 
    Create a [`RequestBuilder` for a get request][RequestBuilder.get_document].

    # Examples

    Get a [`DocumentType`][documents-mod] called `MyType` with an id of `1`:
    
    ```no_run
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateFormat>
    # }
    # let client = ClientBuilder::new().build().unwrap();
    let response = client.get_document::<MyType>(index("myindex"), id(1))
                         .send()
                         .unwrap();

    if let Some(doc) = response.source {
        println!("id: {}", doc.id);
    }
    # }
    ```

    For more details on document types, see the [`types`][types-mod] module.

    Get the same document as a `serde_json::Value`:

    ```no_run
    # extern crate serde;
    # extern crate serde_json;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # let client = ClientBuilder::new().build().unwrap();
    let response = client.get_document::<Value>(index("myindex"), id(1))
                         .ty("mytype")
                         .send()
                         .unwrap();
    # }
    ```

    [RequestBuilder.get_document]: requests/struct.RequestBuilder.html#get-document-builder
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn get_document<TDocument>(&self,
                                       index: Index<'static>,
                                       id: Id<'static>)
                                       -> RequestBuilder<TSender, GetRequestBuilder<TDocument>>
        where TDocument: DeserializeOwned + DocumentType
    {
        let ty = TDocument::name().into();

        RequestBuilder::new(self.clone(),
                            None,
                            GetRequestBuilder {
                                index: index,
                                ty: ty,
                                id: id,
                                _marker: PhantomData,
                            })
    }
}

impl<TDocument> GetRequestBuilder<TDocument> {
    fn into_request(self) -> GetRequest<'static> {
        GetRequest::for_index_ty_id(self.index, self.ty, self.id)
    }
}

/** 
# Get document builder

A request builder for a [Get Document][docs-get] request.

Call [`Client.get_document`][Client.get_document] to get a `RequestBuilder` for a get document request.

[Client.get_document]: ../struct.Client.html#method.get_document
[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
*/
impl<TSender, TDocument> RequestBuilder<TSender, GetRequestBuilder<TDocument>>
    where TSender: Sender
{
    /** Set the type for the get request. */
    pub fn ty<I>(mut self, ty: I) -> Self
        where I: Into<Type<'static>>
    {
        self.inner.ty = ty.into();
        self
    }
}

impl<TDocument> RequestBuilder<SyncSender, GetRequestBuilder<TDocument>>
    where TDocument: DeserializeOwned + DocumentType
{
    /** Send the get request. */
    pub fn send(self) -> Result<GetResponse<TDocument>> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params, RawRequestBuilder::new(req))
            .send()?
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

        let req = client
            .get_document::<Value>(index("test-idx"), id("1"))
            .req
            .into_request();

        assert_eq!("/test-idx/value/1", req.url.as_ref());
    }

    #[test]
    fn specify_ty() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .get_document::<Value>(index("test-idx"), id("1"))
            .ty("new-ty")
            .req
            .into_request();

        assert_eq!("/test-idx/new-ty/1", req.url.as_ref());
    }
}
