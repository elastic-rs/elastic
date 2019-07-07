/*!
Response types for an [index document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html).
*/

use super::common::{
    DocumentResult,
    Shards,
};

use crate::{
    http::receiver::IsOkOnSuccess,
    types::document::{
        Id,
        Index,
        Type,
    },
};

/** Response for an [index document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-index_.html). */
#[derive(Deserialize, Debug)]
pub struct IndexResponse {
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_type")]
    ty: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_version")]
    version: Option<u32>,
    result: DocumentResult,
    #[serde(rename = "_shards")]
    shards: Shards,
}

impl IndexResponse {
    /** Shards metadata for the request. */
    pub fn shards(&self) -> &Shards {
        &self.shards
    }

    /** Whether or not a matching document was created. */
    pub fn created(&self) -> bool {
        match self.result {
            DocumentResult::Created => true,
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

impl IsOkOnSuccess for IndexResponse {}
