use serde::Deserialize;
use reqwest::{self, Client as HttpClient};
use elastic_reqwest::ElasticClient;
use super::errors::*;

pub use reqwest::StatusCode;
pub use reqwest::header;
pub use elastic_reqwest::RequestParams;

/// Request types the Elasticsearch REST API.
pub use elastic_requests::*;

/// Response types for the Elasticsearch REST API.
pub use elastic_responses::*;

/// A HTTP client for the Elasticsearch REST API.
pub struct Client {
    http: HttpClient,
    params: RequestParams,
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
            req: req
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
    pub fn send(self) -> Result<HttpResponse> {
        let params = self.params.as_ref().unwrap_or(&self.client.params);

        let res = self.client.http.elastic_req(params, self.req)?;

        Ok(HttpResponse(res))
    }
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

/// A raw HTTP response.
pub struct HttpResponse(reqwest::Response);

impl HttpResponse {
    /// Get the `reqwest` response.
    pub fn raw(self) -> reqwest::Response {
        self.0
    }

    /// Get the response body as JSON.
    pub fn json<T>(mut self) -> Result<T>
        where T: Deserialize
    {
        self.0.json().map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_builder_params() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = RequestBuilder::new(&client, None, PingRequest::new())
            .params(|p| p.url_param("pretty", true))
            .params(|p| p.url_param("refresh", true));

        let params = &req.params.unwrap();

        let (_, query) = params.get_url_qry();

        assert_eq!("http://eshost:9200", &params.base_url);
        assert_eq!("?pretty=true&refresh=true", query.unwrap());
    }
}