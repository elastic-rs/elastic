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
    use std::io::{Cursor, Read};
    use serde::Deserialize;
    use serde_json::{self, Value};
    use reqwest::{Response as RawResponse, StatusCode};
    use error::*;

    pub use elastic_responses::{AggregationIterator, Aggregations, Hit, Hits, Shards};

    use elastic_responses::{SearchResponseOf, GetResponseOf};

    pub type SearchResponse<T> = SearchResponseOf<Hit<T>>;
    pub type GetResponse<T> = GetResponseOf<T>;

    /// A response that might be successful or an `ApiError`.
    pub struct MaybeOkResponse {
        ok: bool,
        res: MaybeBufferedResponse,
    }

    impl MaybeOkResponse {
        /// Create a new response that indicates where or not the
        /// body is successful or an `ApiError`.
        pub fn new<I>(ok: bool, res: I) -> Self
            where I: Into<MaybeBufferedResponse>
        {
            MaybeOkResponse {
                ok: ok,
                res: res.into(),
            }
        }
    }

    /// A response body that may or may not have been buffered.
    /// 
    /// This type makes it possible to inspect the response body for
    /// an error type before passing it along to be deserialised properly.
    pub enum MaybeBufferedResponse {
        Unbuffered(UnbufferedResponse),
        Buffered(BufferedResponse)
    }

    impl From<UnbufferedResponse> for MaybeBufferedResponse {
        fn from(value: UnbufferedResponse) -> Self {
            MaybeBufferedResponse::Unbuffered(value)
        }
    }

    impl From<BufferedResponse> for MaybeBufferedResponse {
        fn from(value: BufferedResponse) -> Self {
            MaybeBufferedResponse::Buffered(value)
        }
    }

    /// An untouched response body.
    pub struct UnbufferedResponse(RawResponse);

    impl UnbufferedResponse {
        /// Get the HTTP status code for the response.
        pub fn status(&self) -> StatusCode {
            self.0.status().to_owned()
        }

        /// Buffer the response body into a `serde_json::Value` and return
        /// a `BufferedResponse`.
        pub fn body(mut self) -> Result<(Value, BufferedResponse)> {
            let status = self.status();

            let mut buf = Vec::new();
            self.0.read_to_end(&mut buf).map_err(|e| ErrorKind::Json(e.into()))?;

            let body: Value = serde_json::from_reader(Cursor::new(&buf))?;

            Ok((body, BufferedResponse(status, buf)))
        }
    }

    /// A previously buffered response body.
    pub struct BufferedResponse(StatusCode, Vec<u8>);

    impl BufferedResponse {
        /// Get the HTTP status code for the response.
        pub fn status(&self) -> StatusCode {
            self.0.to_owned()
        }
    }

    /// A raw HTTP response.
    pub struct HttpResponse(RawResponse);

    impl From<RawResponse> for HttpResponse {
        fn from(value: RawResponse) -> Self {
            HttpResponse(value)
        }
    }

    macro_rules! read_ok {
        ($buf:expr) => (serde_json::from_reader($buf).map_err(|e| e.into()))
    }

    macro_rules! read_err {
        ($buf:expr) => ({
            let err: ApiError = serde_json::from_reader($buf)?;
            Err(ErrorKind::Api(err).into())
        })
    }

    impl HttpResponse {
        /// Get the status code for the response.
        pub fn status(&self) -> StatusCode {
            self.0.status().to_owned()
        }

        /// Get the raw HTTP response.
        pub fn raw(self) -> RawResponse {
            self.0
        }

        /// Get the response body from JSON.
        ///
        /// This method takes a closure that determines
        /// whether the result is successful.
        pub fn response<T, F>(self, is_ok: F) -> Result<T>
            where T: Deserialize,
                  F: Fn(UnbufferedResponse) -> Result<MaybeOkResponse>
        {
            let maybe = is_ok(UnbufferedResponse(self.0))?;

            let ok = maybe.ok;
            let res = maybe.res;

            match ok {
                true => {
                    match res {
                        MaybeBufferedResponse::Buffered(b) => read_ok!(Cursor::new(b.1)),
                        MaybeBufferedResponse::Unbuffered(b) => read_ok!(b.0),
                    }
                }
                false => {
                    match res {
                        MaybeBufferedResponse::Buffered(b) => read_err!(Cursor::new(b.1)),
                        MaybeBufferedResponse::Unbuffered(b) => read_err!(b.0),
                    }
                }
            }
        }

        /// Deserialise as a Query DSL response.
        pub fn query_response<T>(self) -> Result<SearchResponse<T>>
            where T: Deserialize
        {
            self.response(|res| {
                match res.status() {
                    StatusCode::Ok => Ok(MaybeOkResponse::new(true, res)),
                    _ => Ok(MaybeOkResponse::new(false, res)),
                }
            })
        }

        /// Deserialise as a get document response.
        ///
        /// If the response status is `NotFound` then the response body
        /// will be buffered multiple times to work out whether it's an error
        /// or not.
        pub fn doc_response<T>(self) -> Result<GetResponse<T>>
            where T: Deserialize
        {
            self.response(|res| {
                match res.status() {
                    StatusCode::Ok => Ok(MaybeOkResponse::new(true, res)),
                    StatusCode::NotFound => {
                        // If we get a 404, it could be an IndexNotFound error or ok
                        // Check if the response contains a root 'error' node
                        let (body, res) = res.body()?;

                        let is_ok = body.as_object()
                            .and_then(|body| body.get("error"))
                            .is_none();

                        Ok(MaybeOkResponse::new(is_ok, res))
                    }
                    _ => Ok(MaybeOkResponse::new(false, res)),
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
