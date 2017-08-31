/*! Functions that are exported and used by `elastic_types_derive`. */

use chrono::{DateTime, Utc};
use chrono::format::{self, Parsed};

use private::field::FieldMapping;

pub use date::{DateFormat, DateValue, ParseError, FormattedDate};
pub use document::{DocumentType, FieldType, field_ser};
pub use document::mapping::{DocumentMapping, PropertiesMapping};

pub use chrono::format::{Item, Pad, Numeric, Fixed};
pub use serde::ser::SerializeStruct;

/** Get the mapping for a field. */
#[inline]
pub fn mapping<T, M, F>() -> M
    where T: FieldType<M, F>,
          M: FieldMapping<F>,
          F: Default
{
    T::mapping()
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
