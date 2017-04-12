use std::str::FromStr;
use std::fmt::Debug;

use futures::Stream;
use hyper::{Method, Uri, Error as HyperError};
use hyper::header::ContentType;
use hyper::client::Request;

use elastic_requests::{HttpRequest, HttpMethod};

/// Build a `hyper` request from an `elastic` request.
pub fn build<I, B>(base_url: &str, req: I) -> Request<B>
    where I: Into<HttpRequest<'static, B>>,
          B: Stream<Error = HyperError> + 'static + Debug,
          B::Item: AsRef<[u8]>
{
    let req = req.into();

    // Build the request url
    let mut url = String::with_capacity(base_url.len() + req.url.len());

    url.push_str(base_url);
    url.push_str(&req.url);

    let url = Uri::from_str(&url).unwrap();

    let method = req.method;
    let body = req.body;

    // Build the HTTP request
    let mut req = match method {
        HttpMethod::Get => Request::<B>::new(Method::Get, url),
        HttpMethod::Post => {
            let mut req = Request::<B>::new(Method::Post, url);

            if let Some(body) = body {
                req.set_body(body);
            }

            req
        }
        _ => unimplemented!(),
    };

    // Set content type header
    {
        let mut headers = req.headers_mut();
        headers.set(ContentType::json())
    }

    req
}
