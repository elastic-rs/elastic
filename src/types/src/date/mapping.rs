/*! Mapping for the Elasticsearch `date` type. */

use std::marker::PhantomData;
use super::{Date, DateFormat, DefaultDateFormat, FormattableDateValue};

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
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use std::marker::PhantomData;
# use elastic_types::prelude::*;
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
# #[macro_use]
# extern crate json_str;
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# extern crate serde_json;
# use std::marker::PhantomData;
# use elastic_types::prelude::*;
# #[derive(Default)]
# struct MyDateMapping;
# impl DateMapping for MyDateMapping {
#     type Format = EpochMillis;
#     fn boost() -> Option<f32> {
#         Some(1.5)
#     }
# }
# fn main() {
# let mapping = elastic_types::derive::standalone_field_ser(MyDateMapping).unwrap();
# let json = json_str!(
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
# #[macro_use]
# extern crate elastic_types;
# extern crate serde;
# use std::marker::PhantomData;
# use elastic_types::prelude::*;
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
pub trait DateMapping
where
    Self: Default,
{
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
    fn null_value() -> Option<Date<Self>> {
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
    use serde::{Serialize, Serializer};
    use serde::ser::SerializeStruct;
    use date::{DateFormat, FormattableDateValue};
    use private::field::{DocumentField, FieldMapping, FieldType};
    use super::{DateFieldType, DateMapping};

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
        type DocumentField = DocumentField<TMapping, DatePivot>;

        fn data_type() -> &'static str {
            "date"
        }
    }

    impl<TMapping, TFormat> Serialize for DocumentField<TMapping, DatePivot>
    where
        TMapping: FieldMapping<DatePivot> + DateMapping<Format = TFormat>,
        TFormat: DateFormat,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = try!(serializer.serialize_struct("mapping", 9));

            try!(state.serialize_field("type", TMapping::data_type()));
            try!(state.serialize_field("format", TMapping::Format::name()));

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
    use chrono::{DateTime, Utc};

    use prelude::*;
    use private::field::{DocumentField, FieldType};

    #[derive(Default, Clone)]
    pub struct MyDateMapping;
    impl DateMapping for MyDateMapping {
        type Format = EpochMillis;

        fn null_value() -> Option<Date<Self>> {
            Some(Date::build(2015, 3, 14, 16, 45, 13, 778))
        }

        fn boost() -> Option<f32> {
            Some(1.01)
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
    fn datetime_has_default_mapping() {
        assert_eq!(
            DefaultDateMapping::<ChronoFormat>::default(),
            DateTime::<Utc>::field_mapping()
        );
    }

    #[test]
    fn serialise_mapping_default() {
        let ser = serde_json::to_string(&DocumentField::from(
            DefaultDateMapping::<DefaultDateFormat>::default(),
        )).unwrap();

        let expected = json_str!({
            "type": "date",
            "format": "basic_date_time"
        });

        assert_eq!(expected, ser);
    }

    #[test]
    fn serialise_mapping_custom() {
        let ser = serde_json::to_string(&DocumentField::from(MyDateMapping)).unwrap();

        let expected = json_str!({
            "type": "date",
            "format": "epoch_millis",
            "boost": 1.01,
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
