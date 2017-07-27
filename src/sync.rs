/*! Synchronous http client. */

use std::ops::Deref;
use std::io::Cursor;
use std::fs::File;
use serde::de::DeserializeOwned;
use serde_json::Value;
use reqwest::{Client, RequestBuilder, Response, Body};

use super::req::HttpRequest;
use super::res::parsing::{Parse, IsOk};
use super::{Error, RequestParams, build_url, build_method};

/** Get a default `Client` and `RequestParams`. */
pub fn default() -> Result<(Client, RequestParams), Error> {
    Client::new()
        .map(|cli| (cli, RequestParams::default()))
        .map_err(Into::into)
}

/** A type that can be converted into a request body. */
pub struct SyncBody(Body);

impl SyncBody {
    /** Convert the body into its inner value. */
    pub fn into_inner(self) -> Body {
        self.0
    }
}

impl Deref for SyncBody {
    type Target = Body;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Body> for SyncBody {
    fn from(body: Body) -> SyncBody {
        SyncBody(body)
    }
}

impl From<File> for SyncBody {
    fn from(body: File) -> SyncBody {
        SyncBody(body.into())
    }
}

impl From<Vec<u8>> for SyncBody {
    fn from(body: Vec<u8>) -> SyncBody {
        SyncBody(body.into())
    }
}

impl From<String> for SyncBody {
    fn from(body: String) -> SyncBody {
        SyncBody(body.into())
    }
}

impl From<Value> for SyncBody {
    fn from(body: Value) -> SyncBody {
        SyncBody(body.to_string().into())
    }
}

impl From<&'static [u8]> for SyncBody {
    fn from(body: &'static [u8]) -> SyncBody {
        SyncBody(Body::new(Cursor::new(body)))
    }
}

impl From<&'static str> for SyncBody {
    fn from(body: &'static str) -> SyncBody {
        SyncBody(Body::new(Cursor::new(body)))
    }
}

/** Represents a client that can send Elasticsearch requests synchronously. */
pub trait SyncElasticClient {
    /** 
    Send a request and get a response.
    
    # Examples
    
    Bring the `SyncElasticClient` trait into scope and call `elastic_req` with any type that
    can be converted into a `req::HttpRequest`.
    This method returns a raw `reqwest::Response`.
    
    ```no_run
    # extern crate elastic_reqwest;
    # use elastic_reqwest::req::SimpleSearchRequest;
    # fn main () {
    # let request = SimpleSearchRequest::for_index_ty("myindex", "mytype");
    use elastic_reqwest::SyncElasticClient;
    
    let (client, params) = elastic_reqwest::sync::default().unwrap();
    
    let http_res = client.elastic_req(&params, request).unwrap();
    # }
    ```
    */
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Result<Response, Error>
        where I: Into<HttpRequest<'static, B>>,
              B: Into<SyncBody>;
}

/** Build a synchronous `reqwest::RequestBuilder` from an Elasticsearch request. */
pub fn build_req_sync<I, B>(client: &Client, params: &RequestParams, req: I) -> Result<RequestBuilder, Error>
    where I: Into<HttpRequest<'static, B>>,
          B: Into<SyncBody>
{
    let req = req.into();

    let url = build_url(&req.url, &params);
    let method = build_method(req.method);
    let body = req.body;

    let mut req = client.request(method, &url)?;
    {
        req.headers(params.get_headers());

        if let Some(body) = body {
            req.body(body.into().into_inner());
        }
    }

    Ok(req)
}

impl SyncElasticClient for Client {
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Result<Response, Error>
        where I: Into<HttpRequest<'static, B>>,
              B: Into<SyncBody>
    {
        build_req_sync(&self, params, req)?.send().map_err(Into::into)
    }
}

/** Represents a response that can be parsed into a concrete Elasticsearch response. */
pub trait SyncFromResponse<TResponse> {
    /** Parse a response into a concrete response type. */
    fn from_response(self, response: Response) -> Result<TResponse, Error>;
}

impl<TResponse: IsOk + DeserializeOwned> SyncFromResponse<TResponse> for Parse<TResponse> {
    fn from_response(self, response: Response) -> Result<TResponse, Error> {
        let status: u16 = response.status().into();

        self.from_reader(status, response).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, RequestBuilder, Method};
    use reqwest::header::ContentType;
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

    #[test]
    fn head_req() {
        let cli = Client::new().unwrap();
        let req = build_req_sync(&cli, &params(), PingHeadRequest::new());

        let url = "eshost:9200/path/?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Head, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn get_req() {
        let cli = Client::new().unwrap();
        let req = build_req_sync(&cli, &params(), SimpleSearchRequest::new());

        let url = "eshost:9200/path/_search?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Get, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn post_req() {
        let cli = Client::new().unwrap();
        let req = build_req_sync(&cli,
                            &params(),
                            PercolateRequest::for_index_ty("idx", "ty", vec![]));

        let url = "eshost:9200/path/idx/ty/_percolate?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Post, url, Some(vec![]));

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn put_req() {
        let cli = Client::new().unwrap();
        let req = build_req_sync(&cli,
                            &params(),
                            IndicesCreateRequest::for_index("idx", vec![]));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Put, url, Some(vec![]));

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn delete_req() {
        let cli = Client::new().unwrap();
        let req = build_req_sync(&cli, &params(), IndicesDeleteRequest::for_index("idx"));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Delete, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn file_into_body() {
        SyncBody::from(File::open("Cargo.toml").unwrap());
    }

    #[test]
    fn owned_string_into_body() {
        SyncBody::from(String::new());
    }

    #[test]
    fn borrowed_string_into_body() {
        SyncBody::from("abc");
    }

    #[test]
    fn owned_vec_into_body() {
        SyncBody::from(Vec::new());
    }

    #[test]
    fn borrowed_vec_into_body() {
        static BODY: &'static [u8] = &[0, 1, 2];

        SyncBody::from(BODY);
    }

    #[test]
    fn empty_body_into_body() {
        SyncBody::from(empty_body());
    }

    #[test]
    fn json_value_into_body() {
        SyncBody::from(json!({}));
    }
}
