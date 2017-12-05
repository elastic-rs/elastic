use serde::de::DeserializeOwned;
use reqwest::Response as RawResponse;

use error::{self, Result};
use http::{StatusCode, SyncHttpResponse};
use super::parse::{parse, IsOk};

/**
A builder for a response.

This structure wraps the completed HTTP response but gives you options for converting it into a concrete type.
You can also `Read` directly from the response body.
*/
pub struct SyncResponseBuilder(StatusCode, RawResponse);

pub(crate) fn sync_response(res: RawResponse) -> Result<SyncResponseBuilder> {
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
    # extern crate serde;
    # #[macro_use]
    # extern crate serde_derive;
    # #[macro_use]
    # extern crate elastic_derive;
    # extern crate elastic;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
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
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let client = SyncClientBuilder::new().build()?;
    let response = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                         .send()?
                         .into_response::<Value>();
    # Ok(())
    # }
    ```

    [response-types]: parse/trait.IsOk.html#implementors
    */
    pub fn into_response<T>(self) -> Result<T>
    where
        T: IsOk + DeserializeOwned,
    {
        let status = self.0;
        parse()
            .from_reader(status, self.1)
            .map_err(|e| error::response(status, e))
    }
}
