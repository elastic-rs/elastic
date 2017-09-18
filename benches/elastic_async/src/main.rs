#![feature(plugin)]
#![plugin(json_str)]

#[macro_use]
extern crate elastic_derive;
extern crate futures;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

extern crate elastic;
extern crate serde;

extern crate measure;

use elastic::http;
use elastic::prelude::*;

#[derive(Debug, Deserialize, ElasticType)]
#[elastic(mapping = "BenchDocMapping")]
struct BenchDoc {
    pub id: i32,
    pub title: String,
    pub timestamp: Date<DefaultDateMapping<EpochMillis>>,
}

#[derive(Default)]
struct BenchDocMapping;
impl DocumentMapping for BenchDocMapping {
    fn name() -> &'static str {
        "bench_doc"
    }
}

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

    let client = AsyncClientBuilder::new()
        .params(|p| p.header(http::header::Connection::keep_alive()))
        .build(&core.handle())
        .unwrap();

    let results_future = measure::run_future(runs, || {
        client
            .search::<BenchDoc>()
            .index("bench_index")
            .body(BODY)
            .send()
    });

    results = core.run(results_future).unwrap();

    println!("{}", results);
}
