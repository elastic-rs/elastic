/*!
# Integration tests

This crate is intended to provide a suite of integration tests that are run against an Elasticsearch cluster.
They should ensure that `elastic` behaves as expected when making requests, indexing documents, putting mapping etc.
They should also provide a way to inspect how the client behaves under load and where memory is being allocated.
*/

extern crate clap;
extern crate elastic;
#[macro_use]
extern crate elastic_derive;
extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate term_painter;
extern crate tokio_core;
extern crate tokio_timer;

use std::process;
use term_painter::ToStyle;
use term_painter::Color::*;
use clap::{App, Arg};

mod document;
mod search;
mod run_tests;
mod build_client;
mod build_container;
mod wait_until_ready;

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("elastic_integration_tests")
        .arg(
            Arg::with_name("runs")
                .default_value("default")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let mut failed = Vec::<run_tests::TestResult>::new();
    let mut total = 0;

    let runs = matches.values_of("runs").expect("missing `runs` argument");

    for run in runs {
        println!("\n{} tests\n", run);

        let mut core = tokio_core::reactor::Core::new().unwrap();
        let client = build_client::call(&core.handle(), run).unwrap();

        // Build and start a container to run tests against
        build_container::start(run).unwrap();

        // Wait until the container is ready
        core.run(wait_until_ready::call(client.clone(), 60))
            .unwrap();

        // Run the integration tests
        let results = core.run(run_tests::call(client, 8)).unwrap();
        failed.extend(results.iter().filter(|success| **success == false));
        total += results.len();

        // Kill the container
        build_container::kill(run).unwrap();
    }

    if failed.len() > 0 {
        println!(
            "\n{}",
            Red.bold()
                .paint(format!("{} of {} tests failed", failed.len(), total))
        );
        process::exit(1);
    } else {
        println!("\n{}", Green.paint(format!("all {} tests passed", total)));
        process::exit(0);
    }
}
