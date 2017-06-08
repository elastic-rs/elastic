/// A default type for allocated fields in responses.
pub(crate) type DefaultAllocatedField = String;

/// Returned hits metadata.
#[derive(Deserialize, Debug, Clone)]
pub struct Shards {
    pub total: u32,
    pub successful: u32,
    pub failed: u32,
}
