use futures::Future;
use elastic::prelude::*;
use elastic::error::Error;
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct DoesNotExist;

const INDEX: &'static str = "index_does_not_exist";

impl IntegrationTest for DoesNotExist {
    type Response = IndicesExistsResponse;

    fn kind() -> &'static str {
        "index"
    }
    fn name() -> &'static str {
        "index_does_not_exist"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client.index_delete(index(INDEX)).send().map(|_| ());

        Box::new(delete_res)
    }

    // Execute an index exists request
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.index_exists(index(INDEX)).send();

        Box::new(res)
    }

    // Ensure the index is not reported as existing
    fn assert_ok(&self, res: &Self::Response) -> bool {
        !res.exists()
    }
}
