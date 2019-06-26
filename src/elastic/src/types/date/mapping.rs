/*! Mapping for the Elasticsearch `date` type. */

use super::{
    Date,
    DateFormat,
    DefaultDateFormat,
    FormattableDateValue,
};
use std::marker::PhantomData;

/** A field that will be mapped as a `date`. */
pub trait DateFieldType<TMapping>
where
    Self: Into<FormattableDateValue<TMapping::Format>>,
    TMapping: DateMapping,
{
}

/**
The base requirements for mapping a `date` type.

# Examples

Define a custom `DateMapping`:

Define a custom `DateMapping` that's valid for a single `DateFormat`:

```
# #[macro_use] use std::marker::PhantomData;
# use elastic::types::prelude::*;
#[derive(Default)]
struct MyDateMapping;
impl DateMapping for MyDateMapping {
    type Format = EpochMillis;

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
# use std::marker::PhantomData;
# use elastic::types::prelude::*;
# #[derive(Default)]
# struct MyDateMapping;
# impl DateMapping for MyDateMapping {
#     type Format = EpochMillis;
#     fn boost() -> Option<f32> {
#         Some(1.5)
#     }
# }
# fn main() {
# let mapping = elastic::types::__derive::standalone_field_ser(MyDateMapping).unwrap();
# let json = json!(
{
    "type": "date",
    "format": "epoch_millis",
    "boost": 1.5
}
# );
# assert_eq!(json, mapping);
# }
```

## Map with a generic Format

You can use a generic input parameter to make your `DateMapping` work for any kind of `DateFormat`:

```
# #[macro_use] use std::marker::PhantomData;
# use elastic::types::prelude::*;
#[derive(Default)]
struct MyDateMapping<F> {
    _marker: PhantomData<F>
}

impl <F> DateMapping for MyDateMapping<F>
    where F: DateFormat
{
    type Format = F;
}
# fn main() {}
```

This is how `DefaultDateMapping` is able to support any format.
*/
pub trait DateMapping {
    /**
    The date format bound to this mapping.

    The value of `Format::name()` is what's sent to Elasticsearch as the format to use.
    This is also used to serialise and deserialise formatted `Date`s.
    */
    type Format: DateFormat;

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
    Whether or not the field value should be included in the `_all` field?
    Accepts true or false.
    Defaults to `false` if index is set to `no`, or if a parent object field sets `include_in_all` to false.
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
    Whether the field value should be stored and retrievable separately from the `_source` field.
    Accepts `true` or `false` (default).
    */
    fn store() -> Option<bool> {
        None
    }

    /**
    If `true`, malformed numbers are ignored.
    If `false` (default), malformed numbers throw an exception and reject the whole document.
    */
    fn ignore_malformed() -> Option<bool> {
        None
    }

    /**
    Accepts a date value in one of the configured format's as the field which is substituted for any explicit null values.
    Defaults to `null`, which means the field is treated as missing.
    */
    fn null_value() -> Option<Date<Self>>
    where
        Self: Sized,
    {
        None
    }
}

/** Default mapping for `date`. */
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct DefaultDateMapping<TFormat = DefaultDateFormat>
where
    TFormat: DateFormat,
{
    _f: PhantomData<TFormat>,
}

impl<TFormat> DateMapping for DefaultDateMapping<TFormat>
where
    TFormat: DateFormat,
{
    type Format = TFormat;
}

mod private {
    use super::{
        DateFieldType,
        DateMapping,
    };
    use crate::types::{
        date::{
            DateFormat,
            FormattableDateValue,
        },
        private::field::{
            FieldMapping,
            FieldType,
            SerializeFieldMapping,
            StaticSerialize,
        },
    };
    use serde::{
        ser::SerializeStruct,
        Serialize,
        Serializer,
    };

    impl<TField, TMapping> FieldType<TMapping, DatePivot> for TField
    where
        TField: DateFieldType<TMapping> + Serialize,
        TField: Into<FormattableDateValue<TMapping::Format>>,
        TMapping: DateMapping,
    {
    }

    #[derive(Default)]
    pub struct DatePivot;

    impl<TMapping, TFormat> FieldMapping<DatePivot> for TMapping
    where
        TMapping: DateMapping<Format = TFormat>,
        TFormat: DateFormat,
    {
        type SerializeFieldMapping = SerializeFieldMapping<TMapping, DatePivot>;

        fn data_type() -> &'static str {
            "date"
        }
    }

    impl<TMapping, TFormat> StaticSerialize for SerializeFieldMapping<TMapping, DatePivot>
    where
        TMapping: FieldMapping<DatePivot> + DateMapping<Format = TFormat>,
        TFormat: DateFormat,
    {
        fn static_serialize<S>(serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("mapping", 9)?;

            state.serialize_field("type", TMapping::data_type())?;
            state.serialize_field("format", TMapping::Format::name())?;

            ser_field!(state, "boost", TMapping::boost());
            ser_field!(state, "doc_values", TMapping::doc_values());
            ser_field!(state, "include_in_all", TMapping::include_in_all());
            ser_field!(state, "index", TMapping::index());
            ser_field!(state, "store", TMapping::store());
            ser_field!(state, "ignore_malformed", TMapping::ignore_malformed());
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
    pub struct MyDateMapping;
    impl DateMapping for MyDateMapping {
        type Format = EpochMillis;

        fn null_value() -> Option<Date<Self>> {
            Some(Date::build(2015, 3, 14, 16, 45, 13, 778))
        }

        fn index() -> Option<bool> {
            Some(true)
        }

        fn doc_values() -> Option<bool> {
            Some(true)
        }

        fn include_in_all() -> Option<bool> {
            Some(false)
        }

        fn store() -> Option<bool> {
            Some(true)
        }

        fn ignore_malformed() -> Option<bool> {
            Some(true)
        }
    }

    #[test]
    fn serialise_mapping_default() {
        let ser = serde_json::to_value(&field::serialize(
            DefaultDateMapping::<DefaultDateFormat>::default(),
        ))
        .unwrap();

        let expected = json!({
            "type": "date",
            "format": "basic_date_time"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_custom() {
        let ser = serde_json::to_value(&field::serialize(MyDateMapping)).unwrap();

        let expected = json!({
            "type": "date",
            "format": "epoch_millis",
            "doc_values": true,
            "include_in_all": false,
            "index": true,
            "store": true,
            "ignore_malformed": true,
            "null_value": "1426351513778"
        });

        assert_eq!(expected, ser);
    }

}
