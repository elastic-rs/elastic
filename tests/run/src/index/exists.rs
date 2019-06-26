use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct Exists;

const INDEX: &'static str = "index_exists";

impl IntegrationTest for Exists {
    type Response = IndicesExistsResponse;

    fn kind() -> &'static str {
        "index"
    }
    fn name() -> &'static str {
        "exists"
    }

    // Ensure the index exists
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let create_res = client.index(INDEX).create().send().map(|_| ());

        Box::new(create_res)
    }

    // Execute an index exists request
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.index(INDEX).exists().send();

        Box::new(res)
    }

    // Ensure the index is reported as existing
    fn assert_ok(&self, res: &Self::Response) -> bool {
        res.exists()
    }
}
