use serde_json::Value;
use futures::Future;
use elastic::prelude::*;
use elastic::error::{Error, ApiError};
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct NoIndex;

impl IntegrationTest for NoIndex {
    type Response = SearchResponse<Value>;

    fn kind() -> &'static str { "search" }
    fn name() -> &'static str { "no_index" }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let req = client.request(IndicesDeleteRequest::for_index("no_index_idx"))
                        .send()
                        .map(|_| ());
        
        Box::new(req)
    }

    // Execute a search request against that index
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        client.search()
              .index("no_index_idx")
              .ty(Some("no_index_ty"))
              .send()
    }

    // Ensure an `IndexNotFound` error is returned
    fn assert_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::IndexNotFound { .. }) => true,
            _ => false
        }
    }
}
