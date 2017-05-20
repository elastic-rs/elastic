use common::Shards;
use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

/// Response for a document index request.
#[derive(Deserialize, Debug)]
pub struct IndexResponse {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    pub created: bool,
    #[serde(rename = "_shards")]
    pub shards: Shards,
}

impl IsOk for IndexResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}