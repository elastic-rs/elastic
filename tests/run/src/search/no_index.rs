use elastic::error::{
    ApiError,
    Error,
};
use elastic::prelude::*;
use futures::Future;
use run_tests::IntegrationTest;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct NoIndex;

const INDEX: &'static str = "no_index_idx";

impl IntegrationTest for NoIndex {
    type Response = SearchResponse<Value>;

    fn kind() -> &'static str {
        "search"
    }
    fn name() -> &'static str {
        "no index"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client.index(INDEX).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Execute a search request against that index
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.search().index(INDEX).ty("no_index_ty").send();

        Box::new(res)
    }

    // Ensure an `IndexNotFound` error is returned
    fn assert_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::IndexNotFound { .. }) => true,
            _ => false,
        }
    }
}
