use std::fmt::Debug;

use term_painter::ToStyle;
use term_painter::Color::*;
use futures::{future, Future};
use elastic::prelude::*;
use elastic::error::Error;

pub type TestResult = bool;
pub type Test = Box<Fn(AsyncClient) -> Box<Future<Item = TestResult, Error = ()>>>;

pub trait IntegrationTest: Debug {
    type Response: Debug;

    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>>;

    fn prepare_err(&self, _err: &Error) -> bool {
        false
    }

    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>>;

    fn assert_ok(&self, _res: &Self::Response) -> bool {
        false
    }

    fn assert_err(&self, _err: &Error) -> bool {
        false
    }
}

pub fn test<T>(client: AsyncClient, test: T) -> Box<Future<Item = TestResult, Error = ()>> 
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

pub fn call(client: AsyncClient) -> Box<Future<Item = Vec<TestResult>, Error = ()>> {
    use search;

    let search_tests = search::tests()
        .into_iter()
        .map(move |t| t(client.clone()));

    Box::new(future::join_all(search_tests))
}
