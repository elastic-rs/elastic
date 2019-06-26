use super::{
    BucketAggregation,
    EsAggregation,
};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DateHistogramAggregation {
    pub date_histogram: DateHistogramFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggs: Option<EsAggregation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DateHistogramFields {
    pub field: String,
    pub interval: String,
    pub min_doc_count: u64,
}

impl BucketAggregation for DateHistogramAggregation {
    fn aggs_mut(&mut self) -> Option<&mut EsAggregation> {
        self.aggs.as_mut()
    }

    fn aggs(&self) -> Option<&EsAggregation> {
        self.aggs.as_ref()
    }

    fn aggs_clear(&mut self) {
        self.aggs = None;
    }

    fn aggs_init(&mut self) {
        self.aggs = Some(HashMap::new());
    }

    fn set_aggs(&mut self, replacement: Option<EsAggregation>) {
        self.aggs = replacement;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn date_histo_aggs() {
        let j = r#"{
          "date_histogram": {
            "field": "@timestamp",
            "interval": "${params.interval}ms",
            "min_doc_count": 0
          },
          "aggs": {}
        }"#;
        let _s: DateHistogramAggregation = serde_json::from_str(j).unwrap();

        let j = r#"{
          "date_histogram": {
            "field": "@timestamp",
            "interval": "${params.interval}ms",
            "min_doc_count": 0
          }
        }"#;
        let _s: DateHistogramAggregation = serde_json::from_str(j).unwrap();
    }
}
