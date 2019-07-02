use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use futures::Future;
use serde_json::Value;

const INDEX: &'static str = "no_index_idx";

test! {
    const description: &'static str = "no index";

    type Response = SearchResponse<Value>;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client.index(INDEX).delete().send().map(|_| ());

        Box::new(delete_res)
    }

    // Execute a search request against that index
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
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
