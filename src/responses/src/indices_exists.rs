/*!
Response types for an [index exists request](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html).
*/

use parsing::{HttpResponseHead, IsOk, MaybeOkResponse, ResponseBody, Unbuffered};
use http::StatusCode;
use error::*;

/** Response for an [index exists request](https://www.elastic.co/guide/en/elasticsearch/reference/master/indices-exists.html). */
#[derive(Deserialize, Debug)]
pub struct IndicesExistsResponse {
    exists: bool
}

impl IndicesExistsResponse {
    /** Whether or not the index exists. */
    pub fn exists(&self) -> bool {
        self.exists
    }
}

impl IsOk for IndicesExistsResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseError> {
        match head.status() {
            status if status.is_success() => Ok(MaybeOkResponse::ok(json!({ "exists": true }))),
            StatusCode::NOT_FOUND => Ok(MaybeOkResponse::ok(json!({ "exists": false }))),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}
