use serde::de::DeserializeOwned;
use serde_json::Value;

use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

/// Response for a [get document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html).
#[derive(Deserialize, Debug)]
pub struct GetResponse<T = Value> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    pub found: bool,
    #[serde(rename = "_source")]
    pub source: Option<T>,
    #[serde(rename="_routing")]
    pub routing: Option<String>,
}

impl<T: DeserializeOwned> IsOk for GetResponse<T> {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            404 => {
                // If we get a 404, it could be an IndexNotFound error or ok
                // Check if the response contains a root 'error' node
                let (maybe_err, body) = body.body()?;

                let is_ok = maybe_err.as_object()
                    .and_then(|maybe_err| maybe_err.get("error"))
                    .is_none();

                Ok(MaybeOkResponse::new(is_ok, body))
            }
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}
