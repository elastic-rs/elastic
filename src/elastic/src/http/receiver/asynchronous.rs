use std::sync::Arc;

use futures::{
    future::lazy,
    Future,
    Poll,
    Stream,
};
use reqwest::r#async::Response as RawResponse;
use serde::de::DeserializeOwned;
use tokio_threadpool::ThreadPool;

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
        AsyncChunk,
        AsyncHttpResponse,
        StatusCode,
    },
};

/**
A builder for a response.

This structure wraps the completed HTTP response but gives you options for converting it into a concrete type.
You can also `Read` directly from the response body.
*/
pub struct AsyncResponseBuilder {
    inner: RawResponse,
    status: StatusCode,
    de_pool: Option<Arc<ThreadPool>>,
}

pub(crate) fn async_response(
    res: RawResponse,
    de_pool: Option<Arc<ThreadPool>>,
) -> Result<AsyncResponseBuilder, Error> {
    let status = StatusCode::from_u16(res.status().into()).map_err(error::request)?;
    Ok(AsyncResponseBuilder {
        inner: res,
        status,
        de_pool,
    })
}

impl AsyncResponseBuilder {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /**
    Get the response body from JSON.

    Convert the builder into a raw HTTP response that implements `Read`.
    */
    pub fn into_raw(self) -> AsyncHttpResponse {
        AsyncHttpResponse::from_raw(self.status, self.inner)
    }

    /**
    Parse an API response type from the HTTP body.

    The deserialisation may occur on a background thread.
    This will consume the `AsyncResponseBuilder` and return a [concrete response type][response-types] or an error.

    The response is parsed according to the `IsOk` implementation for `T` that will inspect the response and either return an `Ok(T)` or an `Err(ApiError)`.

    # Examples

    Get a strongly typed `SearchResponse`:

    ```no_run
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # use futures::Future;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                       .send()
                       .and_then(|response| response.into_response::<SearchResponse<MyType>>());

    future.and_then(|response| {
        // Iterate through the hits (of type `MyType`)
        for hit in response.hits() {
            println!("{:?}", hit);
        }

        Ok(())
    });
    # Ok(())
    # }
    ```

    You can also read a response as a `serde_json::Value`, which will be `Ok(Value)`
    if the HTTP status code is `Ok` or `Err(ApiError)` otherwise:

    ```no_run
    # use futures::Future;
    # use serde_json::Value;
    # use elastic::prelude::*;
        # fn main() -> Result<(), Box<::std::error::Error>> {
    # let client = AsyncClientBuilder::new().build()?;
    let future = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                       .send()
                       .and_then(|response| response.into_response::<Value>());
    # Ok(())
    # }
    ```

    [response-types]: parse/trait.IsOk.html#implementors
    */
    pub fn into_response<T>(self) -> IntoResponse<T>
    where
        T: IsOk + DeserializeOwned + Send + 'static,
    {
        let status = self.status;
        let body = self.inner.into_body();

        let de_fn = move |body: AsyncChunk| {
            parse()
                .from_slice(status, body.as_ref())
                .map_err(move |e| error::response(status, e))
        };

        let body_future = body.concat2().map_err(move |e| error::response(status, e));

        if let Some(de_pool) = self.de_pool {
            IntoResponse::new(
                body_future.and_then(move |body| de_pool.spawn_handle(lazy(move || de_fn(body)))),
            )
        } else {
            IntoResponse::new(body_future.and_then(de_fn))
        }
    }
}

/** A future returned by calling `into_response`. */
pub struct IntoResponse<T> {
    inner: Box<dyn Future<Item = T, Error = Error> + Send>,
}

impl<T> IntoResponse<T> {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = T, Error = Error> + Send + 'static,
    {
        IntoResponse {
            inner: Box::new(fut),
        }
    }
}

impl<T> Future for IntoResponse<T>
where
    T: IsOk + DeserializeOwned + Send + 'static,
{
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client,
        tests::*,
    };

    #[test]
    fn is_send() {
        assert_send::<super::IntoResponse<client::responses::PingResponse>>();
    }
}
