#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ExistsFilter {
    pub exists: ExistsField,
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ExistsField {
    pub field: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn exists_filter() {
        let j = r#"{ "exists": { "field": "sourceAddress" } }"#;
        let _s: ExistsFilter = serde_json::from_str(j).unwrap();
    }
}
