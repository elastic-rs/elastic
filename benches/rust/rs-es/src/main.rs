#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate rs_es;

extern crate measure;

use rs_es::Client;
use rs_es::query::Query;
use rs_es::operations::search::SearchResult;

#[derive(Debug, Deserialize)]
struct BenchDoc {
    pub id: i32,
    pub title: String,
    pub timestamp: i64,
}

fn main() {
    let runs = measure::parse_runs_from_env();

    let mut client = Client::new("http://localhost:9200").unwrap();
    let qry = Query::build_query_string("*").build();
    
    let results = measure::run(runs, || {
        let res: SearchResult<BenchDoc> = client.search_query()
            .with_indexes(&["bench_index"])
            .with_types(&["bench_doc"])
            .with_query(&qry)
            .with_size(10)
            .send()
            .unwrap();

        res
    });

    println!("{}", results);
}
