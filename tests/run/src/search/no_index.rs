use serde_json::Value;
use futures::Future;
use elastic::prelude::*;
use elastic::error::{Error, ApiError};
use ::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct NoIndex;

impl IntegrationTest for NoIndex {
    type Response = SearchResponse<Value>;

    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let req = client.request(IndicesDeleteRequest::for_index("no_index_idx"))
                        .send()
                        .map(|_| ());
        
        Box::new(req)
    }

    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        client.search()
              .index("no_index_idx")
              .ty(Some("no_index_ty"))
              .send()
    }

    fn assert_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::IndexNotFound { .. }) => true,
            _ => false
        }
    }
}
