/*! Mapping for the Elasticsearch `ip` type. !*/

use std::net::Ipv4Addr;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use private::field::{DocumentField, FieldMapping, SerializeField};
use document::FieldType;

/** A field that will be mapped as an `ip`. **/
pub trait IpFieldType<M> where M: IpMapping {}

impl<T, M> FieldType<M, IpFormat> for T
    where M: IpMapping,
          T: IpFieldType<M> + Serialize
{
}

#[derive(Default)]
struct IpFormat;

/**
The base requirements for mapping a `ip` type.

Custom mappings can be defined by implementing `IpMapping`.

# Examples

Define a custom `IpMapping`:

## Derive Mapping

```
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use elastic_types::prelude::*;
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
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# #[cfg(feature = "nightly")]
# extern crate serde_json;
# use elastic_types::prelude::*;
# #[derive(Default)]
# struct MyIpMapping;
# impl IpMapping for MyIpMapping {
#     //Overload the mapping functions here
#     fn boost() -> Option<f32> {
#         Some(1.5)
#     }
# }
# fn main() {
# let json = json_str!(
{
    "type": "ip",
    "boost": 1.5
}
# );
# #[cfg(feature = "nightly")]
# let mapping = serde_json::to_string(&DocumentField::from(MyIpMapping)).unwrap();
# #[cfg(not(feature = "nightly"))]
# let mapping = json.clone();
# assert_eq!(json, mapping);
# }
```
**/
pub trait IpMapping
    where Self: Default
{
    /** Field-level index time boosting. Accepts a floating point number, defaults to `1.0`. **/
    fn boost() -> Option<f32> {
        None
    }

    /**
    Should the field be stored on disk in a column-stride fashion,
    so that it can later be used for sorting, aggregations, or scripting?
    Accepts `true` (default) or `false`.
    **/
    fn doc_values() -> Option<bool> {
        None
    }

    /** Should the field be searchable? Accepts `not_analyzed` (default) and `no`. **/
    fn index() -> Option<bool> {
        None
    }

    /**
    Accepts a string value which is substituted for any explicit null values.
    Defaults to `null`, which means the field is treated as missing.
    **/
    fn null_value() -> Option<Ipv4Addr> {
        None
    }

    /**
    Whether the field value should be stored and retrievable separately from the `_source` field.
    Accepts `true` or `false` (default).
    **/
    fn store() -> Option<bool> {
        None
    }
}

impl<T> FieldMapping<IpFormat> for T
    where T: IpMapping
{
    fn data_type() -> &'static str {
        "ip"
    }
}

impl<T> SerializeField<IpFormat> for T
    where T: IpMapping
{
    type Field = DocumentField<T, IpFormat>;
}

impl<T> Serialize for DocumentField<T, IpFormat>
    where T: FieldMapping<IpFormat> + IpMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("mapping", 6));

        try!(state.serialize_field("type", T::data_type()));

        ser_field!(state, "boost", T::boost());
        ser_field!(state, "doc_values", T::doc_values());
        ser_field!(state, "index", T::index());
        ser_field!(state, "store", T::store());
        ser_field!(state, "null_value", T::null_value());

        state.end()
    }
}

/** Default mapping for `geo_shape`. **/
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultIpMapping;
impl IpMapping for DefaultIpMapping {}

#[cfg(test)]
mod tests {
    use serde_json;
    use std::net::Ipv4Addr;

    use prelude::*;
    use private::field::DocumentField;

    #[derive(Default, Clone)]
    pub struct MyIpMapping;
    impl IpMapping for MyIpMapping {
        fn boost() -> Option<f32> {
            Some(1.01)
        }

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
    fn ipv4addr_has_default_mapping() {
        assert_eq!(DefaultIpMapping, Ipv4Addr::mapping());
    }

    #[test]
    fn serialise_mapping_default() {
        let ser = serde_json::to_string(&DocumentField::from(DefaultIpMapping)).unwrap();

        let expected = json_str!({
            "type": "ip"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_custom() {
        let ser = serde_json::to_string(&DocumentField::from(MyIpMapping)).unwrap();

        let expected = json_str!({
            "type": "ip",
            "boost": 1.01,
            "doc_values": true,
            "index": false,
            "store": true,
            "null_value": "127.0.0.1"
        });

        assert_eq!(expected, ser);
    }

}
