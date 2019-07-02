use std::fmt::Debug;

use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use futures::{
    stream,
    Future,
    Stream,
};
use term_painter::{
    Color::*,
    ToStyle,
};

pub type TestResult = bool;
pub type Test = Box<dyn Fn(AsyncClient) -> Box<dyn Future<Item = TestResult, Error = ()>>>;

pub trait IntegrationTest {
    #[doc(hidden)]
    const path: &'static str;

    const description: &'static str;

    type Response: Debug;

    /// Pre-test preparation.
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>>;

    /// Check an error during preparation and possibly continue.
    fn prepare_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::IndexNotFound { .. }) => true,
            _ => false,
        }
    }

    /// Execute requests.
    fn request(&self, client: AsyncClient)
        -> Box<dyn Future<Item = Self::Response, Error = Error>>;

    /// Check the response.
    fn assert_ok(&self, _res: &Self::Response) -> bool {
        false
    }

    /// Check an error.
    fn assert_err(&self, _err: &Error) -> bool {
        false
    }
}

pub fn test<T>(client: AsyncClient, test: T) -> Box<dyn Future<Item = TestResult, Error = ()>>
where
    T: IntegrationTest + Send + 'static,
{
    let prefix = format!("{} ({}):", T::path, T::description);

    let prep_failed = format!("{} prepare failed:", prefix);
    let assert_ok_failed = format!("{} unexpected response:", prefix);
    let assert_err_failed = format!("{} unexpected error:", prefix);
    let ok = format!("{} ok", prefix);

    let fut = test
        .prepare(client.clone())
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

pub fn call(
    client: AsyncClient,
    cases: impl IntoIterator<Item = Test>,
    max_concurrent_tests: usize,
) -> Result<Vec<TestResult>, ()> {
    tokio::runtime::current_thread::block_on_all(call_future(client, cases, max_concurrent_tests))
}

fn call_future(
    client: AsyncClient,
    cases: impl IntoIterator<Item = Test>,
    max_concurrent_tests: usize,
) -> Box<dyn Future<Item = Vec<TestResult>, Error = ()>> {
    let all_tests = cases
        .into_iter()
        .map(move |t| t(client.clone()))
        .collect::<Vec<_>>();

    println!("Running {} tests", all_tests.len());

    let test_stream = stream::futures_unordered(all_tests)
        .map(|r| Ok(r))
        .buffer_unordered(max_concurrent_tests);

    Box::new(test_stream.collect())
}
