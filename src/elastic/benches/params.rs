#![feature(test)]

extern crate reqwest;
extern crate tokio_core;
extern crate elastic;
extern crate test;

use reqwest::Client as ClientSync;
use reqwest::unstable::async::Client as ClientAsync;
use reqwest::header::Referer;
use tokio_core::reactor::Core;
use elastic::client::RequestParams;
use elastic::client::sync::build_req as build_req_sync;
use elastic::client::async::build_req as build_req_async;
use elastic::client::req::PingRequest;

#[inline(always)]
fn with_headers_1(params: RequestParams) -> RequestParams {
    params.header(Referer::new("/People.html#tim"))
}

#[inline(always)]
fn with_headers_5(params: RequestParams) -> RequestParams {
    params.header(Referer::new("/People.html#tim"))
          .header(Referer::new("/People.html#tim"))
          .header(Referer::new("/People.html#tim"))
          .header(Referer::new("/People.html#tim"))
          .header(Referer::new("/People.html#tim"))
}

#[inline(always)]
fn with_params_1(params: RequestParams) -> RequestParams {
    params.url_param("query", "*")
}

#[inline(always)]
fn with_params_5(params: RequestParams) -> RequestParams {
    params.url_param("query", "*")
          .url_param("query", "*")
          .url_param("query", "*")
          .url_param("query", "*")
          .url_param("query", "*")
}

macro_rules! bench {
    ($($bench_name:ident),*) => (
        $(
            mod $bench_name {
                use super::*;

                $bench_name!(
                    { default, RequestParams::default() },
                    { headers_1, with_headers_1(RequestParams::default()) },
                    { headers_5, with_headers_5(RequestParams::default()) },
                    { params_1, with_params_1(RequestParams::default()) },
                    { params_5, with_params_5(RequestParams::default()) }
                );
            }
        )*
    )
}

macro_rules! new {
    ($({ $name:ident, $params:expr }),*) => (
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                b.iter(|| {
                    $params
                })
            }
        )*
    )
}

macro_rules! clone {
    ($({ $name:ident, $params:expr }),*) => (
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                let params = $params;

                b.iter(|| {
                    params.clone()
                })
            }
        )*
    )
}

macro_rules! get_headers {
    ($({ $name:ident, $params:expr }),*) => (
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                let params = $params;

                b.iter(|| {
                    params.get_headers()
                })
            }
        )*
    )
}

macro_rules! build_request_sync {
    ($({ $name:ident, $params:expr }),*) => (
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                let params = RequestParams::default();
                let cli = ClientSync::new();

                b.iter(|| {
                    build_req_sync(&cli, &params, PingRequest::new())
                })
            }
        )*
    )
}

macro_rules! build_request_async {
    ($({ $name:ident, $params:expr }),*) => (
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                let core = Core::new().unwrap();

                let params = RequestParams::default();
                let cli = ClientAsync::new(&core.handle());

                b.iter(|| {
                    build_req_async(&cli, &params, PingRequest::new())
                })
            }
        )*
    )
}

macro_rules! get_url_query {
    ($({ $name:ident, $params:expr }),*) => (
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                let params = $params;

                b.iter(|| {
                    params.get_url_qry()
                })
            }
        )*
    )
}

bench![
    new,
    clone,
    get_headers,
    get_url_query,
    build_request_sync,
    build_request_async
];
