use elastic::{
    error::{
        ApiError,
        Error,
    },
    prelude::*,
};
use futures::{
    future,
    Future,
};

test! {
    const description: &'static str = "invalid syntax";

    type Response = SqlResponse;

    // Ensure the index doesn't exist
    fn prepare(&self, _client: AsyncClient) -> Box<dyn Future<Item = (), Error = Error>> {
        Box::new(future::ok(()))
    }

    // Execute a search request against that index
    fn request(
        &self,
        client: AsyncClient,
    ) -> Box<dyn Future<Item = Self::Response, Error = Error>> {
        let res = client.sql_query("select").send();

        Box::new(res)
    }

    // Ensure a `Parsing` error is returned
    fn assert_err(&self, err: &Error) -> bool {
        match *err {
            Error::Api(ApiError::Parsing { .. }) => true,
            _ => false,
        }
    }
}
