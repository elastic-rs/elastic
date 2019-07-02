use super::ChronoDateTime;
use chrono::{
    self,
    format::{
        DelayedFormat,
        Item,
    },
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    Utc,
};
use std::{
    borrow::Borrow,
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    marker::PhantomData,
    ops::Deref,
    vec::IntoIter,
};

/**
A date value produced and consumed by date formats.

`DateValue` is a very thin wrapper over `DateTime<Utc>` that doesn't carry any formatting semantics.
Like `FormattableDateValue`, this type is used for binding generics in methods that accept date values but it ignores any format on the input type.
You probably won't need to use it directly except to clobber the format on a `Date<M>` or `DateTime<Utc>` value.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct DateValue(ChronoDateTime);

impl DateValue {
    /** Equivalent to `DateTime<Utc>::now()` */
    pub fn now() -> Self {
        DateValue(Utc::now())
    }

    /** Construct a `DateValue` from individual parts. */
    pub fn build(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        milli: u32,
    ) -> Self {
        let ndate = NaiveDate::from_ymd(year, month, day);
        let ntime = NaiveTime::from_hms_milli(hour, minute, second, milli);

        let date = ChronoDateTime::from_utc(NaiveDateTime::new(ndate, ntime), Utc);

        DateValue(date)
    }
}

impl<TFormat> From<FormattableDateValue<TFormat>> for DateValue {
    fn from(date: FormattableDateValue<TFormat>) -> Self {
        date.0
    }
}

impl From<ChronoDateTime> for DateValue {
    fn from(date: ChronoDateTime) -> Self {
        DateValue(date)
    }
}

impl PartialEq<ChronoDateTime> for DateValue {
    fn eq(&self, other: &ChronoDateTime) -> bool {
        PartialEq::eq(&self.0, other)
    }

    fn ne(&self, other: &ChronoDateTime) -> bool {
        PartialEq::ne(&self.0, other)
    }
}

impl PartialEq<DateValue> for ChronoDateTime {
    fn eq(&self, other: &DateValue) -> bool {
        PartialEq::eq(self, &other.0)
    }

    fn ne(&self, other: &DateValue) -> bool {
        PartialEq::ne(self, &other.0)
    }
}

impl Borrow<ChronoDateTime> for DateValue {
    fn borrow(&self) -> &ChronoDateTime {
        &self.0
    }
}

impl Deref for DateValue {
    type Target = ChronoDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/**
A date value paired with a format.

`FormattableDateValue<F>` bundles a `DateValue` with a specific format and is used to ensure the formats of mappable date types aren't accidentally changed.
Like `DateValue`, this type is used for binding generics in methods that accept date values but it requires the input type uses a specific format.
You probably don't need to use it directly except to ensure date formats aren't silently changed.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct FormattableDateValue<TFormat>(DateValue, PhantomData<TFormat>);

impl<TFormat> FormattableDateValue<TFormat>
where
    TFormat: DateFormat,
{
    /** Format the wrapped date value using the generic format. */
    pub fn format<'a>(&'a self) -> FormattedDate<'a> {
        TFormat::format(&self.0)
    }

    /** Parse a date value using the generic format. */
    pub fn parse(date: &str) -> Result<Self, ParseError> {
        let date = TFormat::parse(date)?;

        Ok(FormattableDateValue::from(date))
    }
}

impl<TFormat> From<DateValue> for FormattableDateValue<TFormat> {
    fn from(date: DateValue) -> Self {
        FormattableDateValue(date.into(), PhantomData)
    }
}

impl<TFormat> Borrow<ChronoDateTime> for FormattableDateValue<TFormat> {
    fn borrow(&self) -> &ChronoDateTime {
        self.0.borrow()
    }
}

impl<TFormat> PartialEq<ChronoDateTime> for FormattableDateValue<TFormat> {
    fn eq(&self, other: &ChronoDateTime) -> bool {
        PartialEq::eq(&self.0, other)
    }

    fn ne(&self, other: &ChronoDateTime) -> bool {
        PartialEq::ne(&self.0, other)
    }
}

impl<TFormat> PartialEq<FormattableDateValue<TFormat>> for ChronoDateTime {
    fn eq(&self, other: &FormattableDateValue<TFormat>) -> bool {
        PartialEq::eq(self, &other.0)
    }

    fn ne(&self, other: &FormattableDateValue<TFormat>) -> bool {
        PartialEq::ne(self, &other.0)
    }
}

/**
A format used for parsing and formatting dates.

The format is specified as two functions: `parse` and `format`.
A general `DateValue` is used as an intermediate value passed as input and produced as output for formatting.

# Examples

The easiest way to implement `DateFormat` is to derive `ElasticDateFormat`
on a unit struct:

```
# #[macro_use] extern crate elastic_derive;
# use elastic::types::prelude::*;
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
# #[macro_use] extern crate elastic_derive;
# use elastic::types::prelude::*;
# fn main() {
#[derive(Default, ElasticDateFormat)]
#[elastic(date_format="yyyyMMdd'T'HHmmssZ", date_format_name="basic_date_time_no_millis")]
struct MyFormat;
# }
```
*/
pub trait DateFormat {
    /** Parses a date string to a `chrono::DateTime<Utc>` result. */
    fn parse(date: &str) -> Result<DateValue, ParseError>;

    /** Formats a given `chrono::DateTime<Utc>` as a string. */
    fn format<'a>(date: &'a DateValue) -> FormattedDate<'a>;

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

impl<'a> Display for FormattedDateInner<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        fn fmt_inner<T>(inner: &T, f: &mut Formatter) -> FmtResult
        where
            T: Display,
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
        FormattedDate {
            inner: FormattedDateInner::Delayed(formatted),
        }
    }
}

impl<'a> From<String> for FormattedDate<'a> {
    fn from(formatted: String) -> Self {
        FormattedDate {
            inner: FormattedDateInner::Buffered(formatted),
        }
    }
}

impl<'a> From<i64> for FormattedDate<'a> {
    fn from(formatted: i64) -> Self {
        FormattedDate {
            inner: FormattedDateInner::Number(formatted),
        }
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

    fn cause(&self) -> Option<&dyn Error> {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => Some(err),
            ParseErrorKind::Other(_) => None,
        }
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(err: chrono::ParseError) -> ParseError {
        ParseError {
            kind: ParseErrorKind::Chrono(err),
        }
    }
}

impl From<String> for ParseError {
    fn from(err: String) -> ParseError {
        ParseError {
            kind: ParseErrorKind::Other(err),
        }
    }
}
