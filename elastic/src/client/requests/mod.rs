/*!
Request types for the Elasticsearch REST API.

Key request types include:

- [`SearchRequest`](endpoints/struct.SearchRequest.html) for the [Query DSL](http://www.elastic.co/guide/en/elasticsearch/reference/master/search-search.html)
- [`GetRequest`](endpoints/struct.GetRequest.html) for the [Document API](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html)
- [`IndexRequest`](endpoints/struct.IndexRequest.html) for the [Document API](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-index_.html)
- [`IndicesPutMappingRequest`](endpoints/struct.IndicesPutMappingRequest.html) for the [Mapping API](http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-put-mapping.html)
- [`BulkRequest`](endpoints/struct.BulkRequest.html) for the [Bulk API](http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-bulk.html)
!*/

use std::marker::PhantomData;
use elastic_reqwest::ElasticClient;

use error::*;
use client::{Client, RequestParams, IntoResponse};
use client::responses::ResponseBuilder;

pub use elastic_reqwest::IntoReqwestBody as IntoBody;
pub use elastic_reqwest::req::{HttpRequest, HttpMethod, empty_body, Url, DefaultBody};
pub use elastic_reqwest::req::params;
pub use elastic_reqwest::req::endpoints;

pub use self::params::*;
pub use self::endpoints::*;

mod search;
pub use self::search::*;

mod get;
pub use self::get::*;

mod index_document;
pub use self::index_document::*;

mod put_mapping;
pub use self::put_mapping::*;

mod create_index;
pub use self::create_index::*;

/**
A builder for a request.

This structure wraps up a concrete REST API request type
and lets you adjust parameters before sending it.
**/
pub struct RequestBuilder<'a, TRequest, TBody> {
    client: &'a Client,
    params: Option<RequestParams>,
    req: TRequest,
    _marker: PhantomData<TBody>,
}

impl Client {
    /**
    Create a `RequestBuilder` that can be configured before sending.
    
    The `request` method accepts any type that can be converted into
    a [`HttpRequest<'static>`](requests/struct.HttpRequest.html),
    which includes the endpoint types in the [`requests`](requests/endpoints/index.html) module.
    
    # Examples
    
    Turn a concrete request into a `RequestBuilder`:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = Client::new(RequestParams::default()).unwrap();
    // `PingRequest` implements `Into<HttpRequest>`
    let req = PingRequest::new();
    
    // Turn the `PingRequest` into a `RequestBuilder`
    let builder = client.request(req);
    
    // Send the `RequestBuilder`
    let res = builder.send().unwrap();
    ```
    **/
    pub fn request<'a, TRequest, TBody>(&'a self,
                                        req: TRequest)
                                        -> RequestBuilder<'a, TRequest, TBody>
        where TRequest: Into<HttpRequest<'static, TBody>>,
              TBody: IntoBody
    {
        RequestBuilder::new(&self, None, req)
    }
}

impl<'a, TRequest, TBody> RequestBuilder<'a, TRequest, TBody> {
    fn new(client: &'a Client, params: Option<RequestParams>, req: TRequest) -> Self {
        RequestBuilder {
            client: client,
            params: params,
            req: req,
            _marker: PhantomData,
        }
    }
}

impl<'a, TRequest, TBody> RequestBuilder<'a, TRequest, TBody> {
    /**
    Override the parameters for this request.
    
    This method will clone the `RequestParams` on the `Client` and pass
    them to the closure.
    
    # Examples
    
    Add a url param to force an index refresh:
    
    ```no_run
    # use elastic::prelude::*;
    # let client = Client::new(RequestParams::default()).unwrap();
    # fn get_req() -> PingRequest<'static> { PingRequest::new() }
    client.request(get_req())
          .params(|params| params.url_param("refresh", true))
          .send()
          .unwrap();
    ```
    **/
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

        let res = self.client.http.elastic_req(params, self.req)?;

        Ok(IntoResponse(res).into())
    }
}

impl<'a, TRequest, TBody> RequestBuilder<'a, TRequest, TBody>
    where TRequest: Into<HttpRequest<'static, TBody>>,
          TBody: IntoBody
{
    /**
    Send this request and return the response.
    
    This method consumes the `RequestBuilder` and returns a `ResponseBuilder`
    that can be used to parse the response.
    **/
    pub fn send(self) -> Result<ResponseBuilder> {
        self.send_raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_builder_params() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = RequestBuilder::<_, ()>::new(&client, None, PingRequest::new())
            .params(|p| p.url_param("pretty", true))
            .params(|p| p.url_param("refresh", true));

        let params = &req.params.unwrap();

        let (_, query) = params.get_url_qry();

        assert_eq!("http://eshost:9200", &params.base_url);
        assert_eq!("?pretty=true&refresh=true", query.unwrap());
    }
}
