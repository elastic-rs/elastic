#![feature(plugin)]
#![plugin(json_str)]

#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;

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

    let client = SyncClientBuilder::new()
        .params(|p| p.header(http::header::Connection::keep_alive()))
        .build()
        .unwrap();

    let results = measure::run(runs, || {
        client
            .search::<BenchDoc>()
            .index("bench_index")
            .body(BODY)
            .send()
            .unwrap()
    });

    println!("{}", results);
}
