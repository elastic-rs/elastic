use super::super::Values;
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
pub enum EsDateFormat {
    epoch_millis,    //epoch in ms
    epoch_second,    //epoch in s
    basic_date,      //yyyyMMdd
    basic_date_time, //yyyyMMdd'T'HHmmss.SSSZ
    #[doc(hidden)]
    __Nonexhaustive,
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Field {
    pub field: Values,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct FieldAndValue {
    pub field: String,
    pub value: Values,
}

impl<'de> serde::Deserialize<'de> for FieldAndValue {
    fn deserialize<D>(deserializer: D) -> Result<FieldAndValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(FieldAndValueVisitor)
    }
}

impl Serialize for FieldAndValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.field, &self.value)?;
        map.end()
    }
}

pub(crate) struct FieldAndValueVisitor;

impl<'de> Visitor<'de> for FieldAndValueVisitor {
    type Value = FieldAndValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a key value pair")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        use serde::de::Error;
        let field = map.next_key()?.ok_or(A::Error::custom("expected field"))?;
        let value: Values = map.next_value()?;

        Ok(FieldAndValue { field, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn kv_parse_and_serialise() {
        let o = r#"{"title":"Search"}"#;
        let s: FieldAndValue = serde_json::from_str(o).unwrap();
        let j = serde_json::to_string(&s).unwrap();
        assert_eq!(o, j);

        let o = r#"{"content":"Elasticsearch"}"#;
        let s: FieldAndValue = serde_json::from_str(o).unwrap();
        let j = serde_json::to_string(&s).unwrap();
        assert_eq!(o, j);
    }
}
