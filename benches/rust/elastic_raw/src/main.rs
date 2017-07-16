#![feature(plugin)]
#![plugin(json_str)]

extern crate elastic;

extern crate measure;

use std::io::Read;

use elastic::http;
use elastic::prelude::*;

static BODY: &'static str = json_lit!(
    {
        query: {
            query_string: {
                query: "*"
            }
        },
        size: 10
    }
);

fn main() {
    let runs = measure::parse_runs_from_env();

    let client = ClientBuilder::new()
        .params(|p| p.header(http::header::Connection::keep_alive()))
        .build()
        .unwrap();
    
    let results = measure::run(runs, || {
        let req = SearchRequest::for_index_ty("bench_index", "bench_doc", BODY);
        let mut res = client.request(req).send().unwrap().into_raw();

        let mut buf = Vec::new();
        res.read_to_end(&mut buf).unwrap();

        buf
    });

    println!("{}", results);
}
