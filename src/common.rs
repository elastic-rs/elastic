#[derive(Deserialize, Debug, Clone)]
pub struct Shards {
    pub total: u32,
    pub successful: u32,
    pub failed: u32,
}