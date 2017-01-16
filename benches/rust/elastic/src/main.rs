#![feature(test, plugin)]
#![plugin(json_str)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

extern crate stopwatch;
extern crate time;
extern crate test;

extern crate serde;
extern crate elastic;

use std::env;
use time::Duration;
use stopwatch::Stopwatch;

use elastic::http;
use elastic::prelude::*;

// TODO: Replace with elastic_responses
mod response;
use self::response::*;

#[derive(Debug, Deserialize)]
struct BenchDoc {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<EpochMillis>,
}

lazy_static!(
    static ref REQ: SearchRequest<'static> = {
        SearchRequest::for_index_ty(
            "bench_index", "bench_doc",
            json_lit!({
                query: {
                    query_string: {
                        query: "*"
                    }
                },
                size: 10
            }))
        };
);

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let runs = {
        if args.len() >= 1 {
            args.next().unwrap().parse::<i32>().unwrap()
        } else {
            200
        }
    };

    let params = RequestParams::default().header(http::header::Connection::keep_alive());
    let client = Client::new(params).unwrap();

    let mut results = Vec::<i64>::with_capacity(runs as usize);
    for _ in 0..runs {
        let mut sw = Stopwatch::start_new();

        let req: &SearchRequest<'static> = &REQ;
        let res: SearchResponse<BenchDoc> = client.request(req).send().unwrap().json().unwrap();

        sw.stop();

        test::black_box(res);

        let elapsed = Duration::from_std(sw.elapsed()).unwrap();
        results.push(elapsed.num_nanoseconds().unwrap());
    }

    results.sort();

    let mean: i64 = results.iter().sum();
    println!("took mean {}ns", mean / (runs as i64));

    let pv = percentiles(&results, runs as f32);

    for (p, n) in pv {
        println!("Percentile {}%: {}ns", p * 100f32, n);
    }
}

fn percentiles(data: &Vec<i64>, runs: f32) -> Vec<(f32, i64)> {
    vec![
        0.50,
        0.66,
        0.75,
        0.80,
        0.90,
        0.95,
        0.98,
        0.99,
        1.00
    ]
        .iter()
        .map(|p| {
            let p: f32 = *p;
            let i: usize = (p * runs) as usize;
            (p, data.get(i - 1).unwrap().to_owned())
        })
        .collect()
}
