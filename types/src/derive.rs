/*! Functions that are exported and used by `elastic_types_derive`. */

use std::borrow::Cow;
use chrono::{DateTime, Utc};
use chrono::format::{self, Item, Parsed};

use private::field::FieldMapping;

pub use date::{DateFormat, ParseError, FormattedDate, ParsableDate};
pub use document::{DocumentType, FieldType, field_ser};
pub use document::mapping::{DocumentMapping, PropertiesMapping};

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
pub fn parse_from_tokens<'a, P>(date: P, fmt: Vec<Item<'a>>) -> Result<DateTime<Utc>, ParseError> 
    where P: Into<ParsableDate<'a>>
{
    let date = date.into();

    let mut parsed = Parsed::new();
    match format::parse(&mut parsed, date.as_ref(), fmt.into_iter()) {
        Ok(_) => {
            // If the parsed result doesn't contain any time, set it to the default
            if parsed.hour_mod_12.is_none() {
                let _ = parsed.set_hour(0);
                let _ = parsed.set_minute(0);
            }

            // Set the DateTime result
            let dt = try!(parsed.to_naive_datetime_with_offset(0));
            Ok(DateTime::from_utc(dt, Utc))
        }
        Err(e) => Err(e.into()),
    }
}

/** Format a date string using an owned slice of items. */
pub fn format_with_tokens<'a>(date: Cow<'a, DateTime<Utc>>, fmt: Vec<Item<'a>>) -> FormattedDate<'a> {
    date.format_with_items(fmt.into_iter()).into()
}
