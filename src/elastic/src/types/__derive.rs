/*!
Functions that are exported and used by `elastic_types_derive`.

This module is 'private' and should only be consumed by `elastic_types_derive`.
Its contents aren't subject to SemVer.
*/

use chrono::{
    format::{
        self,
        Parsed,
    },
    DateTime,
    Utc,
};
use serde::Serialize;
use serde_json;

use crate::types::private::field::{
    FieldMapping,
    FieldType,
    SerializeFieldMapping,
};

pub use crate::types::{
    date::{
        DateFormat,
        DateValue,
        FormattedDate,
        ParseError,
    },
    document::{
        mapping::{
            ObjectFieldType,
            ObjectMapping,
            PropertiesMapping,
        },
        DocumentType,
        Id,
        Index,
        StaticIndex,
        StaticType,
        Type,
        DEFAULT_DOC_TYPE,
    },
};

pub use chrono::format::{
    Fixed,
    Item,
    Numeric,
    Pad,
};
pub use serde::ser::SerializeStruct;

/** Serialise a field mapping as a field using the given serialiser. */
#[inline]
pub fn field_ser<TField, TMapping, TPivot, S>(
    state: &mut S,
    field: &'static str,
) -> Result<(), S::Error>
where
    TField: FieldType<TMapping, TPivot>,
    TMapping: FieldMapping<TPivot>,
    S: SerializeStruct,
    SerializeFieldMapping<TMapping, TPivot>: Serialize,
{
    state.serialize_field(field, &SerializeFieldMapping::<TMapping, TPivot>::default())
}

/**
Serialize a field individually.

This method isn't intended to be used publicly, but is useful in the docs.
*/
#[inline]
pub fn standalone_field_ser<TMapping, TPivot>(
    _: TMapping,
) -> Result<serde_json::Value, serde_json::Error>
where
    TMapping: FieldMapping<TPivot>,
    SerializeFieldMapping<TMapping, TPivot>: Serialize,
{
    serde_json::to_value(&SerializeFieldMapping::<TMapping, TPivot>::default())
}

/** Parse a date string using an owned slice of items. */
pub fn parse_from_tokens<'a>(date: &str, fmt: Vec<Item<'a>>) -> Result<DateValue, ParseError> {
    let mut parsed = Parsed::new();
    match format::parse(&mut parsed, date, fmt.into_iter()) {
        Ok(_) => {
            // If the parsed result doesn't contain any time, set it to the default
            if parsed.hour_mod_12.is_none() {
                let _ = parsed.set_hour(0);
                let _ = parsed.set_minute(0);
            }

            // Set the DateTime result
            let naive_date = parsed.to_naive_datetime_with_offset(0)?;

            let date = DateTime::from_utc(naive_date, Utc);

            Ok(date.into())
        }
        Err(e) => Err(e.into()),
    }
}

/** Format a date string using an owned slice of items. */
pub fn format_with_tokens<'a>(date: &'a DateValue, fmt: Vec<Item<'a>>) -> FormattedDate<'a> {
    date.format_with_items(fmt.into_iter()).into()
}
