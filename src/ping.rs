use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

/// Response for a cluster ping request.
#[derive(Deserialize, Debug)]
pub struct PingResponse {
    pub name: String,
    pub cluster_name: String,
    pub tagline: String,
    pub version: ClusterVersion,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct ClusterVersion {
    pub number: String,
    pub build_hash: String,
    pub build_date: String,
    pub build_snapshot: bool,
    pub lucene_version: String
}

impl IsOk for PingResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}