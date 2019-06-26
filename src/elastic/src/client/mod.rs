/*!
HTTP client, requests and responses.

This module contains the HTTP client, as well as request and response types.

# The gist

`elastic` provides two clients:

- [`SyncClient`][SyncClient] for making synchronous requests
- [`AsyncClient`][AsyncClient] for making asynchronous requests using the [`tokio`][tokio] stack.

## Building a synchronous client

Use a [`SyncClientBuilder`][SyncClientBuilder] to configure a synchronous client.

```
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
let client = SyncClient::builder().build()?;
# Ok(())
# }
```

Requests on the synchronous client will block the current thread until a response is received.
The response is returned as a `Result`.

## Building an asynchronous client

Use an [`AsyncClientBuilder`][AsyncClientBuilder] to configure an asynchronous client.

```
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
let client = AsyncClient::builder().build()?;
# Ok(())
# }
```

Requests on the asynchronous client won't block the current thread.
Instead a `Future` will be returned immediately that will resolve to a response at a later point.

## Sending requests

Requests can be sent with an instance of a client using a builder API:

```no_run
# #[macro_use] extern crate serde_json;
# use serde_json::Value;
# use elastic::prelude::*;
# use elastic::Error;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = SyncClient::builder().build()?;
let response = client.search::<Value>()
                     .index("myindex")
                     .ty("myty")
                     .body(json!({
                         "query": {
                             "query_string": {
                                 "query": "*"
                             }
                         }
                     }))
                     .send();

match response {
    Ok(response) => {
        // Iterate through the response hits
        for hit in response.hits() {
            println!("{:?}", hit);
        }
    },
    Err(Error::Api(e)) => {
        // handle a REST API error
    },
    Err(e) => {
        // handle a HTTP or JSON error
    }
}
# Ok(())
# }
```

Requests that work with [document types][documents-mod] can infer index and type metadata:

```no_run
# #[macro_use] extern crate serde_json;
# #[macro_use] extern crate elastic_derive;
# #[macro_use] extern crate serde_derive;
# use serde_json::Value;
# use elastic::prelude::*;
# use elastic::Error;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = SyncClient::builder().build()?;
# #[derive(ElasticType, Deserialize, Debug)]
# struct MyType { }
let response = client.document::<MyType>()
                     .search()
                     .body(json!({
                         "query": {
                             "query_string": {
                                 "query": "*"
                             }
                         }
                     }))
                     .send();

match response {
    Ok(response) => {
        // Iterate through the response hits
        for hit in response.hits() {
            println!("{:?}", hit);
        }
    },
    Err(Error::Api(e)) => {
        // handle a REST API error
    },
    Err(e) => {
        // handle a HTTP or JSON error
    }
}
# Ok(())
# }
```

`SyncClient` and `AsyncClient` offer the same request methods.
The details are explained below.

# Request builders

Some commonly used endpoints have high-level builder methods you can use to configure requests easily.
They're exposed as methods on the `Client`:

Client method                                                 | Elasticsearch API                  | Raw request type                                        | Response type
------------------------------------------------------------- | ---------------------------------- | ------------------------------------------------------- | ------------------------------------
[`search`][Client.search]                                     | [Search][docs-search]              | [`SearchRequest`][SearchRequest]                        | [`SearchResponse`][SearchResponse]
[`bulk`][Client.bulk]                                         | [Bulk][docs-bulk]                  | [`BulkRequest`][BulkRequest]                            | [`BulkResponse`][BulkResponse]
[`ping`][Client.ping]                                         | -                                  | [`PingRequest`][PingRequest]                            | [`PingResponse`][PingResponse]
[`document.search`][Client.document.search]                   | [Search][docs-search]              | [`SearchRequest`][SearchRequest]                        | [`SearchResponse`][SearchResponse]
[`document.get`][Client.document.get]                         | [Get Document][docs-get]           | [`GetRequest`][GetRequest]                              | [`GetResponse`][GetResponse]
[`document.index`][Client.document.index]                     | [Index Document][docs-index]       | [`IndexRequest`][IndexRequest]                          | [`IndexResponse`][IndexResponse]
[`document.update`][Client.document.update]                   | [Update Document][docs-update]     | [`UpdateRequest`][UpdateRequest]                        | [`UpdateResponse`][UpdateResponse]
[`document.delete`][Client.document.delete]                   | [Delete Document][docs-delete]     | [`DeleteRequest`][DeleteRequest]                        | [`DeleteResponse`][DeleteResponse]
[`document.put_mapping`][Client.document.put_mapping]         | [Put Mapping][docs-mapping]        | [`IndicesPutMappingRequest`][IndicesPutMappingRequest]  | [`CommandResponse`][CommandResponse]
[`index.create`][Client.index.create]                         | [Create Index][docs-create-index]  | [`IndicesCreateRequest`][IndicesCreateRequest]          | [`CommandResponse`][CommandResponse]
[`index.open`][Client.index.open]                             | [Open Index][docs-open-index]      | [`IndicesOpenRequest`][IndicesOpenRequest]              | [`CommandResponse`][CommandResponse]
[`index.close`][Client.index.close]                           | [Close Index][docs-close-index]    | [`IndicesCloseRequest`][IndicesCloseRequest]            | [`CommandResponse`][CommandResponse]
[`index.delete`][Client.index.delete]                         | [Delete Index][docs-delete-index]  | [`IndicesDeleteRequest`][IndicesDeleteRequest]          | [`CommandResponse`][CommandResponse]
[`index.exists`][Client.index.exists]                         | [Index Exists][docs-index-exists]  | [`IndicesExistsRequest`][IndicesExistsRequest]          | [`IndicesExistsResponse`][IndicesExistsResponse]

All builders follow a standard pattern:

- The `Client` method takes all required parameters without type inference
- Optional or inferred parameters can be overridden in builder methods with type inference
- `send` will return a specific response type

The high-level request builders are wrappers around the [`Client.request`][Client.request] method, taking a [raw request type][endpoints-mod].
For example, a `get` request for an anonymous json value:

```no_run
# #[macro_use] extern crate serde_json;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = SyncClient::builder().build()?;
let response = client.document::<Value>().get_raw("values", 1).send()?;
# Ok(())
# }
```

is equivalent to:

```no_run
# #[macro_use] extern crate serde_json;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = SyncClient::builder().build()?;
let response = client.request(GetRequest::for_index_ty_id("values", "value", 1))
                     .send()?
                     .into_response::<GetResponse<Value>>()?;
# Ok(())
# }
```

# Raw request types

Not all endpoints have strongly-typed builders, but all Elasticsearch API endpoints have a specific [raw request type][endpoints-mod] that can be used to build a request manually and send with the [`Client.request`][Client.Request] method.
The builders described above are just wrappers around these request types, but that doesn't mean raw requests are a second-class API.
You have more control over how requests are serialised, sent and deserialised using the raw requests API.
All request endpoints live in the [`endpoints`][endpoints-mod] module.

The process of sending raw requests is described in more detail below.

## The raw request process

The pieces involved in sending an Elasticsearch API request and parsing the response are modular.
Each one exposes Rust traits you can implement to support your own logic but if you just want to send a search/get request and parse a search/get response then you won't need to worry about this so much.

The basic flow from request to response is:

**1)** Turn a concrete [request type][endpoints-mod] into a [`RawRequestBuilder`][RawRequestBuilder]:

```text
[RequestType] ---> [Client.request()] ---> [RawRequestBuilder]
```

**2)** Send the [`RawRequestBuilder`][RawRequestBuilder] and get a response builder:

```text
[RawRequestBuilder.send()] ---> [ResponseBuilder]
```

**3)** Parse the response builder to a [response type][response-types]:

```text
[ResponseBuilder.into_response()] ---> [ResponseType]
```

The example below shows how these pieces fit together in code  by sending a simple synchronous `SearchRequest`, with the steps in the above process labelled:

```no_run
# #[macro_use] extern crate serde_json;
# use elastic::prelude::*;
# use serde_json::Value;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = SyncClient::builder().build()?;
let req = SearchRequest::for_index("_all", empty_body());

let response = client.request(req) // 1
                     .send()? // 2
                     .into_response::<SearchResponse<Value>>()?; // 3
# Ok(())
# }
```

### 1. Building raw requests

The [`endpoints`][endpoints-mod] module contains code-generated request types for the Elasticsearch REST API.
Each request type expects its parameters upfront and is generic over the request body.

A raw search request:

```no_run
# #[macro_use] extern crate serde_json;
# use elastic::prelude::*;
# fn main() {
let req = {
    let body = json!({
        "query": {
            "query_string": {
                "query": "*"
            }
        }
    });

    SearchRequest::for_index_ty("myindex", "myty", body)
};
# }
```

A raw request to index a document:

```no_run
# #[macro_use] extern crate serde_json;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let doc = true;
let req = {
    let body = serde_json::to_string(&doc)?;

    IndexRequest::for_index_ty_id("myindex", "myty", 1, body)
};
# Ok(())
# }
```

### 2. Sending requests

Both high-level request builders and raw requests have some common builder methods:

- [`params`][RequestBuilder.params] for setting url query parameters
- a `send` method for sending the request.
For high-level requests this returns a strongly-typed response.
For raw requests this returns a response builder.
If the request was sent synchronously, the response is returned as a `Result`.
If the request was sent asynchronously, the response is returned as a `Future`.

```no_run
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = SyncClient::builder().build()?;
# let req = PingRequest::new();
let request_builder = client.request(req);

// Set additional url parameters
let request_builder = request_builder.params_fluent(|p| p
    .url_param("pretty", true)
    .url_param("refresh", true)
);

// Send the request
let response = request_builder.send();
# Ok(())
# }
```

### 3. Parsing responses synchronously

Call [`SyncResponseBuilder.into_response`][SyncResponseBuilder.into_response] on a sent request to get a [strongly typed response][response-types]:

```no_run
# #[macro_use] extern crate serde_json;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use serde_json::Value;
# use elastic::prelude::*;
# use elastic::Error;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: String,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let client = SyncClient::builder().build()?;
# let req = PingRequest::new();
let response = client.request(req)
                     .send()?
                     .into_response::<SearchResponse<Value>>();

match response {
    Ok(response) => {
        // Iterate through the response hits
        for hit in response.hits() {
            println!("{:?}", hit);
        }
    },
    Err(Error::Api(e)) => {
        // handle a REST API error
    },
    Err(e) => {
        // handle a HTTP or JSON error
    }
}
# Ok(())
# }
```

Alternatively, call [`SyncResponseBuilder.into_raw`][SyncResponseBuilder.into_raw] on a sent request to get a raw [`SyncHttpResponse`][SyncHttpResponse]:

```no_run
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use std::io::Read;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = SyncClient::builder().build()?;
# let req = PingRequest::new();
let mut response = client.request(req)
                         .send()?
                         .into_raw();

let mut body = String::new();
response.read_to_string(&mut body)?;

println!("{}", body);
# Ok(())
# }
```

`SyncHttpResponse` implements the standard `Read` trait so you can buffer out the raw response data.
For more details see the [`responses`][responses-mod] module.

### 3. Parsing responses asynchronously

Call [`AsyncResponseBuilder.into_response`][AsyncResponseBuilder.into_response] on a sent request to get a [strongly typed response][response-types]:

```no_run
# #[macro_use] extern crate serde_json;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use futures::Future;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: String,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let client = AsyncClient::builder().build()?;
# let req = PingRequest::new();
let future = client.request(req)
                   .send()
                   .and_then(|response| response.into_response::<SearchResponse<Value>>());

future.and_then(|response| {
    // Iterate through the response hits
    for hit in response.hits() {
        println!("{:?}", hit);
    }

    Ok(())
});
# Ok(())
# }
```

Alternatively, call [`AsyncResponseBuilder.into_raw`][AsyncResponseBuilder.into_raw] on a sent request to get a raw [`AsyncHttpResponse`][AsyncHttpResponse]:

```no_run
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# use std::str;
# use std::io::Read;
# use futures::{Future, Stream};
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
# let client = AsyncClient::builder().build()?;
# let req = PingRequest::new();
let future = client.request(req)
                   .send()
                   .and_then(|response| Ok(response.into_raw()))
                   .and_then(|raw| raw.concat2())
                   .map_err(|e| Box::new(e) as Box<dyn ::std::error::Error>);

future.and_then(|body| {
    let body = str::from_utf8(body.as_ref())?;

    println!("{}", body);

    Ok(())
});
# Ok(())
# }
```

`AsyncHttpResponse` implements the async `Stream` trait so you can buffer out the raw response data.
For more details see the [`responses`][responses-mod] module.

[docs-bulk]: http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html
[docs-search]: http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html
[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html
[docs-update]: http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-update.html
[docs-delete]: http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html
[docs-index]: https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html
[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/master/mapping.html
[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-create-index.html
[docs-close-index]: https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html
[docs-open-index]: https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html
[docs-index-exists]: https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html
[docs-delete-index]: https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-delete-index.html

[tokio]: https://tokio.rs

[endpoints-mod]: requests/endpoints/index.html
[RequestParams]: struct.RequestParams.html
[SyncClient]: type.SyncClient.html
[SyncClientBuilder]: struct.SyncClientBuilder.html
[AsyncClient]: type.AsyncClient.html
[AsyncClientBuilder]: struct.AsyncClientBuilder.html
[Client.request]: struct.Client.html#method.request
[Client.bulk]: struct.Client.html#bulk-request
[Client.search]: struct.Client.html#search-request
[Client.document.search]: struct.DocumentClient.html#search-request
[Client.document.get]: struct.DocumentClient.html#get-document-request
[Client.document.update]: struct.DocumentClient.html#update-document-request
[Client.document.delete]: struct.DocumentClient.html#delete-document-request
[Client.document.index]: struct.DocumentClient.html#index-document-request
[Client.document.put_mapping]: struct.DocumentClient.html#method.put_mapping
[Client.index.create]: struct.IndexClient.html#create-index-request
[Client.index.open]: struct.IndexClient.html#open-index-request
[Client.index.close]: struct.IndexClient.html#close-index-request
[Client.index.delete]: struct.IndexClient.html#delete-index-request
[Client.index.exists]: struct.IndexClient.html#index-exists-request
[Client.ping]: struct.Client.html#ping-request

[RequestBuilder]: requests/struct.RequestBuilder.html
[RequestBuilder.params]: requests/struct.RequestBuilder.html#method.params
[RawRequestBuilder]: requests/type.RawRequestBuilder.html
[SearchRequest]: requests/endpoints/struct.SearchRequest.html
[BulkRequest]: requests/endpoints/struct.BulkRequest.html
[GetRequest]: requests/endpoints/struct.GetRequest.html
[UpdateRequest]: requests/endpoints/struct.UpdateRequest.html
[DeleteRequest]: requests/endpoints/struct.DeleteRequest.html
[IndexRequest]: requests/endpoints/struct.IndexRequest.html
[IndicesPutMappingRequest]: requests/endpoints/struct.IndicesPutMappingRequest.html
[IndicesCreateRequest]: requests/endpoints/struct.IndicesCreateRequest.html
[IndicesOpenRequest]: requests/endpoints/struct.IndicesOpenRequest.html
[IndicesCloseRequest]: requests/endpoints/struct.IndicesCloseRequest.html
[IndicesDeleteRequest]: requests/endpoints/struct.IndicesDeleteRequest.html
[IndicesExistsRequest]: requests/endpoints/struct.IndicesExistsRequest.html
[PingRequest]: requests/endpoints/struct.PingRequest.html

[responses-mod]: responses/index.html
[SyncResponseBuilder]: responses/struct.SyncResponseBuilder.html
[SyncResponseBuilder.into_response]: responses/struct.SyncResponseBuilder.html#method.into_response
[SyncResponseBuilder.into_raw]: responses/struct.SyncResponseBuilder.html#method.into_raw
[AsyncResponseBuilder]: responses/struct.AsyncResponseBuilder.html
[AsyncResponseBuilder.into_response]: responses/struct.AsyncResponseBuilder.html#method.into_response
[AsyncResponseBuilder.into_raw]: responses/struct.AsyncResponseBuilder.html#method.into_raw
[SearchResponse]: responses/struct.SearchResponse.html
[BulkResponse]: responses/struct.BulkResponse.html
[GetResponse]: responses/struct.GetResponse.html
[UpdateResponse]: responses/struct.UpdateResponse.html
[DeleteResponse]: responses/struct.DeleteResponse.html
[IndexResponse]: responses/struct.IndexResponse.html
[IndicesExistsResponse]: responses/struct.IndicesExistsResponse.html
[PingResponse]: responses/struct.PingResponse.html
[CommandResponse]: responses/struct.CommandResponse.html
[SyncHttpResponse]: responses/struct.SyncHttpResponse.html
[AsyncHttpResponse]: responses/struct.AsyncHttpResponse.html
[response-types]: responses/parse/trait.IsOk.html#implementors

[documents-mod]: ../types/documents/index.html
*/

pub mod requests;
pub mod responses;

mod asynchronous;
mod synchronous;

pub use self::{
    asynchronous::*,
    synchronous::*,
};

#[doc(inline)]
pub use crate::http::sender::{
    PreRequestParams,
    RequestParams,
};

use crate::{
    http::sender::{
        NodeAddresses,
        Sender,
    },
    params::Index,
};

use std::marker::PhantomData;

/**
A HTTP client for the Elasticsearch REST API.

The `Client` is a structure that lets you create and send request builders.
`Client` is generic over a `Sender`, but rather than use `Client` directly, use one of:

- [`SyncClient`][SyncClient]
- [`AsyncClient`][AsyncClient]

# Examples

Create a synchronous `Client` and send a ping request:

```no_run
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
let client = SyncClient::builder().build()?;

let response = client.request(PingRequest::new())
                     .send()?
                     .into_response::<PingResponse>()?;
# Ok(())
# }
```

Create an asynchronous `Client` and send a ping request:

```no_run
# use futures::Future;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<dyn ::std::error::Error>> {
let client = AsyncClient::builder().build()?;

let response_future = client.request(PingRequest::new())
                            .send()
                            .and_then(|res| res.into_response::<PingResponse>());

tokio::runtime::current_thread::block_on_all(response_future)?;
# Ok(())
# }
```

[SyncClient]: type.SyncClient.html
[AsyncClient]: type.AsyncClient.html
*/
#[derive(Clone)]
pub struct Client<TSender> {
    sender: TSender,
    addresses: NodeAddresses<TSender>,
}

impl<TSender> Client<TSender>
where
    TSender: Sender,
{
    /**
    Get a client for working with specific document type.

    The document type can provide extra metadata like index and type names
    that can be used to simplify other API methods.
    */
    pub fn document<TDocument>(&self) -> DocumentClient<TSender, TDocument> {
        DocumentClient {
            inner: (*self).clone(),
            _m: Default::default(),
        }
    }

    /**
    Get a client for working with a specific index.
    */
    pub fn index(&self, index: impl Into<Index<'static>>) -> IndexClient<TSender> {
        IndexClient {
            inner: (*self).clone(),
            index: index.into(),
        }
    }
}

/**
A [`Client`] for a specific document type.

[`Client`]: struct.Client.html
*/
#[derive(Clone)]
pub struct DocumentClient<TSender, TDocument> {
    inner: Client<TSender>,
    _m: PhantomData<TDocument>,
}

/**
A [`Client`] for a specific index.

[`Client`]: struct.Client.html
*/
#[derive(Clone)]
pub struct IndexClient<TSender> {
    inner: Client<TSender>,
    index: Index<'static>,
}

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::{
        requests::prelude::*,
        responses::prelude::*,
        AsyncClient,
        AsyncClientBuilder,
        PreRequestParams,
        RequestParams,
        SyncClient,
        SyncClientBuilder,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn client_is_send_sync() {
        assert_send::<SyncClient>();
        assert_sync::<SyncClient>();

        assert_send::<AsyncClient>();
        assert_sync::<AsyncClient>();
    }
}
