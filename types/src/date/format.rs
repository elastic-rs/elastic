use chrono;
use chrono::{DateTime, Utc};
use chrono::format::{Item, DelayedFormat};
use std::borrow::Cow;
use std::ops::Deref;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};
use std::vec::IntoIter;

/**
A format used for parsing and formatting dates.

The format is specified as two functions: `parse` and `format`.

# Examples

The easiest way to implement `DateFormat` is to derive `ElasticDateFormat`
on a unit struct:

```
# #[macro_use]
# extern crate elastic_types;
# #[macro_use]
# extern crate elastic_types_derive;
# extern crate chrono;
# use elastic_types::prelude::*;
# fn main() {
#[derive(Default, ElasticDateFormat)]
#[elastic(date_format="yyyy-MM-dd'T'HH:mm:ss")]
struct MyFormat;
# }
```

The `#[elastic(date_format)]` attribute is required,
and must contain a valid [format string](http://www.joda.org/joda-time/apidocs/org/joda/time/format/DateTimeFormat.html).

> NOTE: Only a small subset of the Joda time format is supported.

You can customise the indexed format name by adding an `#[elastic(date_format_name)]` attribute:

```
# #[macro_use]
# extern crate elastic_types;
# #[macro_use]
# extern crate elastic_types_derive;
# extern crate chrono;
# use elastic_types::prelude::*;
# fn main() {
#[derive(Default, ElasticDateFormat)]
#[elastic(date_format="yyyyMMdd'T'HHmmssZ", date_format_name="basic_date_time_no_millis")]
struct MyFormat;
# }
```
*/
pub trait DateFormat
    where Self: Default
{
    /** Parses a date string to a `chrono::DateTime<Utc>` result. */
    fn parse<'a, P>(date: P) -> Result<DateTime<Utc>, ParseError> where P: Into<ParsableDate<'a>>;

    /** Formats a given `chrono::DateTime<Utc>` as a string. */
    fn format<'a>(date: Cow<'a, DateTime<Utc>>) -> FormattedDate<'a>;

    /**
    The name of the format.
    
    This is the string used when defining the format in the field mapping.
    */
    fn name() -> &'static str;
}

/**
A formatted date.

This type can avoid allocating strings for date formats.
*/
pub struct FormattedDate<'a> {
    inner: FormattedDateInner<'a>,
}

enum FormattedDateInner<'a> {
    Delayed(DelayedFormat<IntoIter<Item<'a>>>),
    Buffered(String),
    Number(i64),
}

/**
A parsable date.

This type represents a date that can be parsed.
*/
pub struct ParsableDate<'a>(Cow<'a, str>);

impl<'a> Deref for ParsableDate<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<String> for ParsableDate<'a> {
    fn from(s: String) -> Self {
        ParsableDate(Cow::Owned(s))
    }
}

impl<'a> From<&'a str> for ParsableDate<'a> {
    fn from(s: &'a str) -> Self {
        ParsableDate(Cow::Borrowed(s))
    }
}

impl<'a> From<FormattedDate<'a>> for ParsableDate<'a> {
    fn from(date: FormattedDate<'a>) -> Self {
        let inner = match date.inner {
            FormattedDateInner::Buffered(s) => s,
            _ => date.to_string()
        };

        ParsableDate(Cow::Owned(inner))
    }
}

impl<'a> Display for FormattedDateInner<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        fn fmt_inner<T>(inner: &T, f: &mut Formatter) -> FmtResult
            where T: Display
        {
            inner.fmt(f)
        }

        match *self {
            FormattedDateInner::Delayed(ref inner) => fmt_inner(inner, f),
            FormattedDateInner::Buffered(ref inner) => fmt_inner(inner, f),
            FormattedDateInner::Number(ref inner) => fmt_inner(inner, f),
        }
    }
}

impl<'a> Display for FormattedDate<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.inner.fmt(f)
    }
}

impl<'a> From<DelayedFormat<IntoIter<Item<'a>>>> for FormattedDate<'a> {
    fn from(formatted: DelayedFormat<IntoIter<Item<'a>>>) -> Self {
        FormattedDate { inner: FormattedDateInner::Delayed(formatted) }
    }
}

impl<'a> From<String> for FormattedDate<'a> {
    fn from(formatted: String) -> Self {
        FormattedDate { inner: FormattedDateInner::Buffered(formatted) }
    }
}

impl<'a> From<i64> for FormattedDate<'a> {
    fn from(formatted: i64) -> Self {
        FormattedDate { inner: FormattedDateInner::Number(formatted) }
    }
}

/** Represents an error encountered during parsing. */
#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
}

#[derive(Debug)]
enum ParseErrorKind {
    Chrono(chrono::ParseError),
    Other(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => write!(f, "Chrono error: {}", err),
            ParseErrorKind::Other(ref err) => write!(f, "Error: {}", err),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => err.description(),
            ParseErrorKind::Other(ref err) => &err[..],
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => Some(err),
            ParseErrorKind::Other(_) => None,
        }
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(err: chrono::ParseError) -> ParseError {
        ParseError { kind: ParseErrorKind::Chrono(err) }
    }
}

impl From<String> for ParseError {
    fn from(err: String) -> ParseError {
        ParseError { kind: ParseErrorKind::Other(err) }
    }
}
