#![feature(test)]

extern crate reqwest;
extern crate elastic_reqwest;
extern crate test;

use reqwest::Client;
use reqwest::header::Referer;
use elastic_reqwest::{build_req, RequestParams};
use elastic_reqwest::req::PingRequest;

#[inline(always)]
fn with_headers_1(params: RequestParams) -> RequestParams {
    params.headers(|h| h.set(Referer::new("/People.html#tim")))
}

#[inline(always)]
fn with_headers_5(params: RequestParams) -> RequestParams {
    params.headers(|h| h.set(Referer::new("/People.html#tim")))
          .headers(|h| h.set(Referer::new("/People.html#tim")))
          .headers(|h| h.set(Referer::new("/People.html#tim")))
          .headers(|h| h.set(Referer::new("/People.html#tim")))
          .headers(|h| h.set(Referer::new("/People.html#tim")))
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
    ($({$bench_name:ident, $bench_macro:ident}),*) => (
        $(
            mod $bench_name {
                use super::*;

                $bench_macro!(
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

macro_rules! bench_new {
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

macro_rules! bench_clone {
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

macro_rules! bench_get_headers {
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

macro_rules! bench_make_request {
    ($({ $name:ident, $params:expr }),*) => (
        $(
            #[bench]
            fn $name(b: &mut test::Bencher) {
                let params = RequestParams::default();
                let cli = Client::new().unwrap();

                b.iter(|| {
                    build_req(&cli, &params, PingRequest::new())
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

bench!(
    { new, bench_new },
    { clone, bench_clone },
    { get_headers, bench_get_headers },
    { get_url_query, get_url_query },
    { make_request, bench_make_request }
);
