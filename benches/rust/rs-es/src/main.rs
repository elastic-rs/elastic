#![feature(test, plugin, custom_derive, iter_arith)]
#![plugin(serde_macros)]

extern crate test;

extern crate serde;
extern crate stopwatch;
extern crate rs_es;

use std::env;
use stopwatch::Stopwatch;
use rs_es::Client;
use rs_es::query::Query;
use rs_es::operations::search::SearchResult;

#[derive(Deserialize)]
struct BenchDoc {
    pub id: i32,
    pub title: String,
    pub timestamp: i64
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

    let mut client = Client::new("localhost", 9200);

    let mut results = Vec::<i64>::with_capacity(runs as usize);
    for _ in 0..runs {
        let mut sw = Stopwatch::start_new();

        let res: SearchResult<BenchDoc> = client.search_query()
            .with_indexes(&["bench_index"])
            .with_types(&["bench_doc"])
            .with_query(&Query::build_query_string("*").build())
            .with_size(10)
            .send()
            .unwrap();

        sw.stop();

        test::black_box(res);

        results.push(sw.elapsed().num_nanoseconds().unwrap());
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
