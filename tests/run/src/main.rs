/*!
# Integration tests

This crate is intended to provide a suite of integration tests that are run against an Elasticsearch cluster.
They should ensure that `elastic` behaves as expected when making requests, indexing documents, putting mapping etc.
They should also provide a way to inspect how the client behaves under load and where memory is being allocated.
*/

extern crate elastic;
#[macro_use]
extern crate elastic_derive;
extern crate futures;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate term_painter;
extern crate tokio;
extern crate tokio_timer;

use std::process;
use term_painter::ToStyle;
use term_painter::Color::*;
use tokio::runtime::current_thread::block_on_all;

mod document;
mod search;
mod index;
mod run_tests;
mod build_client;
mod build_container;
mod wait_until_ready;

fn main() {
    use std::thread;
    use std::time::Duration;

    let run = "default";
    let client = build_client::call(run).unwrap();

    // Build and start a container to run tests against
    build_container::start(run).unwrap();

    // Wait until the container is ready
    // TODO
    thread::sleep(Duration::from_secs(60));

    // Run the integration tests
    let results = block_on_all(run_tests::call(client, 8)).unwrap();
    let failed: Vec<_> = results
        .iter()
        .filter(|success| **success == false)
        .collect();

    // Kill the container
    build_container::kill(run).unwrap();

    if failed.len() > 0 {
        println!(
            "{}",
            Red.bold().paint(format!(
                "{} of {} tests failed",
                failed.len(),
                results.len()
            ))
        );
        process::exit(1);
    } else {
        println!(
            "{}",
            Green.paint(format!("all {} tests passed", results.len()))
        );
        process::exit(0);
    }
}
