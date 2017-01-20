use serde::Deserialize;
use serde_json::Value;

use parse::MaybeOkResponse;
use super::{HttpResponse, FromResponse, ApiResult};

use std::io::Read;

/// Response for a get document request.
#[derive(Deserialize, Debug)]
pub struct GetResponseOf<T: Deserialize> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub ty: String,
    #[serde(rename = "_version")]
    pub version: Option<u32>,
    pub found: bool,
    #[serde(rename = "_source")]
    pub source: Option<T>,
    #[serde(rename="_routing")]
    pub routing: Option<String>,
}

pub type GetResponse = GetResponseOf<Value>;

impl<T: Deserialize> FromResponse for GetResponseOf<T> {
    fn from_response<I: Into<HttpResponse<R>>, R: Read>(res: I) -> ApiResult<Self> {
        let res = res.into();

        res.response(|res| {
            match res.status() {
                200...299 => Ok(MaybeOkResponse::new(true, res)),
                404 => {
                    // If we get a 404, it could be an IndexNotFound error or ok
                    // Check if the response contains a root 'error' node
                    let (body, res) = res.body()?;

                    let is_ok = body.as_object()
                        .and_then(|body| body.get("error"))
                        .is_none();

                    Ok(MaybeOkResponse::new(is_ok, res))
                }
                _ => Ok(MaybeOkResponse::new(false, res)),
            }
        })
    }
}
