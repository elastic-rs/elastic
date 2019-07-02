/** A default type for allocated fields in responses. */
pub(crate) type DefaultAllocatedField = String;

/** Returned hits metadata. */
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Shards {
    total: u32,
    successful: u32,
    failed: u32,
}

impl Shards {
    /** The total number of shards that participated in this request. */
    pub fn total(&self) -> u32 {
        self.total
    }

    /** The total number of shards that successfully processed the request. */
    pub fn successful(&self) -> u32 {
        self.successful
    }

    /** The total number of shards that failed to process the request. */
    pub fn failed(&self) -> u32 {
        self.failed
    }
}

#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Eq)]
pub(crate) enum DocumentResult {
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "noop")]
    NoOp,
    #[serde(rename = "created")]
    Created,
}
