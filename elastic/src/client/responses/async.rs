use std::io::{Read, Result as IoResult};
use futures::Future;
use futures_cpupool::CpuPool;
use tokio_io::{AsyncRead, io as async_io};
use serde::de::DeserializeOwned;
use reqwest::unstable::async::Response as RawResponse;

use error::*;
use elastic_reqwest::AsyncFromResponse;
use elastic_reqwest::res::parse;
use super::parse::IsOk;

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
    pub fn into_response<T>(self) -> Box<Future<Item = T, Error = Error>>
        where T: IsOk + DeserializeOwned + Send + 'static
    {
        let status = self.status();

        let body_future = async_io::read_to_end(self.inner, Vec::new()).map_err(Into::into);

        let de_fn = move |body| parse().from_slice(status, &body).map_err(Into::into);

        if let Some(de_pool) = self.de_pool {
            let de_future = body_future
                .and_then(move |(_, body)| {
                    de_pool.spawn_fn(move || de_fn(body))
                });

            return Box::new(de_future)
        }
        
        let de_future = body_future
            .and_then(move |(_, body)| de_fn(body));

        Box::new(de_future)
    }
}

/** A raw HTTP response that can be buffered using `Read`. */
pub struct AsyncHttpResponse(RawResponse);

impl AsyncHttpResponse {
    /** Get the HTTP status for the response. */
    pub fn status(&self) -> u16 {
        self.0.status().into()
    }
}

impl Read for AsyncHttpResponse {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.0.read(buf)
    }
}

impl AsyncRead for AsyncHttpResponse {

}