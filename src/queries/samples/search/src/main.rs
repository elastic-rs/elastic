#![feature(ip)]

extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate elastic;
extern crate timestrings;
extern crate futures;
extern crate tokio_core;
extern crate futures_cpupool;
extern crate elastic_queries;

use std::error::Error;
use futures::Future;
use tokio_core::reactor::Core;
use futures_cpupool::CpuPool;
use elastic::prelude::*;

use serde_json::Value;
use timestrings::ts;
use elastic_queries::prelude::*;

fn run() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;
    let pool = CpuPool::new(4);

    let client = AsyncClientBuilder::new().serde_pool(pool).build(
        &core.handle(),
    )?;

    let gte = ts("now-1d").unwrap();
    let lte = ts("now").unwrap();

    let mut query = QueryBuilder::default()
        .size(Some(10))
        .build()
        .expect("could not build query");

    let f = RangeFilter::new("@timestamp",
                     RangeParamsBuilder::default()
                         .gte(Some(gte.into()))
                         .lte(Some(lte.into()))
                         .format(Some(EsDateFormat::epoch_second))
                         .build()
                         .unwrap()
    );

    query.add_filter(BoolQuerySections::Must,
                     f.into());

    let res_future = client
        .search::<Value>()
        .index("_all")
        .body(query.to_string()?)
        .send();


    let search_future = res_future.and_then(|res| {
        // Iterate through the hits in the response.
        for hit in res.hits() {
            println!("{:?}", hit);
        }

        Ok(())
    });

    core.run(search_future)?;

    Ok(())
}

fn main() {
    env_logger::init();
    run().unwrap()
}
