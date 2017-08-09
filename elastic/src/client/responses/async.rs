use futures::{Future, Stream, Poll};
use futures_cpupool::CpuPool;
use serde::de::DeserializeOwned;
use reqwest::unstable::async::Response as RawResponse;

use error::{self, Error};
use elastic_reqwest::res::parse;
use super::parse::IsOk;

pub use reqwest::unstable::async::Chunk;

/**
A builder for a response.

This structure wraps the completed HTTP response but gives you options for converting it into a concrete type.
You can also `Read` directly from the response body.
*/
pub struct AsyncResponseBuilder {
    inner: RawResponse,
    de_pool: Option<CpuPool>
}

pub(crate) fn async_response(res: RawResponse, de_pool: Option<CpuPool>) -> AsyncResponseBuilder {
    AsyncResponseBuilder {
        inner: res,
        de_pool: de_pool
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
    
    This will consume the `AsyncResponseBuilder` and return a [concrete response type][response-types] or an error.
    
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
    # fn main() {
    # #[derive(Serialize, Deserialize, ElasticType)]
    # struct MyType {
    #     pub id: i32,
    #     pub title: String,
    #     pub timestamp: Date<DefaultDateFormat>
    # }
    # let params = RequestParams::new("http://es_host:9200");
    # let client = Client::new(params).unwrap();
    # let req = PingRequest::new();
    let response = client.request(req)
                         .send()
                         .and_then(into_response::<SearchResponse<MyType>>);
    # }
    ```
    
    You can also read a response as a `serde_json::Value`, which will be `Ok(Value)`
    if the HTTP status code is `Ok` or `Err(ApiError)` otherwise:
    
    ```no_run
    # extern crate elastic;
    # extern crate serde_json;
    # use serde_json::Value;
    # use elastic::prelude::*;
    # fn main() {
    # let params = RequestParams::default();
    # let client = Client::new(params).unwrap();
    # let req = PingRequest::new();
    let response = client.request(req)
                         .send()
                         .and_then(into_response::<Value>);
    # }
    ```

    [response-types]: parse/trait.IsOk.html#implementors
    */
    pub fn into_response<T>(mut self) -> Box<Future<Item = T, Error = Error>>
        where T: IsOk + DeserializeOwned + Send + 'static
    {
        let status = self.status();
        let body = self.inner.body();
        let de_fn = move |body: Chunk| parse().from_slice(status, body.as_ref()).map_err(move |e| error::response(status, e));

        let body_future = body.concat2().map_err(move |e| error::response(status, e));

        if let Some(de_pool) = self.de_pool {
            Box::new(body_future.and_then(move |body| de_pool.spawn_fn(move || de_fn(body))))
        }
        else {
            Box::new(body_future.and_then(move |body| de_fn(body)))
        }
    }
}

/** A raw HTTP response that can be buffered using `Read`. */
pub struct AsyncHttpResponse(RawResponse);

impl Stream for AsyncHttpResponse {
    type Item = Chunk;
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
