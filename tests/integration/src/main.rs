/*!
# Integration tests

This crate is intended to provide a suite of integration tests that are run against an Elasticsearch cluster.
They should ensure that `elastic` behaves as expected when making requests, indexing documents, putting mapping etc.
They should also provide a way to inspect how the client behaves under load and where memory is being allocated.
*/

#![allow(non_upper_case_globals)]

#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use clap::{
    App,
    Arg,
};
use std::process;
use term_painter::{
    Color::*,
    ToStyle,
};

mod build_client;
mod build_container;
mod run_tests;
mod tests;
mod wait_until_ready;

fn main() {
    env_logger::init_from_env("ELASTIC_LOG");

    let matches = App::new("elastic_integration_tests")
        .arg(
            Arg::with_name("runs")
                .default_value("default")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("filter")
                .takes_value(true)
                .short("f")
                .long("filter"),
        )
        .get_matches();

    let mut failed = Vec::<run_tests::TestResult>::new();
    let mut total = 0;

    let runs = matches.values_of("runs").expect("missing `runs` argument");
    let filter = matches.value_of("filter");

    for run in runs {
        println!("\n{} tests\n", run);

        let client = build_client::call(run).unwrap();

        // Build and start a container to run tests against
        build_container::start(run).unwrap();

        // Wait until the container is ready
        wait_until_ready::call(client.clone(), 60).unwrap();

        // Apply the first argument as a filter
        let cases = crate::tests::all()
            .into_iter()
            .filter_map(|(meta, test)| match filter {
                Some(ref filter) => {
                    if meta.starts_with(filter) {
                        Some(test)
                    } else {
                        None
                    }
                }
                None => Some(test),
            });

        // Run the integration tests
        let results = run_tests::call(client, cases, 8).unwrap();
        failed.extend(results.iter().filter(|success| !(**success)));
        total += results.len();

        // Kill the container
        build_container::kill(run).unwrap();
    }

    if !failed.is_empty() {
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
