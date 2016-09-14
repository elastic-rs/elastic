#![feature(test, plugin, custom_derive, iter_arith)]
#![plugin(serde_macros, json_str)]

extern crate time;
extern crate test;
extern crate serde;
extern crate serde_json;
extern crate stopwatch;
extern crate hyper;
extern crate elastic_types;
extern crate elastic_hyper as elastic;

use std::env;
use time::Duration;
use stopwatch::Stopwatch;
use hyper::header::Connection;
use elastic_types::date::prelude::*;

mod response;
use self::response::*;

#[derive(Deserialize)]
struct BenchDoc {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<EpochMillis>
}

fn main() {
    let mut args = env::args();
	let _ = args.next().unwrap();
	let runs = {
        if args.len() >= 1 {
            args.next().unwrap().parse::<i32>().unwrap()
        }
        else {
            200
        }
    };

    let (mut client, mut params) = elastic::default();
    params.headers.set(Connection::keep_alive());

    let mut results = Vec::<i64>::with_capacity(runs as usize);
    for _ in 0..runs {
        let mut sw = Stopwatch::start_new();

        let res: SearchResponse<BenchDoc> = serde_json::de::from_reader(
            elastic::search::post_index_type(
        		&mut client, &params,
                "bench_index", "bench_doc",
        		json_lit!({
        			query: {
        				query_string: {
        					query: "*"
        				}
        			},
                    size: 10
        		})
        	).unwrap()
        ).unwrap();

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
    ].iter().map(|p| {
        let p: f32 = *p;
        let i: usize = (p * runs) as usize;
        (p, data.get(i - 1).unwrap().to_owned())
    }).collect()
}
