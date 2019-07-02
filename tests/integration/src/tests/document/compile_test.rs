/**
Tests for ensuring documents can be derived.
*/
use std::collections::HashMap;

#[derive(ElasticType, Serialize, Deserialize)]
#[elastic(index = "sample_index")]
pub struct Sample {
    #[elastic(id)]
    pub id: String,
    pub labels: HashMap<String, String>,
    pub value: f64,
    pub timestamp: i64,
}
