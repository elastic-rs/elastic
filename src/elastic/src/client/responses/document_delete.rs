/*!
Response types for a [delete document request](https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html).
*/

use crate::{
    http::{
        receiver::{
            HttpResponseHead,
            IsOk,
            MaybeOkResponse,
            ParseError,
            ResponseBody,
            Unbuffered,
        },
        StatusCode,
    },
    types::document::{
        Id,
        Index,
        Type,
    },
};

use super::common::DocumentResult;

/** Response for a [delete document request](https://www.elastic.co/guide/en/elasticsearch/reference/master/docs-delete.html). */
#[derive(Deserialize, Debug)]
pub struct DeleteResponse {
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_type")]
    ty: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_version")]
    version: Option<u32>,
    #[serde(rename = "_routing")]
    routing: Option<String>,
    result: DocumentResult,
}

impl DeleteResponse {
    /** Whether or not the document was deleted. */
    pub fn deleted(&self) -> bool {
        match self.result {
            DocumentResult::Deleted => true,
            _ => false,
        }
    }

    /** The index for the document. */
    pub fn index(&self) -> Index {
        Index::from(&self.index)
    }

    /** The type of the document. */
    pub fn ty(&self) -> Type {
        Type::from(&self.ty)
    }

    /** The id of the document. */
    pub fn id(&self) -> Id {
        Id::from(&self.id)
    }

    /** The version of the document. */
    pub fn version(&self) -> Option<u32> {
        self.version.clone()
    }
}

impl IsOk for DeleteResponse {
    fn is_ok<B: ResponseBody>(
        head: HttpResponseHead,
        body: Unbuffered<B>,
    ) -> Result<MaybeOkResponse<B>, ParseError> {
        match head.status() {
            status if status.is_success() || status == StatusCode::NOT_FOUND => {
                Ok(MaybeOkResponse::ok(body))
            }
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}
