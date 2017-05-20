//! HTTP client, requests and responses.
//!
//! This module contains the HTTP client, as well
//! as request and response types.
//! 
//! # Request builders
//! 
//! Some commonly used endpoints have high-level builder methods you can use to configure requests easily.
//! They're exposed as methods on the `Client`:
//! 
//! - `search`
//! - `index_document`
//! - `put_mapping`
//! - `create_index`
//! 
//! All builders follow a standard pattern:
//! 
//! - Client method that takes all required parameters without inference
//! - Optional or inferred parameters can be overrided in builder methods with inference
//! - Send will return a specific response type
//! 
//! ## Examples
//! 
//! A `get` request for a value:
//! 
//! ```no_run
//! let response = client.get::<Value>(index("values"), id(1)).send();
//! ```
//! 
//! Is equivalent to:
//! 
//! ```no_run
//! let response = client.request(GetRequest::for_index_ty_id("values", "value", 1))
//!                      .send()
//!                      .and_then(into_response::<GetResponse<Value>>);
//! ```
//! 
//! These builders are wrappers around the raw request types, which can also be used to make requests.
//!
//! # The request process
//!
//! The pieces involved in sending a request and parsing the response are modular.
//! Each one exposes Rust traits you can implement to support your own logic.
//! If you just want to send search/get requests and parse a search/get response then
//! you won't need to worry about this so much.
//!
//! The basic flow from request to response is:
//!
//! **1)** Turn a concrete [request type](requests/endpoints/index.html) into a [`RequestBuilder`](struct.RequestBuilder.html):
//!
//! ```text
//! [RequestType] ---> [Client.request()] ---> [RequestBuilder]
//! ```
//!
//! **2)** Send the [`RequestBuilder`](struct.RequestBuilder.html) and get a [`ResponseBuilder`](struct.ResponseBuilder.html):
//!
//! ```text
//! [RequestBuilder.send()] ---> [ResponseBuilder]
//! ```
//!
//! **3)** Parse the [`ResponseBuilder`](struct.ResponseBuilder.html) to a [response type](responses/parse/trait.FromResponse.html#Implementors):
//!
//! ```text
//! [ResponseBuilder.response()] ---> [ResponseType]
//! ```
//!
//! The example below shows how these pieces fit together in code.
//!
//! # Examples
//!
//! This example sends a simple `SearchRequest`, with the steps in the above
//! process labelled:
//!
//! ```no_run
//! # extern crate elastic;
//! # #[macro_use]
//! # extern crate json_str;
//! # extern crate serde_json;
//! # use elastic::prelude::*;
//! # use elastic::error::*;
//! # use serde_json::Value;
//! # fn main() {
//! // Create a `Client`
//! let client = Client::new(RequestParams::default()).unwrap();
//!
//! // Create a `SearchRequest` for all indices
//! let req = {
//!     let body = json_str!({
//!         query: {
//!             query_string: {
//!                 query: "*"
//!             }
//!         }
//!     });
//!
//!     SearchRequest::for_index("_all", body)
//! };
//!
//! // Send the request and read the response as a `SearchResponse`
//! let res = client.request(req) // 1
//!                 .send() // 2
//!                 .and_then(|res| res.response::<SearchResponse<Value>>()); // 3
//!
//! match res {
//!     Ok(response) => {
//!         // Iterate through the response hits
//!         for hit in response.hits() {
//!             println!("{:?}", hit);
//!         }
//!     },
//!     Err(e) => {
//!         match *e.kind() {
//!             ErrorKind::Api(ref e) => {
//!                 // handle a REST API error
//!             },
//!             ref e => {
//!                 // handle a HTTP or JSON error
//!             }
//!         }
//!     }
//! }
//! # }
//! ```

pub mod requests;
pub mod responses;

use serde::de::DeserializeOwned;
use reqwest::{Client as HttpClient, Response as RawResponse};

use error::*;
use self::responses::ResponseBuilder;
use self::responses::HttpResponse;
use self::responses::parse::IsOk;

pub use elastic_reqwest::RequestParams;

/// A HTTP client for the Elasticsearch REST API.
///
/// The `Client` is a structure that lets you create and send `RequestBuilder`s.
/// It's mostly a thin wrapper over a `reqwest::Client`.
pub struct Client {
    http: HttpClient,
    params: RequestParams,
}

impl Client {
    /// Create a new client for the given parameters.
    ///
    /// The parameters given here are used as the defaults for any
    /// request made by this client, but can be overriden on a
    /// per-request basis.
    /// This method can return a `HttpError` if the underlying `reqwest::Client`
    /// fails to create.
    ///
    /// # Examples
    ///
    /// Create a `Client` with default parameters:
    ///
    /// ```
    /// # use elastic::prelude::*;
    /// let client = Client::new(RequestParams::default()).unwrap();
    /// ```
    ///
    /// Create a `Client` for a specific node:
    ///
    /// ```
    /// # use elastic::prelude::*;
    /// let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();
    /// ```
    ///
    /// See [`RequestParams`](struct.RequestParams.html) for more configuration options.
    pub fn new(params: RequestParams) -> Result<Self> {
        let client = HttpClient::new()?;

        Ok(Client {
            http: client,
            params: params,
        })
    }
}

/// Try convert a `ResponseBuilder` into a concrete response type.
pub fn into_response<T>(res: ResponseBuilder) -> Result<T>
    where T: IsOk + DeserializeOwned
{
    res.into_response()
}

/// Try convert a `ResponseBuilder` into a raw response type.
pub fn into_raw(res: ResponseBuilder) -> Result<HttpResponse> {
    Ok(res.raw())
}

struct IntoResponse(RawResponse);
