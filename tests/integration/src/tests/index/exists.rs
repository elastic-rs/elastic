use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

const INDEX: &'static str = "index_exists";

test! {
    const description: &'static str = "get index that exists";

    type Response = IndicesExistsResponse;

    // Ensure the index exists
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let create_res = client.index(INDEX).create().send().map(|_| ());

        Box::new(create_res)
    }

    // Execute an index exists request
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let res = client.index(INDEX).exists().send();

        Box::new(res)
    }

    // Ensure the index is reported as existing
    fn assert_ok(&self, res: &Self::Response) -> bool {
        res.exists()
    }
}
