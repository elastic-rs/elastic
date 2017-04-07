//! HTTP client, requests and responses.
//!
//! This module contains the HTTP client, as well
//! as request and response types.
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

use std::marker::PhantomData;

use elastic_reqwest::ElasticClient;
use error::*;
use reqwest::{Client as HttpClient, Response as RawResponse};

use self::requests::{IntoBody, HttpRequest};
use self::responses::HttpResponse;
use self::responses::parse::FromResponse;

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

    /// Create a `RequestBuilder` that can be configured before sending.
    /// 
    /// The `request` method accepts any type that can be converted into
    /// a [`HttpRequest<'static>`](requests/struct.HttpRequest.html), 
    /// which includes the endpoint types in the [`requests`](requests/endpoints/index.html) module.
    ///
    /// # Examples
    /// 
    /// Turn a concrete request into a `RequestBuilder`:
    /// 
    /// ```no_run
    /// # use elastic::prelude::*;
    /// # let client = Client::new(RequestParams::default()).unwrap();
    /// // `PingRequest` implements `Into<HttpRequest>`
    /// let req = PingRequest::new();
    /// 
    /// // Turn the `PingRequest` into a `RequestBuilder`
    /// let builder = client.request(req);
    /// 
    /// // Send the `RequestBuilder`
    /// let res = builder.send().unwrap();
    /// ```
    pub fn request<'a, I, B>(&'a self, req: I) -> RequestBuilder<'a, I, B>
        where I: Into<HttpRequest<'static, B>>,
              B: IntoBody
    {
        RequestBuilder::new(&self, None, req)
    }
}

/// A builder for a request.
/// 
/// This structure wraps up a concrete REST API request type
/// and lets you adjust parameters before sending it.
pub struct RequestBuilder<'a, I, B> {
    client: &'a Client,
    params: Option<RequestParams>,
    req: I,
    _body: PhantomData<B>,
}

impl<'a, I, B> RequestBuilder<'a, I, B> {
    /// Manually construct a `RequestBuilder`.
    ///
    /// If the `RequestParams` are `None`, then the parameters from the
    /// `Client` are used.
    fn new(client: &'a Client, params: Option<RequestParams>, req: I) -> Self {
        RequestBuilder {
            client: client,
            params: params,
            req: req,
            _body: PhantomData,
        }
    }
}

impl<'a, I, B> RequestBuilder<'a, I, B>
    where I: Into<HttpRequest<'static, B>>,
          B: IntoBody
{
    /// Override the parameters for this request.
    ///
    /// This method will clone the `RequestParams` on the `Client` and pass
    /// them to the closure.
    /// 
    /// # Examples
    /// 
    /// Add a url param to force an index refresh:
    /// 
    /// ```no_run
    /// # use elastic::prelude::*;
    /// # let client = Client::new(RequestParams::default()).unwrap();
    /// # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    /// client.request(get_req())
    ///       .params(|params| params.url_param("refresh", true))
    ///       .send()
    ///       .unwrap();
    /// ```
    pub fn params<F>(mut self, builder: F) -> Self
        where F: Fn(RequestParams) -> RequestParams
    {
        self.params = Some(builder(self.params.unwrap_or(self.client.params.clone())));

        self
    }

    /// Send this request and return the response.
    /// 
    /// This method consumes the `RequestBuilder` and returns a `ResponseBuilder`
    /// that can be used to parse the response.
    pub fn send(self) -> Result<ResponseBuilder> {
        let params = self.params.as_ref().unwrap_or(&self.client.params);

        let res = self.client.http.elastic_req(params, self.req)?;

        Ok(ResponseBuilder::from(res))
    }
}

/// A builder for a response.
/// 
/// This structure wraps the completed HTTP response but gives you
/// options for converting it into a concrete type.
pub struct ResponseBuilder(RawResponse);

impl From<RawResponse> for ResponseBuilder {
    fn from(value: RawResponse) -> Self {
        ResponseBuilder(value)
    }
}

impl Into<HttpResponse<RawResponse>> for ResponseBuilder {
    fn into(self) -> HttpResponse<RawResponse> {
        let status = self.0.status().to_u16();

        HttpResponse::new(status, self.0)
    }
}

impl ResponseBuilder {
    /// Get the raw HTTP response.
    /// 
    /// This will consume the `ResponseBuilder` and return a raw
    /// `HttpResponse` that can be read as a byte buffer.
    pub fn raw(self) -> HttpResponse<RawResponse> {
        HttpResponse::new(self.0.status().to_u16(), self.0)
    }

    /// Get the HTTP status for the response.
    pub fn status(&self) -> u16 {
        self.0.status().to_u16()
    }

    /// Get the response body from JSON.
    /// 
    /// This will consume the `ResponseBuilder` and return a
    /// concrete response type or an error.
    ///
    /// The response is parsed according to the `FromResponse`
    /// implementation for `T` that will inspect the response and
    /// either return an `Ok(T)` or an `Err(ApiError)`.
    ///
    /// # Examples
    /// 
    /// Get a strongly typed `SearchResponse`:
    /// 
    /// ```no_run
    /// # extern crate serde;
    /// # #[macro_use]
    /// # extern crate serde_derive;
    /// # #[macro_use]
    /// # extern crate elastic_derive;
    /// # extern crate elastic;
    /// # use elastic::prelude::*;
    /// # fn main() {
    /// # #[derive(Serialize, Deserialize, ElasticType)]
    /// # struct MyType {
    /// #     pub id: i32,
    /// #     pub title: String,
    /// #     pub timestamp: Date<DefaultDateFormat>
    /// # }
    /// # let params = RequestParams::new("http://es_host:9200");
    /// # let client = Client::new(params).unwrap();
    /// # let req = PingRequest::new();
    /// let response = client.request(req)
    ///                      .send()
    ///                      .and_then(|res| res.response::<SearchResponse<MyType>>());
    /// # }
    /// ```
    /// 
    /// You can also read a response as a `serde_json::Value`, which will be `Ok(Value)`
    /// if the HTTP status code is `Ok` or `Err(ApiError)` otherwise:
    /// 
    /// ```no_run
    /// # extern crate elastic;
    /// # extern crate serde_json;
    /// # use serde_json::Value;
    /// # use elastic::prelude::*;
    /// # fn main() {
    /// # let params = RequestParams::default();
    /// # let client = Client::new(params).unwrap();
    /// # let req = PingRequest::new();
    /// let response = client.request(req)
    ///                      .send()
    ///                      .and_then(|res| res.response::<Value>());
    /// # }
    /// ```
    pub fn response<T>(self) -> Result<T>
        where T: FromResponse
    {
        T::from_response(self).map_err(|e| e.into())
    }
}

pub mod requests {
    //! Request types for the Elasticsearch REST API.

    pub use elastic_reqwest::IntoReqwestBody as IntoBody;
    pub use elastic_requests::{HttpRequest, HttpMethod, empty_body, Url};
    pub use elastic_requests::params;
    pub use elastic_requests::endpoints;

    pub use self::params::*;
    pub use self::endpoints::*;
    pub use impls::*;
}

pub mod responses {
    //! Response types for the Elasticsearch REST API.

    pub use elastic_responses::{HttpResponse, AggregationIterator, Aggregations, Hit, Hits, Shards};

    pub mod parse {
        //! Utility types for response parsing.
        //! 
        //! # Examples
        //! 
        //! Implement `FromResponse` for a deserialisable type that converts
        //! a http response into a concrete type.
        //! This example defines a search response that, for whatever reason, 
        //! only includes the `took` field:
        //! 
        //! ```
        //! # extern crate serde;
        //! # #[macro_use]
        //! # extern crate serde_derive;
        //! # extern crate elastic;
        //! # use std::io::Read;
        //! # use elastic::prelude::*;
        //! # use elastic::error::ResponseError;
        //! # use elastic::client::responses::parse::*;
        //! #[derive(Deserialize)]
        //! struct MyResponse {
        //!     took: u64
        //! }
        //! 
        //! impl FromResponse for MyResponse {
        //!     fn from_response<I, R>(res: I) -> Result<Self, ResponseError> 
        //!         where I: Into<HttpResponse<R>>, R: Read 
        //!     {
        //!         // HttpResponse.response() lets you inspect a response for success
        //!         res.into().response(|http_res| {
        //!             match http_res.status() {
        //!                 // If the status is 2xx then return the response with `ok: true`
        //!                 // The body will be parsed as a `MyResponse`.
        //!                 200...299 => Ok(MaybeOkResponse::new(true, http_res)),
        //!                 // Otherwise return the response with `ok: false`
        //!                 // The body will be parsed as an `ApiError`.
        //!                 _ => Ok(MaybeOkResponse::new(false, http_res))
        //!             }
        //!         })
        //!     }
        //! }
        //! # fn main() {}
        //! ```
        //! 
        //! You can also parse the response body into a temporary `serde_json::Value`
        //! if the status code isn't enough to determine if it's ok.
        //! This will consume the `UnbufferedResponse` and return a `BufferedResponse`
        //! instead that keeps the response body private for later handlers to use.

        pub use elastic_responses::FromResponse;
        pub use elastic_responses::parse::{MaybeOkResponse, MaybeBufferedResponse,
                                           UnbufferedResponse, BufferedResponse};
    }

    use elastic_responses::{SearchResponseOf, GetResponseOf};

    pub use elastic_responses::PingResponse;

    /// A generic Search API response.
    /// 
    /// For responses that contain multiple document types, use
    /// `SearchResponse<serde_json::Value>`.
    /// 
    /// This type won't parse if you've applied any [response filters]().
    /// If you need to tweak the shape of the search response you can
    /// define your own response type and implement `FromResponse` for it.
    /// See the [`parse`](parse/index.html) mod for more details.
    pub type SearchResponse<T> = SearchResponseOf<Hit<T>>;

    /// A generic Get Document API response.
    pub type GetResponse<T> = GetResponseOf<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_builder_params() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = RequestBuilder::new(&client, None, requests::PingRequest::new())
            .params(|p| p.url_param("pretty", true))
            .params(|p| p.url_param("refresh", true));

        let params = &req.params.unwrap();

        let (_, query) = params.get_url_qry();

        assert_eq!("http://eshost:9200", &params.base_url);
        assert_eq!("?pretty=true&refresh=true", query.unwrap());
    }
}
