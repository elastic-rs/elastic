/*!
Utility types for response parsing.

# Examples

Implement `FromResponse` for a deserialisable type that converts
a http response into a concrete type.
This example defines a search response that, for whatever reason,
only includes the `took` field:

```
# extern crate serde;
# #[macro_use]
# extern crate serde_derive;
# extern crate elastic;
# use std::io::Read;
# use elastic::prelude::*;
# use elastic::error::ParseResponseError;
# use elastic::client::responses::parse::*;
#[derive(Deserialize)]
struct MyResponse {
    took: u64
}

impl IsOk for MyResponse {
    fn is_ok<B>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError>
        where B: ResponseBody
    {
        match head.status() {
            // If the status is 2xx then return the response with `ok: true`
            // The body will be parsed as a `MyResponse`.
            200...299 => Ok(MaybeOkResponse::ok(body)),
            // If the status is 404 it might be ok, or it might be an error
            404 => {
                let (maybe_err, body) = body.body()?;

                // See if the body contains an `error` field
                // If it does, then it's an error
                let is_ok = maybe_err.as_object()
                    .and_then(|maybe_err| maybe_err.get("error"))
                    .is_none();

                Ok(MaybeOkResponse::new(is_ok, body))
            }
            // Otherwise return the response with `ok: false`
            // The body will be parsed as an `ApiError`.
            _ => Ok(MaybeOkResponse::err(body))
        }
    }
}
# fn main() {}
```

You can also parse the response body into a temporary `serde_json::Value`
if the status code isn't enough to determine if it's ok.
This will consume the `UnbufferedResponse` and return a `BufferedResponse`
instead that keeps the response body private for later handlers to use.
!*/

pub use elastic_reqwest::res::parsing::{HttpResponseHead, IsOk, ResponseBody, MaybeOkResponse, MaybeBufferedResponse,
                                   Unbuffered, Buffered};
