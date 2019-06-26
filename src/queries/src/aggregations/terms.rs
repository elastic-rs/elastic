use super::{
    super::filters::common::*,
    BucketAggregation,
    EsAggregation,
};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TermAggregation {
    pub terms: TermsAggFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggs: Option<EsAggregation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TermsAggFields {
    pub field: String,
    pub size: u64,
    pub order: Option<FieldAndValue>,
}

impl BucketAggregation for TermAggregation {
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
    fn terms_aggs() {
        let j = r#"{
                      "terms": {
                        "field": "flowInputInterface",
                        "size": 10000,
                        "order": {
                          "_term": "asc"
                        }
                      }
                    }"#;
        let _s: TermAggregation = serde_json::from_str(j).unwrap();
    }
}
