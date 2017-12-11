use super::super::filters::common::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AvgAggregation {
    pub avg: Field,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaxAggregation {
    pub max: Field,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SumAggregation {
    pub sum: Field,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn stats_aggs() {
        let j = r#"{
                          "sum": {
                            "field": "numberOfFlows"
                          }
                        }"#;
        let _s: SumAggregation = serde_json::from_str(j).unwrap();

        let j = r#"{
                            "avg": {
                            "field": "flowPacketsPerSecond"
                           }
                        }"#;
        let _s: AvgAggregation = serde_json::from_str(j).unwrap();
    }
}
