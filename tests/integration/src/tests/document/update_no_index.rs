use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use futures::Future;

#[derive(Debug, PartialEq, Serialize, Deserialize, ElasticType)]
#[elastic(index = "update_no_index_idx")]
pub struct Doc {
    #[elastic(id)]
    id: String,
}

test! {
    const description: &'static str = "update and create index";

    type Response = UpdateResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        let delete_res = client
            .index(Doc::static_index())
            .delete()
            .send()
            .map(|_| ());

        Box::new(delete_res)
    }

    // Execute an update request against that index
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let res = client.document::<Doc>().update("1").doc(json!({})).send();

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
