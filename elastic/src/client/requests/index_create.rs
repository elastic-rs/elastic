use futures::Future;

use error::*;
use client::{Client, Sender, SyncSender, AsyncSender};
use client::requests::{empty_body, DefaultBody, Index, IndicesCreateRequest,
                       RequestBuilder};
use client::requests::raw::RawRequestInner;
use client::responses::CommandResponse;

/** 
A [create index request][docs-create-index] builder that can be configured before sending. 

Call [`Client.index_create`][Client.index_create] to get a `IndexCreateRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.index_create]: ../struct.Client.html#create-index-request
*/
pub type IndexCreateRequestBuilder<TSender, TBody> = RequestBuilder<TSender, IndexCreateRequestInner<TBody>>;

#[doc(hidden)]
pub struct IndexCreateRequestInner<TBody> {
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
    Create a [`IndexCreateRequestBuilder`][IndexCreateRequestBuilder] with this `Client` that can be configured before sending.

    For more details, see:

    - [builder methods][builder-methods]
    - [send synchronously][send-sync]
    - [send asynchronously][send-async]
    
    # Examples
    
    Create an index called `myindex`:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let my_index = index("myindex");

    let response = client.index_create(my_index).send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
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
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
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

    let response = client.index_create(my_index)
                         .body(body.to_string())
                         .send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [IndexCreateRequestBuilder]: requests/type.IndexCreateRequestBuilder.html
    [builder-methods]: requests/type.IndexCreateRequestBuilder.html#builder-methods
    [send-sync]: requests/type.IndexCreateRequestBuilder.html#send-synchronously
    [send-async]: requests/type.IndexCreateRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn index_create(&self, index: Index<'static>) -> IndexCreateRequestBuilder<TSender, DefaultBody> {
        RequestBuilder::new(self.clone(),
                            None,
                            IndexCreateRequestInner {
                                index: index,
                                body: empty_body(),
                            })
    }
}

impl<TBody> IndexCreateRequestInner<TBody> {
    fn into_request(self) -> IndicesCreateRequest<'static, TBody> {
        IndicesCreateRequest::for_index(self.index, self.body)
    }
}

/** 
# Builder methods

Configure a `IndexCreateRequestBuilder` before sending it.
*/
impl<TSender, TBody> IndexCreateRequestBuilder<TSender, TBody>
    where TSender: Sender,
          TBody: Into<TSender::Body>
{
    /** 
    Set the body for the create index request.
    
    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self,
                          body: TNewBody)
                          -> IndexCreateRequestBuilder<TSender, TNewBody>
        where TNewBody: Into<TSender::Body>
    {
        RequestBuilder::new(self.client,
                            self.params,
                            IndexCreateRequestInner {
                                index: self.inner.index,
                                body: body,
                            })
    }
}

/**
# Send synchronously
*/
impl<TBody> IndexCreateRequestBuilder<SyncSender, TBody>
    where TBody: Into<<SyncSender as Sender>::Body>
{
    /**
    Send a `IndexCreateRequestBuilder` synchronously using a [`SyncClient`]().

    This will block the current thread until a response arrives and is deserialised.

    # Examples
    
    Create an index called `myindex`:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let my_index = index("myindex");

    let response = client.index_create(my_index).send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```
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
impl<TBody> IndexCreateRequestBuilder<AsyncSender, TBody>
    where TBody: Into<<AsyncSender as Sender>::Body>
{
    /**
    Send a `IndexCreateRequestBuilder` asynchronously using an [`AsyncClient`]().
    
    This will return a future that will resolve to the deserialised command response.

    # Examples
    
    Create an index called `myindex`:
    
    ```no_run
    # extern crate futures;
    # extern crate tokio_core;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    let my_index = index("myindex");

    let future = client.index_create(my_index).send();

    future.and_then(|response| {
        assert!(response.acknowledged());

        Ok(())
    });
    # Ok(())
    # }
    ```
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
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client.index_create(index("testindex")).inner.into_request();

        assert_eq!("/testindex", req.url.as_ref());
    }

    #[test]
    fn specify_body() {
        let client = SyncClientBuilder::new().build().unwrap();

        let req = client
            .index_create(index("testindex"))
            .body("{}")
            .inner.into_request();

        assert_eq!("{}", req.body);
    }
}
