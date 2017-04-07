use parse::MaybeOkResponse;
use super::{HttpResponse, FromResponse, ApiResult};

use std::io::Read;

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

impl FromResponse for PingResponse {
    fn from_response<I: Into<HttpResponse<R>>, R: Read>(res: I) -> ApiResult<Self> {
        let res = res.into();

        res.response(|res| {
            match res.status() {
                200...299 => Ok(MaybeOkResponse::new(true, res)),
                _ => Ok(MaybeOkResponse::err(res)),
            }
        })
    }
}
