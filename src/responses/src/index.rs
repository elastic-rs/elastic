/*!
Response types for an [index document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html).
*/

use common::Shards;
use parsing::{HttpResponseHead, IsOk, MaybeOkResponse, ResponseBody, Unbuffered};
use error::*;

/** Response for an [index document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html). */
#[derive(Deserialize, Debug)]
pub struct IndexResponse {
    #[serde(rename = "_index")] index: String,
    #[serde(rename = "_type")] ty: String,
    #[serde(rename = "_id")] id: String,
    #[serde(rename = "_version")] version: Option<u32>,
    created: bool,
    #[serde(rename = "_shards")] shards: Shards,
}

impl IndexResponse {
    /** Shards metadata for the request. */
    pub fn shards(&self) -> &Shards {
        &self.shards
    }

    /** Whether or not a matching document was created. */
    pub fn created(&self) -> bool {
        self.created
    }

    /** The index for the document. */
    pub fn index(&self) -> &str {
        &self.index
    }

    /** The type of the document. */
    pub fn ty(&self) -> &str {
        &self.ty
    }

    /** The id of the document. */
    pub fn id(&self) -> &str {
        &self.id
    }

    /** The version of the document. */
    pub fn version(&self) -> Option<u32> {
        self.version.clone()
    }
}

impl IsOk for IndexResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}
