use reqwest::Response as RawResponse;
use serde::de::DeserializeOwned;

use crate::{
    error::{
        self,
        Error,
    },
    http::{
        receiver::{
            parse,
            IsOk,
        },
        StatusCode,
        SyncHttpResponse,
    },
};

/**
A builder for a response.

This structure wraps the completed HTTP response but gives you options for converting it into a concrete type.
You can also `Read` directly from the response body.
*/
pub struct SyncResponseBuilder(StatusCode, RawResponse);

pub(crate) fn sync_response(res: RawResponse) -> Result<SyncResponseBuilder, Error> {
    let status = StatusCode::from_u16(res.status().into()).map_err(error::request)?;
    Ok(SyncResponseBuilder(status, res))
}

impl SyncResponseBuilder {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> StatusCode {
        self.0
    }

    /**
    Get the response body from JSON.

    Convert the builder into a raw HTTP response that implements `Read`.
    */
    pub fn into_raw(self) -> SyncHttpResponse {
        SyncHttpResponse::from_raw(self.0, self.1)
    }

    /**
    Parse an API response type from the HTTP body.

    This will consume the `SyncResponseBuilder` and return a [concrete response type][response-types] or an error.

    The response is parsed according to the `IsOk` implementation for `T` that will inspect the response and either return an `Ok(T)` or an `Err(ApiError)`.

    # Examples

    Get a strongly typed `SearchResponse`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: String,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateMapping>
    # }
    # let client = SyncClientBuilder::new().build()?;
    let response = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                         .send()?
                         .into_response::<SearchResponse<MyType>>();
    # Ok(())
    # }
    ```

    You can also read a response as a `serde_json::Value`, which will be `Ok(Value)`
    if the HTTP status code is `Ok` or `Err(ApiError)` otherwise:

    ```no_run
    # #[macro_use] extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                         .send()?
                         .into_response::<Value>();
    # Ok(())
    # }
    ```

    [response-types]: parse/trait.IsOk.html#implementors
    */
    pub fn into_response<T>(self) -> Result<T, Error>
    where
        T: IsOk + DeserializeOwned,
    {
        let status = self.0;
        parse()
            .from_reader(status, self.1)
            .map_err(|e| error::response(status, e))
    }
}
