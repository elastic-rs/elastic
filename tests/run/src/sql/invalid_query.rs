use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use futures::Future;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct InvalidQuery;

const INDEX: &'static str = "no_sql_index_idx";

impl IntegrationTest for InvalidQuery {
    type Response = SqlResponse;

    fn kind() -> &'static str {
        "sql"
    }
    fn name() -> &'static str {
        "invalid query"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client.index(INDEX).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Execute a search request against that index
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.sql_query(&format!("select * from {}", INDEX)).send();

        Box::new(res)
    }

    // Ensure a `Parsing` error is returned
    fn assert_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::Verification { .. }) => true,
            _ => false,
        }
    }
}
