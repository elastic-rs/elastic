use super::common::*;

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct MatchFilter {
    #[serde(rename = "match")]
    pub match_: FieldAndValue,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn match_filter() {
        let j = r#"{ "match": { "title":   "Search"        }}"#;
        let _s: MatchFilter = serde_json::from_str(j).unwrap();

        let j = r#"{ "match": { "content": "Elasticsearch" }}"#;
        let _s: MatchFilter = serde_json::from_str(j).unwrap();
    }
}
