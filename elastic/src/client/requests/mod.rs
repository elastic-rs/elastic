//! Request types for the Elasticsearch REST API.
//!
//! Key request types include:
//!
//! - [`SearchRequest`](endpoints/struct.SearchRequest.html) for the [Query DSL](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html)
//! - [`GetRequest`](endpoints/struct.GetRequest.html) for the [Document API](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html)
//! - [`IndexRequest`](endpoints/struct.IndexRequest.html) for the [Document API](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html)
//! - [`IndicesPutMappingRequest`](endpoints/struct.IndicesPutMappingRequest.html) for the [Mapping API](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html)
//! - [`BulkRequest`](endpoints/struct.BulkRequest.html) for the [Bulk API](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html)

mod search;

use elastic_reqwest::ElasticClient;

use error::*;
use client::{RequestParams, RequestBuilder, ResponseBuilder};

pub use elastic_reqwest::IntoReqwestBody as IntoBody;
pub use elastic_reqwest::req::{HttpRequest, HttpMethod, empty_body, Url, DefaultBody};
pub use elastic_reqwest::req::params;
pub use elastic_reqwest::req::endpoints;

pub use self::params::*;
pub use self::endpoints::*;
pub use self::search::*;

impl<'a, TRequest, TBody> RequestBuilder<'a, TRequest, TBody> {
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
}

impl<'a, TRequest, TBody> RequestBuilder<'a, TRequest, TBody>
    where TRequest: Into<HttpRequest<'static, TBody>>,
          TBody: IntoBody
{
    fn send_raw(self) -> Result<ResponseBuilder> {
        let params = self.params.as_ref().unwrap_or(&self.client.params);

        let res = self.client.http.elastic_req(params, self.req)?.into();

        Ok(ResponseBuilder(res))
    }
}

impl<'a, TRequest, TBody> RequestBuilder<'a, TRequest, TBody>
    where TRequest: Into<HttpRequest<'static, TBody>>,
          TBody: IntoBody
{
    /// Send this request and return the response.
    ///
    /// This method consumes the `RequestBuilder` and returns a `ResponseBuilder`
    /// that can be used to parse the response.
    pub fn send(self) -> Result<ResponseBuilder> {
        self.send_raw()
    }
}