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
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let client = SyncClientBuilder::new().build()?;
# Ok(())
# }
```

Requests on the synchronous client will block the current thread until a response is received.
The response is returned as a `Result`.

## Building an asynchronous client

Use an [`AsyncClientBuilder`][AsyncClientBuilder] to configure an asynchronous client.

The asynchronous client requires a handle to a `tokio::reactor::Core`:

```
# extern crate tokio_core;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let core = tokio_core::reactor::Core::new()?;
let client = AsyncClientBuilder::new().build(&core.handle())?;
# Ok(())
# }
```

Requests on the asynchronous client won't block the current thread.
Instead a `Future` will be returned immediately that will resolve to a response at a later point.

## Sending requests

Requests can be sent with an instance of a client using a builder API:

```no_run
# #[macro_use] extern crate serde_json;
# extern crate elastic;
# use serde_json::Value;
# use elastic::prelude::*;
# use elastic::Error;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let client = SyncClientBuilder::new().build()?;
let response = client.search::<Value>()
                     .index("myindex")
                     .ty(Some("myty"))
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
[`document_get`][Client.document_get]                         | [Get Document][docs-get]           | [`GetRequest`][GetRequest]                              | [`GetResponse`][GetResponse]
[`document_index`][Client.document_index]                     | [Index Document][docs-index]       | [`IndexRequest`][IndexRequest]                          | [`IndexResponse`][IndexResponse]
[`document_put_mapping`][Client.document_put_mapping]         | [Put Mapping][docs-mapping]        | [`IndicesPutMappingRequest`][IndicesPutMappingRequest]  | [`CommandResponse`][CommandResponse]
[`index_create`][Client.index_create]                         | [Create Index][docs-create-index]  | [`IndicesCreateRequest`][IndicesCreateRequest]          | [`CommandResponse`][CommandResponse]

All builders follow a standard pattern:

- The `Client` method takes all required parameters without type inference
- Optional or inferred parameters can be overridden in builder methods with type inference
- `send` will return a specific response type

The high-level request builders are wrappers around the [`Client.request`][Client.request] method, taking a [raw request type][endpoints-mod].
For example, a `document_get` request for a value:

```no_run
# extern crate serde_json;
# extern crate elastic;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let client = SyncClientBuilder::new().build()?;
let response = client.document_get::<Value>(index("values"), id(1)).send()?;
# Ok(())
# }
```

is equivalent to:

```no_run
# extern crate serde_json;
# extern crate elastic;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let client = SyncClientBuilder::new().build()?;
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
# extern crate elastic;
# #[macro_use]
# extern crate json_str;
# extern crate serde_json;
# use elastic::prelude::*;
# use serde_json::Value;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let client = SyncClientBuilder::new().build()?;
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
# extern crate elastic;
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
# extern crate serde_json;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
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
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let client = SyncClientBuilder::new().build()?;
# let req = PingRequest::new();
let request_builder = client.request(req);

// Set additional url parameters
let request_builder = request_builder.params(|p| p
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
# extern crate serde;
# extern crate serde_json;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use serde_json::Value;
# use elastic::prelude::*;
# use elastic::Error;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let client = SyncClientBuilder::new().build()?;
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
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use std::io::Read;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let client = SyncClientBuilder::new().build()?;
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
# extern crate futures;
# extern crate tokio_core;
# extern crate serde;
# extern crate serde_json;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use futures::Future;
# use serde_json::Value;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# #[derive(Serialize, Deserialize, ElasticType)]
# struct MyType {
#     pub id: i32,
#     pub title: String,
#     pub timestamp: Date<DefaultDateMapping>
# }
# let core = tokio_core::reactor::Core::new()?;
# let client = AsyncClientBuilder::new().build(&core.handle())?;
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
# extern crate futures;
# extern crate tokio_core;
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# #[macro_use] extern crate elastic_derive;
# extern crate elastic;
# use std::str;
# use std::io::Read;
# use futures::{Future, Stream};
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let core = tokio_core::reactor::Core::new()?;
# let client = AsyncClientBuilder::new().build(&core.handle())?;
# let req = PingRequest::new();
let future = client.request(req)
                   .send()
                   .and_then(|response| Ok(response.into_raw()))
                   .and_then(|raw| raw.concat2())
                   .map_err(|e| Box::new(e) as Box<::std::error::Error>);

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

[docs-search]: http://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html
[docs-get]: http://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html
[docs-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html
[docs-mapping]: https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping.html
[docs-create-index]: https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-create-index.html

[tokio]: https://tokio.rs

[endpoints-mod]: requests/endpoints/index.html
[RequestParams]: struct.RequestParams.html
[SyncClient]: type.SyncClient.html
[SyncClientBuilder]: struct.SyncClientBuilder.html
[AsyncClient]: type.AsyncClient.html
[AsyncClientBuilder]: struct.AsyncClientBuilder.html
[Client.request]: struct.Client.html#method.request
[Client.search]: struct.Client.html#search-request
[Client.document_get]: struct.Client.html#get-document
[Client.document_index]: struct.Client.html#index-request
[Client.document_put_mapping]: struct.Client.html#method.document_put_mapping
[Client.index_create]: struct.Client.html#create-index-request

[RequestBuilder]: requests/struct.RequestBuilder.html
[RequestBuilder.params]: requests/struct.RequestBuilder.html#method.params
[RawRequestBuilder]: requests/type.RawRequestBuilder.html
[SearchRequest]: requests/endpoints/struct.SearchRequest.html
[GetRequest]: requests/endpoints/struct.GetRequest.html
[IndexRequest]: requests/endpoints/struct.IndexRequest.html
[IndicesPutMappingRequest]: requests/endpoints/struct.IndicesPutMappingRequest.html
[IndicesCreateRequest]: requests/endpoints/struct.IndicesCreateRequest.html

[responses-mod]: responses/index.html
[SyncResponseBuilder]: responses/struct.SyncResponseBuilder.html
[SyncResponseBuilder.into_response]: responses/struct.SyncResponseBuilder.html#method.into_response
[SyncResponseBuilder.into_raw]: responses/struct.SyncResponseBuilder.html#method.into_raw
[AsyncResponseBuilder]: responses/struct.AsyncResponseBuilder.html
[AsyncResponseBuilder.into_response]: responses/struct.AsyncResponseBuilder.html#method.into_response
[AsyncResponseBuilder.into_raw]: responses/struct.AsyncResponseBuilder.html#method.into_raw
[SearchResponse]: responses/type.SearchResponse.html
[GetResponse]: responses/type.GetResponse.html
[IndexResponse]: responses/struct.IndexResponse.html
[CommandResponse]: responses/struct.CommandResponse.html
[SyncHttpResponse]: responses/struct.SyncHttpResponse.html
[AsyncHttpResponse]: responses/struct.AsyncHttpResponse.html
[response-types]: responses/parse/trait.IsOk.html#implementors
*/

pub mod sender;
pub mod requests;
pub mod responses;

pub use self::sender::{SyncClientBuilder, AsyncClientBuilder, SyncClient, AsyncClient, PreRequestParams, RequestParams};

use self::sender::NodeAddresses;

/**
A HTTP client for the Elasticsearch REST API.

The `Client` is a structure that lets you create and send request builders.
`Client` is generic over a `Sender`, but rather than use `Client` directly, use one of:

- [`SyncClient`][SyncClient]
- [`AsyncClient`][AsyncClient]

# Examples

Create a synchronous `Client` and send a ping request:

```no_run
# extern crate elastic;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let client = SyncClientBuilder::new().build()?;

let response = client.request(PingRequest::new())
                     .send()?
                     .into_response::<PingResponse>()?;
# Ok(())
# }
```

Create an asynchronous `Client` and send a ping request:

```no_run
# extern crate futures;
# extern crate tokio_core;
# extern crate elastic;
# use futures::Future;
# use tokio_core::reactor::Core;
# use elastic::prelude::*;
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
let mut core = Core::new()?;
let client = AsyncClientBuilder::new().build(&core.handle())?;

let response_future = client.request(PingRequest::new())
                            .send()
                            .and_then(|res| res.into_response::<PingResponse>());

core.run(response_future)?;
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

pub mod prelude {
    /*! A glob import for convenience. */

    pub use super::sender::{PreRequestParams, RequestParams};
    pub use super::{AsyncClient, AsyncClientBuilder, SyncClient, SyncClientBuilder};
    pub use super::requests::prelude::*;
    pub use super::responses::prelude::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tests::*;

    #[test]
    fn client_is_send_sync() {
        assert_send::<SyncClient>();
        assert_sync::<SyncClient>();
    }
}
