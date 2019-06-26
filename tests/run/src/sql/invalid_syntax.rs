use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use futures::{
    future,
    Future,
};
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct InvalidSyntax;

impl IntegrationTest for InvalidSyntax {
    type Response = SqlResponse;

    fn kind() -> &'static str {
        "sql"
    }
    fn name() -> &'static str {
        "invalid syntax"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, _client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        Box::new(future::ok(()))
    }

    // Execute a search request against that index
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.sql_query("select").send();

        Box::new(res)
    }

    // Ensure a `Parsing` error is returned
    fn assert_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::Parsing { .. }) => true,
            _ => false,
        }
    }
}
