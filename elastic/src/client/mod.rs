/*!
HTTP client, requests and responses.

This module contains the HTTP client, as well as request and response types.

# Request builders

Some commonly used endpoints have high-level builder methods you can use to configure requests easily.
They're exposed as methods on the `Client`:

Client method | Elasticsearch API | Raw request type | Response type
------------- | ----------------- | ------------ | -------------
[`search`]()      | [Query DSL]()     | [`SearchRequest`]() | [`SearchResponse`]()
[`get`]()         | [Get Document]()  | [`GetRequest`]() | [`GetResponse`]()
[`index_document`]() | [Put Document]() | [`IndexRequest`]() | [`IndexResponse`]()
[`put_mapping`]() | [Put Mapping]() | [`IndicesPutMappingRequest`]() | [`CommandResponse`]()
[`create_index`]() | [Create Index]() | [`IndicesCreateRequest`]() | [`CommandResponse`]()

All builders follow a standard pattern:

- The `Client` method that takes all required parameters without inference
- Optional or inferred parameters can be overridden in builder methods with inference
- `send` will return a specific response type

## Examples

A `get` request for a value:

```no_run
let response = client.get::<Value>(index("values"), id(1)).send();
```

Is equivalent to:

```no_run
let response = client.request(GetRequest::for_index_ty_id("values", "value", 1))
                     .send()
                     .and_then(into_response::<GetResponse<Value>>);
```

A `search` request for a value:

```no_run
let response = client.search::<Value>()
                     .index("myindex")
                     .ty("myty")
                     .body(json_str!({
                         query: {
                             query_string: {
                                 query: "*"
                             }
                         }
                     }))
                     .send()
                     .unwrap();

match response {
    Ok(response) => {
        // Iterate through the response hits
        for hit in response.hits() {
            println!("{:?}", hit);
        }
    },
    Err(e) => {
        match *e.kind() {
            ErrorKind::Api(ref e) => {
                // handle a REST API error
            },
            ref e => {
                // handle a HTTP or JSON error
            }
        }
    }
}
```

These builders are wrappers around the raw request types, which can also be used to make requests.

# Raw request types

All Elasticsearch API endpoints have a strongly-typed request type that can be used to build a request.
The process is described in more detail below.

## The raw request process

The pieces involved in sending a request and parsing the response are modular.
Each one exposes Rust traits you can implement to support your own logic.
If you just want to send search/get requests and parse a search/get response then
you won't need to worry about this so much.

The basic flow from request to response is:

**1)** Turn a concrete [request type](requests/endpoints/index.html) into a [`RequestBuilder`](struct.RequestBuilder.html):

```text
[RequestType] ---> [Client.request()] ---> [RequestBuilder]
```

**2)** Send the [`RequestBuilder`](struct.RequestBuilder.html) and get a [`ResponseBuilder`](struct.ResponseBuilder.html):

```text
[RequestBuilder.send()] ---> [ResponseBuilder]
```

**3)** Parse the [`ResponseBuilder`](struct.ResponseBuilder.html) to a [response type](responses/parse/trait.FromResponse.html#Implementors):

```text
[ResponseBuilder.response()] ---> [ResponseType]
```

The example below shows how these pieces fit together in code  by sending a simple `SearchRequest`, 
with the steps in the above process labelled:

```no_run
# extern crate elastic;
# #[macro_use]
# extern crate json_str;
# extern crate serde_json;
# use elastic::prelude::*;
# use elastic::error::*;
# use serde_json::Value;
# fn main() {
// Create a `Client`
let client = Client::new(RequestParams::default()).unwrap();

// Create a `SearchRequest` for all indices
let req = {
    let body = json_str!({
        query: {
            query_string: {
                query: "*"
            }
        }
    });

    SearchRequest::for_index("_all", body)
};

// Send the request and read the response as a `SearchResponse`
let response = client.request(req) // 1
                     .send() // 2
                     .and_then(|res| res.into_response::<SearchResponse<Value>>()); // 3

match response {
    Ok(response) => {
        // Iterate through the response hits
        for hit in response.hits() {
            println!("{:?}", hit);
        }
    },
    Err(e) => {
        match *e.kind() {
            ErrorKind::Api(ref e) => {
                // handle a REST API error
            },
            ref e => {
                // handle a HTTP or JSON error
            }
        }
    }
}
# }
```

### 1. Building raw requests

The [`endpoints`]() module contains code-generated request types for the Elasticsearch REST API.
Each request type expects its parameters upfront and is generic over the request body.

A raw search request:

```no_run
let req = {
    let body = json_str!({
        query: {
            query_string: {
                query: "*"
            }
        }
    });

    SearchRequest::for_index_ty("myindex", "myty", body)
};
```

A raw request to index a document:

```no_run
let req = {
    let body = serde_json::to_string(&doc).unwrap();

    IndexRequest::for_index_ty_id("myindex", "myty", body)
};
```

### 2. Sending requests

Both high-level request builders and raw requests have some common builder methods:

- [`params`]() for setting url query parameters
- [`send`]() for sending the request.
For high-level requests this returns a strongly-typed response.
For raw requests this returns a `HttpResponse`.

```no_run
let request_builder = client.request(req);

// Set additional url parameters
let request_builder = request_builder.params(|p| p
    .url_param("pretty", true)
    .url_param("refresh", true)
);

// Send the request
let response = request_builder.send();
```

### 3. Parsing responses

Call `into_response` on a sent request to get a strongly typed response:

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
# let params = RequestParams::new("http://es_host:9200");
# let client = Client::new(params).unwrap();
# let req = PingRequest::new();
let response = client.request(req)
                     .send()
                     .and_then(|res| res.response::<SearchResponse<MyType>>())
                     .unwrap();

// Iterate through the response hits
for hit in response.hits() {
    println!("{:?}", hit);
}
# }
```

Alternatively to `into_repsonse`, call `into_raw` on a sent request to get a raw `HttpResponse`:

```no_run
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# #[macro_use]
# extern crate elastic_derive;
# extern crate elastic;
# use elastic::prelude::*;
# fn main() {
# let params = RequestParams::new("http://es_host:9200");
# let client = Client::new(params).unwrap();
# let req = PingRequest::new();
let response = client.request(req)
                     .send()
                     .map(|res| res.into_raw())
                     .unwrap();

let mut body = String::new();
response.read_to_string(&mut body).unwrap();
# }
```

The `HttpResponse` implements `Read` so you can buffer out the raw response data.
For more details see the [`responses`]() module.
!*/

pub mod requests;
pub mod responses;

use serde::de::DeserializeOwned;
use reqwest::{Client as HttpClient, Response as RawResponse};

use error::*;
use self::responses::ResponseBuilder;
use self::responses::HttpResponse;
use self::responses::parse::IsOk;

pub use elastic_reqwest::RequestParams;

/**
A HTTP client for the Elasticsearch REST API.

The `Client` is a structure that lets you create and send `RequestBuilder`s.
It's mostly a thin wrapper over a `reqwest::Client`.
**/
pub struct Client {
    http: HttpClient,
    params: RequestParams,
}

impl Client {
    /**
    Create a new client for the given parameters.
    
    The parameters given here are used as the defaults for any
    request made by this client, but can be overriden on a
    per-request basis.
    This method can return a `HttpError` if the underlying `reqwest::Client`
    fails to create.
    
    # Examples
    
    Create a `Client` with default parameters:
    
    ```
    # use elastic::prelude::*;
    let client = Client::new(RequestParams::default()).unwrap();
    ```
    
    Create a `Client` for a specific node:
    
    ```
    # use elastic::prelude::*;
    let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();
    ```
    
    See [`RequestParams`](struct.RequestParams.html) for more configuration options.
    **/
    pub fn new(params: RequestParams) -> Result<Self> {
        let client = HttpClient::new()?;

        Ok(Client {
               http: client,
               params: params,
           })
    }
}

/** Try convert a `ResponseBuilder` into a concrete response type. **/
pub fn into_response<T>(res: ResponseBuilder) -> Result<T>
    where T: IsOk + DeserializeOwned
{
    res.into_response()
}

/** Try convert a `ResponseBuilder` into a raw response type. **/
pub fn into_raw(res: ResponseBuilder) -> Result<HttpResponse> {
    Ok(res.raw())
}

struct IntoResponse(RawResponse);
