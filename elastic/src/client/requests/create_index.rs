use futures::Future;

use error::*;
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{empty_body, DefaultBody, Index, IndicesCreateRequest,
                       RequestBuilder};
use client::requests::raw::RawRequestInner;
use client::responses::CommandResponse;

/** 
A [create index request][docs-create-index] builder that can be configured before sending. 

Call [`Client.create_index`][Client.create_index] to get a `CreateIndexRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.create_index]: ../struct.Client.html#create-index-request
*/
pub type CreateIndexRequestBuilder<TSender, TBody> = RequestBuilder<TSender, CreateIndexRequestInner<TBody>>;

#[doc(hidden)]
pub struct CreateIndexRequestInner<TBody> {
    index: Index<'static>,
    body: TBody,
}

/**
# Create index request
*/
impl<TSender> Client<TSender> 
    where TSender: Sender
{
    /** 
    Create a [`CreateIndexRequestBuilder`][CreateIndexRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]
    
    # Examples
    
    Create an index called `myindex`:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = ClientBuilder::new().build().unwrap();
    let my_index = index("myindex");

    let response = client.create_index(my_index).send().unwrap();

    assert!(response.acknowledged);
    ```

    Create an index with settings and document mappings for a [`DocumentType`][documents-mod] called `MyType`:

    ```no_run
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # #[macro_use] extern crate serde_json;
    # extern crate elastic;
    # use elastic::prelude::*;
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # fn main() {
    # let client = ClientBuilder::new().build().unwrap();
    let my_index = index("myindex");

    let body = json!({
        "settings": {
            "index": {
                "number_of_shards": 3,
                "number_of_replicas": 2
            }
        },
        "mappings": {
            MyType::name(): IndexDocumentMapping::from(MyType::mapping())
        }
    });

    let response = client.create_index(my_index)
                         .body(body.to_string())
                         .send()
                         .unwrap();

    assert!(response.acknowledged);
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [CreateIndexRequestBuilder]: requests/type.CreateIndexRequestBuilder.html
    [builder-methods]: requests/type.CreateIndexRequestBuilder.html#builder-methods
    [send-sync]: requests/type.CreateIndexRequestBuilder.html#send-synchronously
    [send-async]: requests/type.CreateIndexRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn create_index(&self, index: Index<'static>) -> CreateIndexRequestBuilder<TSender, DefaultBody> {
        RequestBuilder::new(self.clone(),
                            None,
                            CreateIndexRequestInner {
                                index: index,
                                body: empty_body(),
                            })
    }
}

impl<TBody> CreateIndexRequestInner<TBody> {
    fn into_request(self) -> IndicesCreateRequest<'static, TBody> {
        IndicesCreateRequest::for_index(self.index, self.body)
    }
}

/** 
# Builder methods

Configure a `CreateIndexRequestBuilder` before sending it.
*/
impl<TSender, TBody> CreateIndexRequestBuilder<TSender, TBody>
    where TSender: Sender,
          TBody: Into<TSender::Body>
{
    /** 
    Set the body for the create index request.
    
    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self,
                          body: TNewBody)
                          -> CreateIndexRequestBuilder<TSender, TNewBody>
        where TNewBody: Into<TSender::Body>
    {
        RequestBuilder::new(self.client,
                            self.params,
                            CreateIndexRequestInner {
                                index: self.inner.index,
                                body: body,
                            })
    }
}

/**
# Send synchronously
*/
impl<TBody> CreateIndexRequestBuilder<SyncSender, TBody>
    where TBody: Into<<SyncSender as Sender>::Body>
{
    /**
    Send a `CreateIndexRequestBuilder` synchronously using a [`SyncClient`]().

    This will block the current thread until a response arrives and is deserialised.
    */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TBody> CreateIndexRequestBuilder<AsyncSender, TBody>
    where TBody: Into<<AsyncSender as Sender>::Body>
{
    /**
    Send a `CreateIndexRequestBuilder` asynchronously using an [`AsyncClient`]().
    
    This will return a future that will resolve to the deserialised command response.
    */
    pub fn send(self) -> Box<Future<Item = CommandResponse, Error = Error>> {
        let req = self.inner.into_request();

        let res_future = RequestBuilder::new(self.client, self.params, RawRequestInner::new(req))
            .send()
            .and_then(|res| res.into_response());

        Box::new(res_future)
    }
}

#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn default_request() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client.create_index(index("testindex")).req.into_request();

        assert_eq!("/testindex", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = client
            .create_index(index("testindex"))
            .body("{}")
            .req
            .into_request();

        assert_eq!("{}", req.body);
    }
}
