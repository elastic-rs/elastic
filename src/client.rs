use elastic_reqwest::ElasticClient;
use error::*;
use reqwest::Client as HttpClient;

/// Request types the Elasticsearch REST API.
pub mod requests {
    pub use elastic_requests::*;
    pub use impls::*;
}

/// Response types for the Elasticsearch REST API.
pub mod responses {
    use serde::Deserialize;
    use serde_json;
    use reqwest::{Response as RawResponse, StatusCode};
    use error::*;

    pub use elastic_responses::*;

    /// A raw HTTP response.
    pub struct HttpResponse(RawResponse);

    impl From<RawResponse> for HttpResponse {
        fn from(value: RawResponse) -> Self {
            HttpResponse(value)
        }
    }

    pub trait IntoQueryResponse<T> 
        where T: Deserialize
    {
        fn query_response(self) -> Result<QueryResponseOf<Hit<T>>>;
    }

    pub trait IntoGetDocResponse<T: Deserialize>
        where T: Deserialize
    {
        fn doc_response(self) -> Result<GetDocResponseOf<T>>;
    }

    impl HttpResponse {
        /// Get the status code for the response.
        pub fn status(&self) -> &StatusCode {
            self.0.status()
        }

        /// Get the raw HTTP response.
        pub fn raw(self) -> RawResponse {
            self.0
        }

        /// Get the response body from JSON.
        /// 
        /// This method takes an `is_ok` closure that determines
        /// whether the result is successful.
        pub fn response<T, F>(self, is_ok: F) -> Result<T>
            where T: Deserialize,
                  F: Fn(&RawResponse) -> bool
        {
            match is_ok(&self.0) {
                true => serde_json::from_reader(self.0).map_err(|e| e.into()),
                false => {
                    let err: ApiError = serde_json::from_reader(self.0)?;
                    Err(ErrorKind::Api(err).into())
                }
            }
        }

        /// Deserialise as a Query DSL response.
        pub fn query_response<T>(self) -> Result<QueryResponseOf<Hit<T>>>
            where T: Deserialize
        {
            self.response(|res| {
                match *res.status() {
                    StatusCode::Ok => true,
                    _ => false
                }
            })
        }

        /// Deserialise as a get document response.
        pub fn doc_response<T>(self) -> Result<GetDocResponseOf<T>>
            where T: Deserialize
        {
            self.response(|res| {
                match *res.status() {
                    StatusCode::Ok |
                    StatusCode::NotFound => true,
                    _ => false
                }
            })
        }
    }
}

pub use elastic_reqwest::RequestParams;

use self::requests::HttpRequest;
use self::responses::HttpResponse;

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
    pub fn send(self) -> Result<responses::HttpResponse> {
        let params = self.params.as_ref().unwrap_or(&self.client.params);

        let res = self.client.http.elastic_req(params, self.req)?;

        Ok(HttpResponse::from(res))
    }
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