use bytes::Bytes;
use serde::de::DeserializeOwned;
use serde_json::Value;
use reqwest::unstable::async::{Client, RequestBuilder, Response, Body};
use futures::{Future, IntoFuture};
use tokio_io::io;

use super::req::HttpRequest;
use super::res::parsing::{Parse, IsOk};
use super::{Error, RequestParams, build_url, build_method};

/// A type that can be converted into a request body.
pub trait IntoBodyAsync {
    /// Convert self into a body.
    fn into_body(self) -> Body;
}

impl IntoBodyAsync for Body {
    fn into_body(self) -> Body {
        self
    }
}

impl IntoBodyAsync for Vec<u8> {
    fn into_body(self) -> Body {
        self.into()
    }
}

impl IntoBodyAsync for String {
    fn into_body(self) -> Body {
        self.into()
    }
}

impl IntoBodyAsync for Value {
    fn into_body(self) -> Body {
        self.to_string().into()
    }
}

impl IntoBodyAsync for &'static [u8] {
    fn into_body(self) -> Body {
        Bytes::from(self).into()
    }
}

impl IntoBodyAsync for &'static str {
    fn into_body(self) -> Body {
        Bytes::from(self).into()
    }
}

/// Represents a client that can send Elasticsearch requests.
pub trait ElasticClientAsync {
    /// Send a request and get a response.
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Box<Future<Item = Response, Error = Error>>
        where I: Into<HttpRequest<'static, B>>,
              B: IntoBodyAsync;
}

/// Build an asynchronous `reqwest::RequestBuilder` from an Elasticsearch request.
pub fn build_req_async<I, B>(client: &Client, params: &RequestParams, req: I) -> Result<RequestBuilder, Error>
    where I: Into<HttpRequest<'static, B>>,
          B: IntoBodyAsync
{
    let req = req.into();

    let url = build_url(&req.url, &params);
    let method = build_method(req.method);
    let body = req.body;

    let mut req = client.request(method, &url)?;
    {
        req.headers(params.get_headers());

        if let Some(body) = body {
            req.body(body.into_body());
        }
    }

    Ok(req)
}

impl ElasticClientAsync for Client {
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Box<Future<Item = Response, Error = Error>>
        where I: Into<HttpRequest<'static, B>>,
              B: IntoBodyAsync
    {
        let fut = build_req_async(&self, params, req)
            .into_future()
            .and_then(|mut req| req.send().map_err(Into::into));

        Box::new(fut)
    }
}

/// Represents a response that can be parsed into a concrete Elasticsearch response.
pub trait ParseResponseAsync<TResponse> {
    /// Parse a response into a concrete response type.
    fn from_response(self, response: Response) -> Box<Future<Item = TResponse, Error = Error>>;
}

impl<TResponse: IsOk + DeserializeOwned + 'static> ParseResponseAsync<TResponse> for Parse<TResponse> {
    fn from_response(self, response: Response) -> Box<Future<Item = TResponse, Error = Error>> {
        let status: u16 = response.status().into();

        let body_future = io::read_to_end(response, Vec::new())
            .map_err(Into::into);

        let de_future = body_future
            .and_then(move |(_, body)| {
                self.from_slice(status, &body).map_err(Into::into)
            });

        Box::new(de_future)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use reqwest::unstable::async::{Client, RequestBuilder};
    use reqwest::header::ContentType;
    use tokio_core::reactor::Core;

    use super::*;
    use req::*;

    fn params() -> RequestParams {
        RequestParams::new("eshost:9200/path")
            .url_param("pretty", true)
            .url_param("q", "*")
    }

    fn expected_req(cli: &Client, method: Method, url: &str, body: Option<Vec<u8>>) -> RequestBuilder {
        let mut req = cli.request(method, url).unwrap();
        {
            req.header(ContentType::json());

            if let Some(body) = body {
                req.body(body);
            }
        }

        req
    }

    fn assert_req(expected: RequestBuilder, actual: RequestBuilder) {
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    fn core() -> Core {
        Core::new().unwrap()
    }

    #[test]
    fn head_req() {
        let cli = Client::new(&core().handle()).unwrap();
        let req = build_req_async(&cli, &params(), PingHeadRequest::new());

        let url = "eshost:9200/path/?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Head, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn get_req() {
        let cli = Client::new(&core().handle()).unwrap();
        let req = build_req_async(&cli, &params(), SimpleSearchRequest::new());

        let url = "eshost:9200/path/_search?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Get, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn post_req() {
        let cli = Client::new(&core().handle()).unwrap();
        let req = build_req_async(&cli,
                            &params(),
                            PercolateRequest::for_index_ty("idx", "ty", vec![]));

        let url = "eshost:9200/path/idx/ty/_percolate?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Post, url, Some(vec![]));

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn put_req() {
        let cli = Client::new(&core().handle()).unwrap();
        let req = build_req_async(&cli,
                            &params(),
                            IndicesCreateRequest::for_index("idx", vec![]));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Put, url, Some(vec![]));

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn delete_req() {
        let cli = Client::new(&core().handle()).unwrap();
        let req = build_req_async(&cli, &params(), IndicesDeleteRequest::for_index("idx"));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Delete, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn owned_string_into_body() {
        String::new().into_body();
    }

    #[test]
    fn borrowed_string_into_body() {
        "abc".into_body();
    }

    #[test]
    fn owned_vec_into_body() {
        Vec::new().into_body();
    }

    #[test]
    fn borrowed_vec_into_body() {
        static BODY: &'static [u8] = &[0, 1, 2];

        (&BODY).into_body();
    }

    #[test]
    fn empty_body_into_body() {
        empty_body().into_body();
    }

    #[test]
    fn json_value_into_body() {
        json!({}).into_body();
    }
}
