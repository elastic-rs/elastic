/*!
# Integration tests

This crate is intended to provide a suite of integration tests that are run against an Elasticsearch cluster.
They should ensure that `elastic` behaves as expected when making requests, indexing documents, putting mapping etc.
They should also provide a way to inspect how the client behaves under load and where memory is being allocated.
*/

extern crate term_painter;
extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate elastic;
#[macro_use]
extern crate elastic_derive;

use std::process;
use std::fmt::Debug;

use term_painter::ToStyle;
use term_painter::Color::*;
use futures::{Future, IntoFuture};
use serde_json::Value;
use elastic::prelude::*;
use elastic::error::Error;

mod search;

type Test = Box<Fn(AsyncClient) -> Box<Future<Item = bool, Error = ()>>>;

trait IntegrationTest: Debug {
    type Response: Debug;

    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>>;

    fn prepare_err(&self, err: &Error) -> bool {
        false
    }

    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>>;

    fn assert_ok(&self, res: &Self::Response) -> bool {
        false
    }

    fn assert_err(&self, err: &Error) -> bool {
        false
    }
}

fn run_test<T>(client: AsyncClient, test: T) -> Box<Future<Item = bool, Error = ()>> 
    where T: IntegrationTest + Send + 'static
{
    let fut = test
        .prepare(client.clone())
        .then(move |prep| {
            match prep {
                Err(ref e) if !test.prepare_err(e) => {
                    println!("{} {:?}", Red.bold().paint(format!("{:?}: prepare failed:", test)), e);
                    Err(())
                },
                _ => Ok(test)
            }
        })
        .and_then(move |test| {
            test.request(client.clone())
                .then(move |res| {
                    match res {
                        Ok(ref res) if !test.assert_ok(res) => {
                            println!("{} {:?}", Red.bold().paint(format!("{:?}: unexpected response:", test)), res);
                            Err(())
                        },
                        Err(ref e) if !test.assert_err(e) => {
                            println!("{} {:?}", Red.bold().paint(format!("{:?}: unexpected error:", test)), e);
                            Err(())
                        },
                        _ => {
                            println!("{}", Green.paint(format!("{:?}: ok", test)));
                            Ok(true)
                        }
                    }
                })
        })
        .then(|outcome| {
            match outcome {
                Err(_) => Ok(false),
                outcome => outcome
            }
        });

    Box::new(fut)
}

fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = AsyncClientBuilder::new().build(&core.handle()).unwrap();

    let search_tests = search::tests()
        .into_iter()
        .map(|test| test(client.clone()));

    let tests = futures::future::join_all(search_tests);

    let results = core.run(tests).unwrap();
    let failed: Vec<_> = results.iter().filter(|success| **success == false).collect();

    if failed.len() > 0 {
        println!("{}", Red.bold().paint(format!("{} of {} tests failed", failed.len(), results.len())));
        process::exit(1);
    } else {
        println!("{}", Green.paint(format!("all {} tests passed", results.len())));
        process::exit(0);
    }
}
