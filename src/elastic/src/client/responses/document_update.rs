/*!
Response types for a [update document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update.html).
*/

use super::common::DocumentResult;

use crate::{
    http::receiver::IsOkOnSuccess,
    types::document::{
        Id,
        Index,
        Type,
    },
};

/** Response for a [update document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-update.html). */
#[derive(Deserialize, Debug)]
pub struct UpdateResponse {
    #[serde(rename = "_index")]
    index: String,
    #[serde(rename = "_type")]
    ty: String,
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_version")]
    version: Option<u32>,
    #[serde(rename = "_seq_no")]
    sequence_number: Option<u32>,
    #[serde(rename = "_primary_term")]
    primary_term: Option<u32>,
    #[serde(rename = "_routing")]
    routing: Option<String>,
    result: DocumentResult,
}

impl UpdateResponse {
    /** Whether or not the document was updated. */
    pub fn updated(&self) -> bool {
        match self.result {
            DocumentResult::Updated => true,
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
        self.version
    }

    /**
     * The [sequence number] of the document.
     *
     * [sequence number]: https://www.elastic.co/guide/en/elasticsearch/reference/current/optimistic-concurrency-control.html
     */
    pub fn sequence_number(&self) -> Option<u32> {
        self.sequence_number
    }

    /**
     * The [primary term] of the document.
     *
     * [primary term]: https://www.elastic.co/guide/en/elasticsearch/reference/current/optimistic-concurrency-control.html
     */
    pub fn primary_term(&self) -> Option<u32> {
        self.primary_term
    }
}

impl IsOkOnSuccess for UpdateResponse {}
