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

use std::fmt::Debug;
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
                    println!("{:?}: prepare failwhale: {:?}", test, e);
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
                            println!("{:?}: unexpected response failwhale: {:?}", test, res);
                            Ok(false)
                        },
                        Err(ref e) if !test.assert_err(e) => {
                            println!("{:?}: unexpected error failwhale: {:?}", test, e);
                            Ok(false)
                        },
                        _ => {
                            println!("{:?}: ok", test);
                            Ok(true)
                        }
                    }
                })
        })
        .map_err(|e| ());

    Box::new(fut)
}

fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let client = AsyncClientBuilder::new().build(&core.handle()).unwrap();

    let search_tests = search::tests()
        .into_iter()
        .map(|test| test(client.clone()));

    let tests = futures::future::join_all(search_tests);

    core.run(tests).unwrap();
}
