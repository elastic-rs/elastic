/*!
Mapping for the Elasticsearch `number` types.

Custom mappings can be defined by implementing the right number mapping for some Rust primitive number type.
The implementation is the same for all number types, the only difference is the return type of `null_value`.

# Examples

Define a custom `IntegerMapping`:

```
# #[macro_use] use elastic::types::prelude::*;
#[derive(Default)]
struct MyIntegerMapping;
impl IntegerMapping for MyIntegerMapping {
    //Overload the mapping functions here
    fn null_value() -> Option<i32> {
        Some(42)
    }
}
# fn main() {}
```

This will produce the following mapping:

```
# #[macro_use] extern crate serde_json;
# use elastic::types::prelude::*;
# #[derive(Default)]
# struct MyIntegerMapping;
# impl IntegerMapping for MyIntegerMapping {
#   //Overload the mapping functions here
#   fn null_value() -> Option<i32> {
#       Some(42)
#   }
# }
# fn main() {
# let json = json!(
{
    "type": "integer",
    "null_value": 42
}
# );
# let mapping = elastic::types::__derive::standalone_field_ser(MyIntegerMapping).unwrap();
# assert_eq!(json, mapping);
# }
```
*/

macro_rules! number_mapping {
    ($mapping:ident, $pivot:ident, $field_trait:ident, $datatype_name:expr, $std_ty:ty, $private_mod:ident) => {
        /** A field that will be mapped as a number. */
        pub trait $field_trait<TMapping> {}

        /** Base `number` mapping. */
        pub trait $mapping
        where
            Self: Default,
        {
            /** Try to convert strings to numbers and truncate fractions for integers. Accepts `true` (default) and `false`. */
            fn coerce() -> Option<bool> {
                None
            }

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

            /**
            If `true`, malformed numbers are ignored. If `false` (default),
            malformed numbers throw an exception and reject the whole document.
            */
            fn ignore_malformed() -> Option<bool> {
                None
            }

            /**
            Whether or not the field value should be included in the `_all` field?
            Accepts `true` or `false`. Defaults to false if index is set to no,
            or if a parent object field sets `include_in_all` to false.
            Otherwise defaults to `true`.
            */
            fn include_in_all() -> Option<bool> {
                None
            }

            /** Should the field be searchable? Accepts `not_analyzed` (default) and `no`. */
            fn index() -> Option<bool> {
                None
            }

            /**
            Accepts a numeric value of the same type as the field which is substituted for any explicit null values.
            Defaults to `null`, which means the field is treated as missing.
            */
            fn null_value() -> Option<$std_ty> {
                None
            }

            /**
            Whether the field value should be stored and retrievable separately from the `_source` field.
            Accepts true or false (default).
            */
            fn store() -> Option<bool> {
                None
            }
        }

        mod $private_mod {
            use super::{$field_trait, $mapping};
            use crate::types::private::field::{FieldMapping, FieldType, SerializeFieldMapping, StaticSerialize};
            use serde::ser::SerializeStruct;
            use serde::Serialize;

            #[derive(Default)]
            pub struct $pivot;

            impl<TField, TMapping> FieldType<TMapping, $pivot> for TField
            where
                TField: $field_trait<TMapping> + Serialize,
                TMapping: $mapping,
            {
            }

            impl<TMapping> FieldMapping<$pivot> for TMapping
            where
                TMapping: $mapping,
            {
                type SerializeFieldMapping = SerializeFieldMapping<TMapping, $pivot>;

                fn data_type() -> &'static str {
                    $datatype_name
                }
            }

            impl<TMapping> StaticSerialize for SerializeFieldMapping<TMapping, $pivot>
            where
                TMapping: FieldMapping<$pivot> + $mapping,
            {
                fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    let mut state = serializer.serialize_struct("mapping", 8)?;

                    state.serialize_field("type", TMapping::data_type())?;

                    ser_field!(state, "coerce", TMapping::coerce());
                    ser_field!(state, "boost", TMapping::boost());
                    ser_field!(state, "doc_values", TMapping::doc_values());
                    ser_field!(state, "ignore_malformed", TMapping::ignore_malformed());
                    ser_field!(state, "include_in_all", TMapping::include_in_all());
                    ser_field!(state, "null_value", TMapping::null_value());
                    ser_field!(state, "store", TMapping::store());

                    state.end()
                }
            }
        }
    };
}

number_mapping!(
    IntegerMapping,
    IntegerFormat,
    IntegerFieldType,
    "integer",
    i32,
    private_i32
);
number_mapping!(
    LongMapping,
    LongFormat,
    LongFieldType,
    "long",
    i64,
    private_i64
);
number_mapping!(
    ShortMapping,
    ShortFormat,
    ShortFieldType,
    "short",
    i16,
    private_i16
);
number_mapping!(
    ByteMapping,
    ByteFormat,
    ByteFieldType,
    "byte",
    i8,
    private_i8
);
number_mapping!(
    FloatMapping,
    FloatFormat,
    FloatFieldType,
    "float",
    f32,
    private_f32
);
number_mapping!(
    DoubleMapping,
    DoubleFormat,
    DoubleFieldType,
    "double",
    f64,
    private_f64
);

/** Default mapping for an `integer` type. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultIntegerMapping;
impl IntegerMapping for DefaultIntegerMapping {}
impl IntegerFieldType<DefaultIntegerMapping> for i32 {}

/** Default mapping for a `long` type. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultLongMapping;
impl LongMapping for DefaultLongMapping {}
impl LongFieldType<DefaultLongMapping> for i64 {}
impl LongFieldType<DefaultLongMapping> for isize {}

/** Default mapping for a `short` type. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultShortMapping;
impl ShortMapping for DefaultShortMapping {}
impl ShortFieldType<DefaultShortMapping> for i16 {}

/** Default mapping for a `byte` type. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultByteMapping;
impl ByteMapping for DefaultByteMapping {}
impl ByteFieldType<DefaultByteMapping> for i8 {}

/** Default mapping for a `float` type. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultFloatMapping;
impl FloatMapping for DefaultFloatMapping {}
impl FloatFieldType<DefaultFloatMapping> for f32 {}

/** Default mapping for a `double` type. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultDoubleMapping;
impl DoubleMapping for DefaultDoubleMapping {}
impl DoubleFieldType<DefaultDoubleMapping> for f64 {}

#[cfg(test)]
mod tests {
    use serde_json;

    use crate::types::{
        prelude::*,
        private::field,
    };

    #[derive(Default, Clone)]
    pub struct MyIntegerMapping;
    impl IntegerMapping for MyIntegerMapping {
        fn coerce() -> Option<bool> {
            Some(true)
        }

        fn doc_values() -> Option<bool> {
            Some(false)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(true)
        }

        fn index() -> Option<bool> {
            Some(false)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn null_value() -> Option<i32> {
            Some(42)
        }
    }

    #[derive(Default, Clone)]
    pub struct MyLongMapping;
    impl LongMapping for MyLongMapping {
        fn coerce() -> Option<bool> {
            Some(true)
        }

        fn doc_values() -> Option<bool> {
            Some(false)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(true)
        }

        fn index() -> Option<bool> {
            Some(false)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn null_value() -> Option<i64> {
            Some(-42)
        }
    }

    #[derive(Default, Clone)]
    pub struct MyShortMapping;
    impl ShortMapping for MyShortMapping {
        fn coerce() -> Option<bool> {
            Some(true)
        }

        fn doc_values() -> Option<bool> {
            Some(false)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(true)
        }

        fn index() -> Option<bool> {
            Some(false)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn null_value() -> Option<i16> {
            Some(42)
        }
    }

    #[derive(Default, Clone)]
    pub struct MyByteMapping;
    impl ByteMapping for MyByteMapping {
        fn coerce() -> Option<bool> {
            Some(true)
        }

        fn doc_values() -> Option<bool> {
            Some(false)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(true)
        }

        fn index() -> Option<bool> {
            Some(false)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn null_value() -> Option<i8> {
            Some(1)
        }
    }

    #[derive(Default, Clone)]
    pub struct MyFloatMapping;
    impl FloatMapping for MyFloatMapping {
        fn coerce() -> Option<bool> {
            Some(true)
        }

        fn doc_values() -> Option<bool> {
            Some(false)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(true)
        }

        fn index() -> Option<bool> {
            Some(false)
        }

        fn store() -> Option<bool> {
            Some(true)
        }
    }

    #[derive(Default, Clone)]
    pub struct MyDoubleMapping;
    impl DoubleMapping for MyDoubleMapping {
        fn coerce() -> Option<bool> {
            Some(true)
        }

        fn doc_values() -> Option<bool> {
            Some(false)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(true)
        }

        fn index() -> Option<bool> {
            Some(false)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn null_value() -> Option<f64> {
            Some(-0.00002)
        }
    }

    #[test]
    fn serialise_mapping_integer_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultIntegerMapping)).unwrap();

        let expected = json!({
            "type": "integer"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_integer_custom() {
        let ser = serde_json::to_value(&field::serialize(MyIntegerMapping)).unwrap();

        let expected = json!({
            "type": "integer",
            "coerce": true,
            "doc_values": false,
            "ignore_malformed": true,
            "include_in_all": true,
            "null_value": 42,
            "store": true
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_long_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultLongMapping)).unwrap();

        let expected = json!({
            "type": "long"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_long_custom() {
        let ser = serde_json::to_value(&field::serialize(MyLongMapping)).unwrap();

        let expected = json!({
            "type": "long",
            "coerce": true,
            "doc_values": false,
            "ignore_malformed": true,
            "include_in_all": true,
            "null_value": -42,
            "store": true
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_short_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultShortMapping)).unwrap();

        let expected = json!({
            "type": "short"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_short_custom() {
        let ser = serde_json::to_value(&field::serialize(MyShortMapping)).unwrap();

        let expected = json!({
            "type": "short",
            "coerce": true,
            "doc_values": false,
            "ignore_malformed": true,
            "include_in_all": true,
            "null_value": 42,
            "store": true
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_byte_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultByteMapping)).unwrap();

        let expected = json!({
            "type": "byte"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_byte_custom() {
        let ser = serde_json::to_value(&field::serialize(MyByteMapping)).unwrap();

        let expected = json!({
            "type": "byte",
            "coerce": true,
            "doc_values": false,
            "ignore_malformed": true,
            "include_in_all": true,
            "null_value": 1,
            "store": true
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_double_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultDoubleMapping)).unwrap();

        let expected = json!({
            "type": "double"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_double_custom() {
        let ser = serde_json::to_value(&field::serialize(MyDoubleMapping)).unwrap();

        let expected = json!({
            "type": "double",
            "coerce": true,
            "doc_values": false,
            "ignore_malformed": true,
            "include_in_all": true,
            "null_value": -0.00002,
            "store": true
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_float_default() {
        let ser = serde_json::to_value(&field::serialize(DefaultFloatMapping)).unwrap();

        let expected = json!({
            "type": "float"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_float_custom() {
        let ser = serde_json::to_value(&field::serialize(MyFloatMapping)).unwrap();

        let expected = json!({
            "type": "float",
            "coerce": true,
            "doc_values": false,
            "ignore_malformed": true,
            "include_in_all": true,
            "store": true
        });

        assert_eq!(expected, ser);
    }
}
