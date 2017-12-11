use super::common::*;

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct WildcardFilter {
    pub wildcard: WildcardVariants,
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(untagged)]
pub enum WildcardVariants {
    FieldAndValue(FieldAndValue),
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct WildcardField {
    pub field: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn wildcard_filter() {
        let j = r#"{ "wildcard" : { "user" : "ki*y" } }"#;
        let _s: WildcardFilter = serde_json::from_str(j).unwrap();
    }
}
