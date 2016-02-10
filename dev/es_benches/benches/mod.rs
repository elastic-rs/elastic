#![feature(test, plugin)]
#![plugin(elastic_types_codegen)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate elastic_types_codegen;
extern crate rs_es;
extern crate hyper;

extern crate test;

use std::io::Read;
use test::Bencher;

use hyper::header::{ Headers, ContentType };
use hyper::client::response::Response;
use hyper::error::Result;
use rs_es::query::*;

#[bench]
fn rs_es_query(b: &mut Bencher) {
    let mut client = rs_es::Client::new("localhost", "9200");
	b.iter(|| {
        client.search_query()
            .with_indexes(&["bench_index"])
            .with_types(&["docs"])
            .with_query(
                Query::build_query_string("doc")
                    .with_default_field("title")
                    .build()
            )
            .send();
    });
}

#[bench]
fn elastic_hyper_query(b: &mut Bencher) {
    let mut client = hyper::Client::new();
    
    b.iter(|| {
        post_index_type(
            client, "http://localhost:9200", "bench_index", "docs", 
            json!({
                query: {
                    query_string: {
                    default_field: "title",
                    query: "doc"
                    }
                }
            })
        );
    });
}

fn post_index_type(client: hyper::Client, baseurl: &str, index: &str, _type: &str, body: String) -> Result<Response> {
    let mut url = String::with_capacity(
        baseurl.len() +
        "/".len() + 
        index.len() + 
        "/".len() +
        _type.len()
    );

    //Push the parts/params in order
    url.push_str(&baseurl);
    url.push_str("/");
    url.push_str(&index);
    url.push_str("/");
    url.push_str(&_type);

    let mut headers = Headers::new();
    headers.set(ContentType::json());
    
    let mut res = client.post(&url)
        .headers(headers)
        .body(body);
        
    res.send()
}