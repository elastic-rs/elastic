#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_types_derive;
#[macro_use]
extern crate json_str;

extern crate elastic_requests as req;
extern crate elastic_responses as res;
extern crate elastic_types;

extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate tokio_core;

extern crate hyper;
extern crate futures_cpupool;

use std::str;
use std::io::Read;
use std::borrow::Cow;

use req::{RawBody, SearchRequest};

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures_cpupool::CpuPool;

use hyper::{Client, Body, Method};
use hyper::client::Request;

fn main() {
    let url = "http://localhost:9200";

    let mut core = Core::new().unwrap();

    let client = Client::new(&core.handle());
    let pool = CpuPool::new(4);

    let req = SearchRequest::for_index("_all", r#"{ "query": { "match_all": { } } }"#);

    let req = client.request(hyper_req(&url, req))
        .and_then(|res| {
            res.body()
                .fold(Vec::new(),
                      |mut buf, chunk| chunk.as_ref().read_to_end(&mut buf).map(|_| buf))
                .and_then(|buf| {
                    // TODO: Deserialize the response on the cpu pool

                    println!("{:?}", str::from_utf8(&buf).unwrap());

                    futures::finished(())
                })
        });

    core.run(req).unwrap();
}

fn hyper_req<I>(base_url: &str, req: I) -> Request
    where I: Into<req::HttpRequest<'static>>
{
    let req = req.into();

    let mut url = String::with_capacity(base_url.len() + req.url.len());

    url.push_str(base_url);
    url.push_str(&req.url);

    let url = hyper::Url::parse(&url).unwrap();

    let method = req.method;
    let body = req.body;

    match method {
        req::HttpMethod::Get => Request::new(Method::Get, url),
        req::HttpMethod::Post => {
            let mut req = Request::new(Method::Post, url);

            if let Some(body) = body {
                let body = body.into_raw();

                let body: Body = match body {
                    Cow::Borrowed(b) => Body::from(b),
                    Cow::Owned(b) => Body::from(b),
                };
                
                req.set_body(body);
            }

            req
        }
        _ => unimplemented!(),
    }
}
