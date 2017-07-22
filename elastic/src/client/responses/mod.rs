/*!
Response types for the Elasticsearch REST API.

All [high-level builders][request-builders] return one of these response types for you.
This module also contains helpers that can be used to parse responses from a raw [`HttpResponse`][HttpResponse].

[HttpResponse]: struct.HttpResponse.html
[request-builders]: ../index.html#request-builders
*/

pub mod parse;

use std::io::{Read, Result as IoResult};
use serde::de::DeserializeOwned;
use elastic_reqwest::SyncFromResponse;
use elastic_reqwest::res::parse;
use reqwest::Response as RawResponse;

use error::*;
use client::IntoResponseBuilder;
use self::parse::IsOk;

pub use elastic_reqwest::res::{SearchResponse, GetResponse, Shards, CommandResponse, IndexResponse, PingResponse,
                               BulkResponse, BulkErrorsResponse};

pub use elastic_reqwest::res::search;
pub use elastic_reqwest::res::bulk;

/**
A builder for a response.

This structure wraps the completed HTTP response but gives you options for converting it into a concrete type.
You can also `Read` directly from the response body.
*/
pub struct ResponseBuilder(RawResponse);

impl Into<ResponseBuilder> for IntoResponseBuilder {
    fn into(self) -> ResponseBuilder {
        ResponseBuilder(self.0)
    }
}

impl ResponseBuilder {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> u16 {
        self.0.status().into()
    }

    /**
    Get the response body from JSON.
    
    Convert the builder into a raw HTTP response that implements `Read`.
    */
    pub fn into_raw(self) -> HttpResponse {
        HttpResponse(self.0)
    }

    /**
    Parse an API response type from the HTTP body.
    
    This will consume the `ResponseBuilder` and return a [concrete response type][response-types] or an error.
    
    The response is parsed according to the `IsOk` implementation for `T` that will inspect the response and either return an `Ok(T)` or an `Err(ApiError)`.
    
    # Examples
    
    Get a strongly typed `SearchResponse`:
    
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
                         .and_then(into_response::<SearchResponse<MyType>>);
    # }
    ```
    
    You can also read a response as a `serde_json::Value`, which will be `Ok(Value)`
    if the HTTP status code is `Ok` or `Err(ApiError)` otherwise:
    
    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # let params = RequestParams::default();
    # let client = Client::new(params).unwrap();
    # let req = PingRequest::new();
    let response = client.request(req)
                         .send()
                         .and_then(into_response::<Value>);
    # }
    ```

    [response-types]: parse/trait.IsOk.html#implementors
    */
    pub fn into_response<T>(self) -> Result<T>
        where T: IsOk + DeserializeOwned
    {
        parse().from_response(self.0).map_err(Into::into)
    }
}

/** A raw HTTP response that can be buffered using `Read`. */
pub struct HttpResponse(RawResponse);

impl HttpResponse {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> u16 {
        self.0.status().into()
    }
}

impl Read for HttpResponse {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.0.read(buf)
    }
}
