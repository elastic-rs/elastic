use std::fmt::Debug;

use term_painter::ToStyle;
use term_painter::Color::*;
use futures::{stream, Future, Stream};
use elastic::prelude::*;
use elastic::Error;

pub type TestResult = bool;
pub type Test = Box<Fn(AsyncClient) -> Box<Future<Item = TestResult, Error = ()>>>;

pub trait IntegrationTest: Debug {
    type Response: Debug;

    fn kind() -> &'static str;
    fn name() -> &'static str;

    /// Pre-test preparation.
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>>;

    /// Check an error during preparation and possibly continue.
    fn prepare_err(&self, _err: &Error) -> bool {
        false
    }

    /// Execute requests.
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>>;

    /// Check the response.
    fn assert_ok(&self, _res: &Self::Response) -> bool {
        false
    }

    /// Check an error.
    fn assert_err(&self, _err: &Error) -> bool {
        false
    }
}

pub fn test<T>(client: AsyncClient, test: T) -> Box<Future<Item = TestResult, Error = ()>>
where
    T: IntegrationTest + Send + 'static,
{
    let prefix = format!("{}: {} ({:?}):", T::kind(), T::name(), test);

    let prep_failed = format!("{} prepare failed:", prefix);
    let assert_ok_failed = format!("{} unexpected response:", prefix);
    let assert_err_failed = format!("{} unexpected error:", prefix);
    let ok = format!("{} ok", prefix);

    let fut = test.prepare(client.clone())
        .then(move |prep| match prep {
            Err(ref e) if !test.prepare_err(e) => {
                println!("{} {:?}", Red.bold().paint(prep_failed), e);
                Err(())
            }
            _ => Ok(test),
        })
        .and_then(move |test| {
            test.request(client.clone()).then(move |res| match res {
                Ok(ref res) if !test.assert_ok(res) => {
                    println!("{} {:?}", Red.bold().paint(assert_ok_failed), res);
                    Err(())
                }
                Err(ref e) if !test.assert_err(e) => {
                    println!("{} {:?}", Red.bold().paint(assert_err_failed), e);
                    Err(())
                }
                _ => {
                    println!("{}", Green.paint(ok));
                    Ok(true)
                }
            })
        })
        .then(|outcome| match outcome {
            Err(_) => Ok(false),
            outcome => outcome,
        });

    Box::new(fut)
}

pub fn call(client: AsyncClient, max_concurrent_tests: usize) -> Box<Future<Item = Vec<TestResult>, Error = ()>> {
    use search;

    let search_tests = search::tests().into_iter().map(move |t| t(client.clone()));

    let test_stream = stream::futures_unordered(search_tests)
        .map(|r| Ok(r))
        .buffer_unordered(max_concurrent_tests);

    Box::new(test_stream.collect())
}
