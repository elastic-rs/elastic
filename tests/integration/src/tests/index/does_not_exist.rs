use elastic::{
    error::Error,
    prelude::*,
};
use futures::Future;

const INDEX: &'static str = "index_does_not_exist";

test! {
    const description: &'static str = "get index that doesn't exist";

    type Response = IndicesExistsResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client.index(INDEX).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Execute an index exists request
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let res = client.index(INDEX).exists().send();

        Box::new(res)
    }

    // Ensure the index is not reported as existing
    fn assert_ok(&self, res: &Self::Response) -> bool {
        !res.exists()
    }
}
