use std::mem;
use futures::{Future, Poll, Stream};
use futures_cpupool::CpuPool;
use serde::de::DeserializeOwned;
use reqwest::async::{Decoder, Response as RawResponse, Chunk as AsyncChunk};

use error::{self, Error};
use super::parse::{parse, IsOk};

/**
A builder for a response.

This structure wraps the completed HTTP response but gives you options for converting it into a concrete type.
You can also `Read` directly from the response body.
*/
pub struct AsyncResponseBuilder {
    inner: RawResponse,
    de_pool: Option<CpuPool>,
}

pub(crate) fn async_response(res: RawResponse, de_pool: Option<CpuPool>) -> AsyncResponseBuilder {
    AsyncResponseBuilder {
        inner: res,
        de_pool: de_pool,
    }
}

impl AsyncResponseBuilder {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> u16 {
        self.inner.status().into()
    }

    /**
    Get the response body from JSON.
    
    Convert the builder into a raw HTTP response that implements `Read`.
    */
    pub fn into_raw(self) -> AsyncHttpResponse {
        AsyncHttpResponse(self.inner)
    }

    /**
    Parse an API response type from the HTTP body.
    
    The deserialisation may occur on a background thread.
    This will consume the `AsyncResponseBuilder` and return a [concrete response type][response-types] or an error.
    
    The response is parsed according to the `IsOk` implementation for `T` that will inspect the response and either return an `Ok(T)` or an `Err(ApiError)`.
    
    # Examples
    
    Get a strongly typed `SearchResponse`:
    
    ```no_run
    # extern crate tokio;
    # extern crate futures;
    # extern crate serde;
    # #[macro_use] extern crate serde_derive;
    # #[macro_use] extern crate elastic_derive;
    # extern crate elastic;
    # use futures::Future;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # #[derive(Debug, Serialize, Deserialize, ElasticType)]
    # struct MyType { }
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
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
    # extern crate tokio;
    # extern crate futures;
    # extern crate serde_json;
    # extern crate elastic;
    # use futures::Future;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() { run().unwrap() }
    # fn run() -> Result<(), Box<::std::error::Error>> {
    # let core = tokio_core::reactor::Core::new()?;
    # let client = AsyncClientBuilder::new().build(&core.handle())?;
    let future = client.request(SimpleSearchRequest::for_index_ty("myindex", "mytype"))
                       .send()
                       .and_then(|response| response.into_response::<Value>());
    # Ok(())
    # }
    ```

    [response-types]: parse/trait.IsOk.html#implementors
    */
    pub fn into_response<T>(mut self) -> IntoResponse<T>
    where
        T: IsOk + DeserializeOwned + Send + 'static,
    {
        let status = self.status();
        let body = mem::replace(self.inner.body_mut(), Decoder::empty());

        let de_fn = move |body: AsyncChunk| {
            parse()
                .from_slice(status, body.as_ref())
                .map_err(move |e| error::response(status, e))
        };

        let body_future = body.concat2().map_err(move |e| error::response(status, e));

        if let Some(de_pool) = self.de_pool {
            IntoResponse::new(body_future.and_then(move |body| de_pool.spawn_fn(move || de_fn(body))))
        } else {
            IntoResponse::new(body_future.and_then(de_fn))
        }
    }
}

/** A future returned by calling `into_response`. */
pub struct IntoResponse<T> {
    inner: Box<Future<Item = T, Error = Error>>,
}

impl<T> IntoResponse<T> {
    fn new<F>(fut: F) -> Self
    where
        F: Future<Item = T, Error = Error> + 'static,
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

/** A raw HTTP response that can be buffered using `Read`. */
pub struct AsyncHttpResponse(RawResponse);

impl Stream for AsyncHttpResponse {
    type Item = AsyncChunk;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.body_mut().poll().map_err(|e| {
            let status = self.status();

            error::response(status, e)
        })
    }
}

impl AsyncHttpResponse {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> u16 {
        self.0.status().into()
    }
}
