/**
Tests for ensuring documents can be derived.
*/

use std::collections::HashMap;

#[derive(ElasticType, Serialize, Deserialize)]
pub struct Sample {
    pub labels: HashMap<String, String>,
    pub value: f64,
    pub timestamp: i64,
}
