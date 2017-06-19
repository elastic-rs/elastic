//! Response types for a cluster ping request.

use parsing::{IsOk, HttpResponseHead, ResponseBody, Unbuffered, MaybeOkResponse};
use error::*;

/// Response for a cluster ping request.
#[derive(Deserialize, Debug)]
pub struct PingResponse {
    name: String,
    cluster_name: String,
    tagline: String,
    version: ClusterVersion,
}

#[doc(hidden)]
#[derive(Deserialize, Debug)]
pub struct ClusterVersion {
    number: String,
    build_hash: String,
    build_date: String,
    build_snapshot: bool,
    lucene_version: String
}

impl PingResponse {
    /// The name of the pinged node.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The name of the cluster the pinged node belongs to.
    pub fn cluster_name(&self) -> &str {
        &self.cluster_name
    }

    /// The Elasticsearch version metadata.
    pub fn version(&self) -> &ClusterVersion {
        &self.version
    }
}

impl ClusterVersion {
    /// The builder number.
    pub fn number(&self) -> &str {
        &self.number
    }

    /// The build hash.
    pub fn hash(&self) -> &str {
        &self.build_hash
    }

    // The build date.
    pub fn date(&self) -> &str {
        &self.build_date
    }

    /// Whether or not the build is a snapshot.
    pub fn snapshot(&self) -> bool {
        self.build_snapshot
    }

    /// The underlying Lucene version.
    pub fn lucene_version(&self) -> &str {
        &self.lucene_version
    }
}

impl IsOk for PingResponse {
    fn is_ok<B: ResponseBody>(head: HttpResponseHead, body: Unbuffered<B>) -> Result<MaybeOkResponse<B>, ParseResponseError> {
        match head.status() {
            200...299 => Ok(MaybeOkResponse::ok(body)),
            _ => Ok(MaybeOkResponse::err(body)),
        }
    }
}