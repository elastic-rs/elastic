/*! Mapping for the Elasticsearch `ip` type. */

use std::net::Ipv4Addr;

/** A field that will be mapped as an `ip`. */
pub trait IpFieldType<TMapping> {}

/**
The base requirements for mapping a `ip` type.

Custom mappings can be defined by implementing `IpMapping`.

# Examples

Define a custom `IpMapping`:

```
# #[macro_use] use elastic::types::prelude::*;
#[derive(Default)]
struct MyIpMapping;
impl IpMapping for MyIpMapping {
    //Overload the mapping functions here
    fn boost() -> Option<f32> {
        Some(1.5)
    }
}
# fn main() {}
```

This will produce the following mapping:

```
# #[macro_use] extern crate serde_json;
# use elastic::types::prelude::*;
# #[derive(Default)]
# struct MyIpMapping;
# impl IpMapping for MyIpMapping {
#     //Overload the mapping functions here
#     fn boost() -> Option<f32> {
#         Some(1.5)
#     }
# }
# fn main() {
# let json = json!(
{
    "type": "ip",
    "boost": 1.5
}
# );
# let mapping = elastic::types::__derive::standalone_field_ser(MyIpMapping).unwrap();
# assert_eq!(json, mapping);
# }
```
*/
pub trait IpMapping {
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
    fn null_value() -> Option<Ipv4Addr> {
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

/** Default mapping for `geo_shape`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultIpMapping;
impl IpMapping for DefaultIpMapping {}

mod private {
    use super::{
        IpFieldType,
        IpMapping,
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
    pub struct IpPivot;

    impl<TField, TMapping> FieldType<TMapping, IpPivot> for TField
    where
        TField: IpFieldType<TMapping> + Serialize,
        TMapping: IpMapping,
    {
    }

    impl<TMapping> FieldMapping<IpPivot> for TMapping
    where
        TMapping: IpMapping,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, IpPivot>;

        fn data_type() -> &'static str {
            "ip"
        }
    }

    impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, IpPivot>
    where
        TMapping: FieldMapping<IpPivot> + IpMapping,
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
    use std::net::Ipv4Addr;

    use crate::types::{
        prelude::*,
        private::field,
    };

    #[derive(Default, Clone)]
    pub struct MyIpMapping;
    impl IpMapping for MyIpMapping {
        fn index() -> Option<bool> {
            Some(false)
        }

        fn doc_values() -> Option<bool> {
            Some(true)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn null_value() -> Option<Ipv4Addr> {
            Some(Ipv4Addr::new(127, 0, 0, 1))
        }
    }

    #[test]
    fn serialise_mapping_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultIpMapping)).unwrap();

        let expected = json!({
            "type": "ip"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_custom() {
        let ser = serde_json::to_value(&field::serialize(MyIpMapping)).unwrap();

        let expected = json!({
            "type": "ip",
            "doc_values": true,
            "index": false,
            "store": true,
            "null_value": "127.0.0.1"
        });

        assert_eq!(expected, ser);
    }

}
