use elastic_reqwest::ElasticClient;
use error::*;
use reqwest::{Client as HttpClient, Response as RawResponse};

use self::requests::HttpRequest;
use self::responses::HttpResponse;
use self::responses::parse::FromResponse;

pub use elastic_reqwest::RequestParams;

/// A HTTP client for the Elasticsearch REST API.
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
    pub fn new(params: RequestParams) -> Result<Self> {
        let client = HttpClient::new()?;

        Ok(Client {
            http: client,
            params: params,
        })
    }

    /// Create a `RequestBuilder` that can be configured before sending.
    pub fn request<'a, I>(&'a self, req: I) -> RequestBuilder<'a, I>
        where I: Into<HttpRequest<'static>>
    {
        RequestBuilder::new(&self, None, req)
    }
}

/// A builder for a request.
pub struct RequestBuilder<'a, I> {
    client: &'a Client,
    params: Option<RequestParams>,
    req: I,
}

impl<'a, I> RequestBuilder<'a, I> {
    /// Manually construct a `RequestBuilder`.
    ///
    /// If the `RequestParams` are `None`, then the parameters from the
    /// `Client` are used.
    pub fn new(client: &'a Client, params: Option<RequestParams>, req: I) -> Self {
        RequestBuilder {
            client: client,
            params: params,
            req: req,
        }
    }
}

impl<'a, I> RequestBuilder<'a, I>
    where I: Into<HttpRequest<'static>>
{
    /// Override the parameters for this request.
    ///
    /// This method will clone the `RequestParams` on the `Client` and pass
    /// them to the closure.
    pub fn params<F>(mut self, builder: F) -> Self
        where F: Fn(RequestParams) -> RequestParams
    {
        self.params = Some(builder(self.params.unwrap_or(self.client.params.clone())));

        self
    }

    /// Send this request and return the response.
    pub fn send(self) -> Result<ResponseBuilder> {
        let params = self.params.as_ref().unwrap_or(&self.client.params);

        let res = self.client.http.elastic_req(params, self.req)?;

        Ok(ResponseBuilder::from(res))
    }
}

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
    pub fn raw(self) -> HttpResponse<RawResponse> {
        HttpResponse::new(self.0.status().to_u16(), self.0)
    }

    /// Get the status for the response.
    pub fn status(&self) -> u16 {
        self.0.status().to_u16()
    }

    /// Get the response body from JSON.
    pub fn response<T>(self) -> Result<T>
        where T: FromResponse
    {
        T::from_response(self).map_err(|e| e.into())
    }
}

/// Request types the Elasticsearch REST API.
pub mod requests {
    pub use elastic_requests::*;
    pub use impls::*;
}

/// Response types for the Elasticsearch REST API.
pub mod responses {
    pub use elastic_responses::{HttpResponse, AggregationIterator, Aggregations,
                                Hit, Hits, Shards};

    pub mod parse {
        pub use elastic_responses::FromResponse;
        pub use elastic_responses::parse::{MaybeOkResponse, MaybeBufferedResponse, UnbufferedResponse, BufferedResponse};
    }

    use elastic_responses::{SearchResponseOf, GetResponseOf};

    pub type SearchResponse<T> = SearchResponseOf<Hit<T>>;
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
