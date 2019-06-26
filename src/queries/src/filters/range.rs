use super::{
    super::Values,
    common::EsDateFormat,
};
use serde::{
    self,
    de::Visitor,
    ser::{
        Serialize,
        SerializeMap,
        Serializer,
    },
};
use std::fmt;

//FIXME: Implement builder pattern for RangeFilter
#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct RangeFilter {
    pub range: RangeField,
}

impl RangeFilter {
    pub fn new(field: &str, params: RangeParams) -> RangeFilter {
        RangeFilter {
            range: RangeField {
                field: field.to_string(),
                params: params,
            },
        }
    }
}

#[derive(Builder, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct RangeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    gte: Option<Values>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    gt: Option<Values>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    lte: Option<Values>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    lt: Option<Values>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    format: Option<EsDateFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    time_zone: Option<Values>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "None")]
    boost: Option<String>,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct RangeField {
    pub field: String,
    pub params: RangeParams,
}

impl Serialize for RangeField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.field, &self.params)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for RangeField {
    fn deserialize<D>(deserializer: D) -> Result<RangeField, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(RangeFilterVisitor)
    }
}

pub(crate) struct RangeFilterVisitor;

impl<'de> Visitor<'de> for RangeFilterVisitor {
    type Value = RangeField;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a range filter structure")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;
        let field: String = map.next_key()?.ok_or(A::Error::custom("expected field"))?;
        let value: RangeParams = map.next_value()?;

        Ok(RangeField {
            field: field,
            params: value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn range_filter_new() {
        let p = RangeParamsBuilder::default()
            .gte(Some(0.into()))
            .lte(Some(1.into()))
            .build()
            .unwrap();

        let r = RangeFilter::new("foo", p);

        assert!(r.range.field == "foo");
        assert!(r.range.params.gte == Some(0.into()));
    }

    #[test]
    fn range_filter() {
        let o = r#"{"range":{"@timestamp":{"gte":1,"lte":2}}}"#;
        let s: RangeFilter = serde_json::from_str(o).unwrap();
        let j = serde_json::to_string(&s).unwrap();
        assert_eq!(o, j);

        let o = r#"{"range":{"@timestamp":{"gte":"now","lte":"now-2h"}}}"#;
        let s: RangeFilter = serde_json::from_str(o).unwrap();
        let j = serde_json::to_string(&s).unwrap();
        assert_eq!(o, j);
    }
}
