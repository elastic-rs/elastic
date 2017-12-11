/*!
Response types for a [get document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html).
*/

use serde::de::DeserializeOwned;
use http::StatusCode;

use parsing::{HttpResponseHead, IsOk, MaybeOkResponse, ResponseBody, Unbuffered};
use error::*;

/** Response for a [get document request](https://www.elastic.co/guide/en/elasticsearch/reference/current/docs-get.html). */
#[derive(Deserialize, Debug)]
pub struct GetResponse<T> {
    #[serde(rename = "_index")] index: String,
    #[serde(rename = "_type")] ty: String,
    #[serde(rename = "_id")] id: String,
    #[serde(rename = "_version")] version: Option<u32>,
    found: bool,
    #[serde(rename = "_source")] source: Option<T>,
    #[serde(rename = "_routing")] routing: Option<String>,
}

impl<T> GetResponse<T> {
    /** Get a reference to the source document. */
    pub fn document(&self) -> Option<&T> {
        self.source.as_ref()
    }

    /** Convert the response into the source document. */
    pub fn into_document(self) -> Option<T> {
        self.source
    }

    /** Whether or not a matching document was found. */
    pub fn found(&self) -> bool {
        self.found
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

impl<T: DeserializeOwned> IsOk for GetResponse<T> {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseError> {
        match head.status() {
            status if status.is_success() => Ok(MaybeOkResponse::ok(body)),
            StatusCode::NOT_FOUND => {
                // If we get a 404, it could be an IndexNotFound error or ok
                // Check if the response contains a root 'error' node
                let (maybe_err, body) = body.body()?;

                let is_ok = maybe_err
                    .as_object()
                    .and_then(|maybe_err| maybe_err.get("error"))
                    .is_none();

                Ok(MaybeOkResponse::new(is_ok, body))
            }
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}
