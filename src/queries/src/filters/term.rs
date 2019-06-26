use super::super::{
    filters::common::*,
    Values,
};

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct TermFilter {
    pub term: FieldAndValue,
}

impl TermFilter {
    pub fn new(f: String, v: Values) -> TermFilter {
        TermFilter {
            term: FieldAndValue { field: f, value: v },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn term_filter() {
        let j = r#"{ "term" : { "user" : "Kimchy" }}"#;
        let _s: TermFilter = serde_json::from_str(j).unwrap();

        let j = r#"{ "term" : { "user" : 1 } }"#;
        let _s: TermFilter = serde_json::from_str(j).unwrap();

        let j = r#"{ "term" : { "user" : true } }"#;
        let _s: TermFilter = serde_json::from_str(j).unwrap();

        let j = r#"{ "term":  { "status": "published" }}"#;
        let _s: TermFilter = serde_json::from_str(j).unwrap();
    }
}
