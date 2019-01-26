#![feature(test)]

extern crate elastic;
extern crate reqwest;
extern crate test;
extern crate tokio_core;

use elastic::client::requests::PingRequest;
use elastic::client::RequestParams;
use reqwest::header::Referer;
use tokio_core::reactor::Core;

#[inline(always)]
fn with_headers_1(params: RequestParams) -> RequestParams {
    params.header(Referer::new("/People.html#tim"))
}

#[inline(always)]
fn with_headers_5(params: RequestParams) -> RequestParams {
    params
        .header(Referer::new("/People.html#tim"))
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
    params
        .url_param("query", "*")
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

bench! {
    new,
    clone,
    get_headers,
    get_url_query
}
