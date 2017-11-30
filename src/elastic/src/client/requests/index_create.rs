/*!
Builders for [create index requests][docs-create-index].

[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
*/

use futures::{Future, Poll};

use error::*;
use client::Client;
use client::sender::{AsyncSender, Sender, SyncSender};
use client::requests::{empty_body, DefaultBody, RequestBuilder};
use client::requests::params::Index;
use client::requests::endpoints::IndicesCreateRequest;
use client::requests::raw::RawRequestInner;
use client::responses::CommandResponse;

/** 
A [create index request][docs-create-index] builder that can be configured before sending. 

Call [`Client.index_create`][Client.index_create] to get an `IndexCreateRequestBuilder`.
The `send` method will either send the request [synchronously][send-sync] or [asynchronously][send-async], depending on the `Client` it was created from.

[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html
[send-sync]: #send-synchronously
[send-async]: #send-asynchronously
[Client.index_create]: ../../struct.Client.html#create-index-request
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
where
    TSender: Sender,
{
    /** 
    Create an [`IndexCreateRequestBuilder`][IndexCreateRequestBuilder] with this `Client` that can be configured before sending.

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
    let body = json!({
        "settings": {
            "index": {
                "number_of_shards": 3,
                "number_of_replicas": 2
            }
        },
        "mappings": {
            MyType::name(): MyType::index_mapping()
        }
    });

    let response = client.index_create(index("myindex"))
                         .body(body.to_string())
                         .send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    For more details on document types and mapping, see the [`types`][types-mod] module.

    [IndexCreateRequestBuilder]: requests/index_create/type.IndexCreateRequestBuilder.html
    [builder-methods]: requests/index_create/type.IndexCreateRequestBuilder.html#builder-methods
    [send-sync]: requests/index_create/type.IndexCreateRequestBuilder.html#send-synchronously
    [send-async]: requests/index_create/type.IndexCreateRequestBuilder.html#send-asynchronously
    [types-mod]: ../types/index.html
    [documents-mod]: ../types/document/index.html
    */
    pub fn index_create(&self, index: Index<'static>) -> IndexCreateRequestBuilder<TSender, DefaultBody> {
        RequestBuilder::initial(
            self.clone(),
            IndexCreateRequestInner {
                index: index,
                body: empty_body(),
            },
        )
    }
}

impl<TBody> IndexCreateRequestInner<TBody> {
    fn into_request(self) -> IndicesCreateRequest<'static, TBody> {
        IndicesCreateRequest::for_index(self.index, self.body)
    }
}

/** 
# Builder methods

Configure an `IndexCreateRequestBuilder` before sending it.
*/
impl<TSender, TBody> IndexCreateRequestBuilder<TSender, TBody>
where
    TSender: Sender,
    TBody: Into<TSender::Body>,
{
    /** 
    Set the body for the create index request.
    
    If no body is specified then an empty query will be used.
    */
    pub fn body<TNewBody>(self, body: TNewBody) -> IndexCreateRequestBuilder<TSender, TNewBody>
    where
        TNewBody: Into<TSender::Body>,
    {
        RequestBuilder::new(
            self.client,
            self.params_builder,
            IndexCreateRequestInner {
                index: self.inner.index,
                body: body,
            },
        )
    }
}

/**
# Send synchronously
*/
impl<TBody> IndexCreateRequestBuilder<SyncSender, TBody>
where
    TBody: Into<<SyncSender as Sender>::Body> + 'static,
{
    /**
    Send an `IndexCreateRequestBuilder` synchronously using a [`SyncClient`][SyncClient].

    This will block the current thread until a response arrives and is deserialised.

    # Examples
    
    Create an index called `myindex`:
    
    ```no_run
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.index_create(index("myindex")).send()?;

    assert!(response.acknowledged());
    # Ok(())
    # }
    ```

    [SyncClient]: ../../type.SyncClient.html
    */
    pub fn send(self) -> Result<CommandResponse> {
        let req = self.inner.into_request();

        RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()?
            .into_response()
    }
}

/**
# Send asynchronously
*/
impl<TBody> IndexCreateRequestBuilder<AsyncSender, TBody>
where
    TBody: Into<<AsyncSender as Sender>::Body> + 'static,
{
    /**
    Send an `IndexCreateRequestBuilder` asynchronously using an [`AsyncClient`][AsyncClient].
    
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
    let future = client.index_create(index("myindex")).send();

    future.and_then(|response| {
        assert!(response.acknowledged());

        Ok(())
    });
    # Ok(())
    # }
    ```

    [AsyncClient]: ../../type.AsyncClient.html
    */
    pub fn send(self) -> Pending {
        let req = self.inner.into_request();

        let res_future = RequestBuilder::new(self.client, self.params_builder, RawRequestInner::new(req))
            .send()
            .and_then(|res| res.into_response());

        Pending::new(res_future)
    }
}

/** A future returned by calling `send`. */
pub struct Pending {
    inner: Box<Future<Item = CommandResponse, Error = Error>>,
}

impl Pending {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = CommandResponse, Error = Error> + 'static,
    {
        Pending {
            inner: Box::new(fut),
        }
    }
}

impl Future for Pending {
    type Item = CommandResponse;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
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
            .inner
            .into_request();

        assert_eq!("{}", req.body);
    }
}
