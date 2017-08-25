/*!
Utility types for response parsing.

# Examples

Implement `IsOk` for a deserialisable type that converts a http response into a concrete type.
This example defines a search response that, for whatever reason, only includes the `took` field:

```
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# extern crate elastic;
# use elastic::prelude::*;
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
            // Otherwise return the response with `ok: false`
            // The body will be parsed as an `ApiError`.
            _ => Ok(MaybeOkResponse::err(body))
        }
    }
}
# fn main() {}
```

The `MyResponse` type can then be used for deserialising a concrete response:

```no_run
# extern crate serde;
# #[macro_use] extern crate serde_derive;
# extern crate elastic;
# use elastic::prelude::*;
# use elastic::error::Error;
# use elastic::client::responses::parse::*;
# #[derive(Deserialize)]
# struct MyResponse {
#    took: u64
# }
# impl IsOk for MyResponse {
#     fn is_ok<B>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError>
#         where B: ResponseBody
#     {
#         match head.status() {
#             200...299 => Ok(MaybeOkResponse::ok(body)),
#             _ => Ok(MaybeOkResponse::err(body))
#         }
#     }
# }
# fn main() { run().unwrap() }
# fn run() -> Result<(), Box<::std::error::Error>> {
# let client = SyncClientBuilder::new().build()?;
# let req = SearchRequest::new("");
let response = client.request(req)
                     .send()?
                     .into_response::<MyResponse>();

match response {
    Ok(response) => {
        println!("took: {}", response.took);
    },
    Err(e) => {
        match *e.kind() {
            ErrorKind::Api(ref e) => {
                // handle a REST API error
            },
            ref e => {
                // handle a HTTP or JSON error
            }
        }
    }
}
# Ok(())
# }
```

You can also parse the response body into a temporary `serde_json::Value` if the status code isn't enough to determine if it's ok.
This will consume the `UnbufferedResponse` and return a `BufferedResponse` instead that keeps the response body private for later handlers to use.
See the [`IsOk`][IsOk] trait for more details.

[IsOk]: trait.IsOk.html
*/

pub use elastic_reqwest::res::parsing::{HttpResponseHead, IsOk, ResponseBody, MaybeOkResponse,
                                        MaybeBufferedResponse, Unbuffered, Buffered};
pub use elastic_reqwest::res::error::ParseResponseError;
