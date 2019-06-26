/*! Mapping for the Elasticsearch `boolean` type. */

/** A field that will be mapped as a `boolean`. */
pub trait BooleanFieldType<TMapping> {}

/**
The base requirements for mapping a `boolean` type.

Custom mappings can be defined by implementing `BooleanMapping`.

# Examples

Define a custom `BooleanMapping`:

```
# #[macro_use] use elastic::types::prelude::*;
# fn main() {
#[derive(Default)]
struct MyBooleanMapping;
impl BooleanMapping for MyBooleanMapping {
    //Overload the mapping functions here
    fn boost() -> Option<f32> {
        Some(1.5)
    }
}
# }
```

This will produce the following mapping:

```
# #[macro_use] extern crate serde_json;
# use elastic::types::prelude::*;
# #[derive(Default)]
# struct MyBooleanMapping;
# impl BooleanMapping for MyBooleanMapping {
#     //Overload the mapping functions here
#     fn boost() -> Option<f32> {
#         Some(1.5)
#     }
# }
# fn main() {
# let json = json!(
{
    "type": "boolean",
    "boost": 1.5
}
# );
# let mapping = elastic::types::__derive::standalone_field_ser(MyBooleanMapping).unwrap();
# assert_eq!(json, mapping);
# }
```
*/
pub trait BooleanMapping {
    /** Field-level index time boosting. Accepts a floating point number, defaults to `1.0`. */
    fn boost() -> Option<f32> {
        None
    }

    /**
    Should the field be stored on disk in a column-stride fashion,
    so that it can later be used for sorting, aggregations, or scripting?
    Accepts `true` (default) or `false`.
    */
    fn doc_values() -> Option<bool> {
        None
    }

    /** Should the field be searchable? Accepts `not_analyzed` (default) and `no`. */
    fn index() -> Option<bool> {
        None
    }

    /**
    Accepts a string value which is substituted for any explicit null values.
    Defaults to `null`, which means the field is treated as missing.
    */
    fn null_value() -> Option<bool> {
        None
    }

    /**
    Whether the field value should be stored and retrievable separately from the `_source` field.
    Accepts `true` or `false` (default).
    */
    fn store() -> Option<bool> {
        None
    }
}

/** Default mapping for `bool`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultBooleanMapping;
impl BooleanMapping for DefaultBooleanMapping {}

mod private {
    use super::{
        BooleanFieldType,
        BooleanMapping,
    };
    use crate::types::private::field::{
        FieldMapping,
        FieldType,
        SerializeFieldMapping,
        StaticSerialize,
    };
    use serde::{
        ser::SerializeStruct,
        Serialize,
        Serializer,
    };

    #[derive(Default)]
    pub struct BooleanPivot;

    impl<TField, TMapping> FieldType<TMapping, BooleanPivot> for TField
    where
        TMapping: BooleanMapping,
        TField: BooleanFieldType<TMapping> + Serialize,
    {
    }

    impl<TMapping> FieldMapping<BooleanPivot> for TMapping
    where
        TMapping: BooleanMapping,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, BooleanPivot>;

        fn data_type() -> &'static str {
            "boolean"
        }
    }

    impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, BooleanPivot>
    where
        TMapping: FieldMapping<BooleanPivot> + BooleanMapping,
    {
        fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("mapping", 6)?;

            state.serialize_field("type", TMapping::data_type())?;

            ser_field!(state, "boost", TMapping::boost());
            ser_field!(state, "doc_values", TMapping::doc_values());
            ser_field!(state, "index", TMapping::index());
            ser_field!(state, "store", TMapping::store());
            ser_field!(state, "null_value", TMapping::null_value());

            state.end()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::{
        prelude::*,
        private::field,
    };

    #[derive(Default, Clone)]
    pub struct MyBooleanMapping;
    impl BooleanMapping for MyBooleanMapping {
        fn index() -> Option<bool> {
            Some(false)
        }

        fn doc_values() -> Option<bool> {
            Some(true)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn null_value() -> Option<bool> {
            Some(false)
        }
    }

    #[test]
    fn serialise_mapping_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultBooleanMapping)).unwrap();

        let expected = json!({
            "type": "boolean"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_custom() {
        let ser = serde_json::to_value(&field::serialize(MyBooleanMapping)).unwrap();

        let expected = json!({
            "type": "boolean",
            "doc_values": true,
            "index": false,
            "store": true,
            "null_value": false
        });

        assert_eq!(expected, ser);
    }

}
