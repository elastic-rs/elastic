use futures::Future;
use elastic::prelude::*;
use elastic::error::{Error, ApiError};
use run_tests::IntegrationTest;

#[derive(Debug, Clone, Copy)]
pub struct UpdateNoIndex;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
pub struct Doc {
    id: i32,
}

const INDEX: &'static str = "update_no_index_idx";

impl IntegrationTest for UpdateNoIndex {
    type Response = UpdateResponse;

    fn kind() -> &'static str {
        "document"
    }
    fn name() -> &'static str {
        "update_no_index"
    }

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<Future<Item = (), Error = Error>> {
        let delete_res = client
            .index_delete(index(INDEX))
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Execute an update request against that index
    fn request(&self, client: AsyncClient) -> Box<Future<Item = Self::Response, Error = Error>> {
        let res = client.document_update::<Doc>(index(INDEX), id(1))
            .doc(Doc { id: 1 })
            .send();

        Box::new(res)
    }

    // Ensure an `DocumentMissing` error is returned
    fn assert_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::DocumentMissing { .. }) => true,
            _ => false,
        }
    }
}
