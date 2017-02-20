use std::marker::PhantomData;
use chrono::{UTC, NaiveDateTime, NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::ChronoDateTime;
use super::format::{DateFormat, ParseError};
use super::formats::ChronoFormat;
use super::mapping::{DateMapping, DefaultDateMapping, DateFormatWrapper};
use ::field::FieldType;

pub use chrono::{Datelike, Timelike};

impl FieldType<DefaultDateMapping<ChronoFormat>, DateFormatWrapper<ChronoFormat>> for ChronoDateTime {}

/// An Elasticsearch `date` type with a required `time` component.
///
/// The [format](format/index.html) is provided as a generic parameter.
/// This struct wraps up a `chrono::DateTime<UTC>` struct, meaning storing time in `UTC` is required.
///
/// # Examples
///
/// Defining a date using the default format:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date: Date<DefaultDateFormat> = Date::now();
/// ```
///
/// Defining a date using a named format:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date = Date::<BasicDateTime>::now();
/// ```
///
/// Defining a date using a custom mapping:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date: Date<BasicDateTime, DefaultDateMapping<_>> = Date::now();
/// ```
///
/// Accessing the values of a date:
///
/// ```
/// # use elastic_types::prelude::*;
/// let date = Date::<BasicDateTime>::now();
///
/// //eg: 2010/04/30 13:56:59.372
/// println!("{}/{}/{} {}:{}:{}.{}",
///         date.year(),
///     date.month(),
///     date.day(),
///     date.hour(),
///     date.minute(),
///     date.second(),
///     date.nanosecond() / 1000000
/// );
/// ```
///
/// # Links
///
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
#[derive(Debug, Clone, PartialEq)]
pub struct Date<F, M = DefaultDateMapping<F>>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    value: ChronoDateTime,
    _t: PhantomData<(M, F)>,
}

impl<F, M> Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    /// Creates a new `Date` from the given `chrono::DateTime<UTC>`.
    ///
    /// This function will consume the provided `chrono` date.
    ///
    /// # Examples
    ///
    /// Create an `Date` from the given `chrono::DateTime`:
    ///
    /// ```
    /// # extern crate elastic_types;
    /// # extern crate chrono;
    /// # fn main() {
    /// use chrono::UTC;
    /// use elastic_types::date::{ Date, DefaultDateFormat };
    ///
    /// //Create a chrono DateTime struct
    /// let chronoDate = UTC::now();
    ///
    /// //Give it to the Date struct
    /// let esDate: Date<DefaultDateFormat> = Date::new(chronoDate);
    /// # }
    /// ```
    pub fn new(date: ChronoDateTime) -> Date<F, M> {
        Date {
            value: date,
            _t: PhantomData,
        }
    }

    /// Creates an `Date` from the given UTC primitives:
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let esDate: Date<DefaultDateFormat> = Date::build(
    ///     2015,
    ///     5,
    ///     14,
    ///     16,
    ///     45,
    ///     8,
    ///     886
    /// );
    /// ```
    pub fn build(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, milli: u32) -> Date<F, M> {
        Date {
            value: ChronoDateTime::from_utc(NaiveDateTime::new(NaiveDate::from_ymd(year, month, day),
                                                               NaiveTime::from_hms_milli(hour, minute, second, milli)),
                                            UTC),
            _t: PhantomData,
        }
    }

    /// Gets the current system time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let date: Date<DefaultDateFormat> = Date::now();
    /// ```
    pub fn now() -> Date<F, M> {
        Date {
            value: UTC::now(),
            _t: PhantomData,
        }
    }

    /// Parse the date and time from a string.
    ///
    /// The format of the string must match the given `DateFormat`.
    ///
    /// # Examples
    ///
    /// Parsing from a specified `DateFormat`.
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let date = Date::<BasicDateTime>::parse("20151126T145543.778Z").unwrap();
    /// ```
    pub fn parse(date: &str) -> Result<Date<F, M>, ParseError> {
        F::parse(date).map(Date::new)
    }

    /// Format the date and time as a string.
    ///
    /// The format of the string is specified by the given `DateFormat`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// let date: Date<BasicDateTime> = Date::now();
    /// let fmt = date.format();
    ///
    /// //eg: 20151126T145543.778Z
    /// println!("{}", fmt);
    /// ```
    pub fn format(&self) -> String {
        F::format(&self.value)
    }

    /// Change the format/mapping of this date.
    ///
    /// # Examples
    ///
    /// ```
    /// # use elastic_types::prelude::*;
    /// //Get the current datetime formatted as basic_date_time
    /// let date: Date<BasicDateTime> = Date::now();
    ///
    /// //Change the format to epoch_millis
    /// let otherdate: Date<EpochMillis> = date.remap();
    /// ```
    pub fn remap<FInto, MInto>(self) -> Date<FInto, MInto>
        where FInto: DateFormat,
              MInto: DateMapping<Format = FInto>
    {
        Date::<FInto, MInto>::new(self.value)
    }
}

impl<F, M> FieldType<M, DateFormatWrapper<F>> for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
}

impl_mapping_type!(ChronoDateTime, Date, DateMapping, DateFormat);

impl<F, M> Default for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn default() -> Date<F, M> {
        Date::<F, M>::now()
    }
}

impl<F, M> Serialize for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.format())
    }
}

impl<F, M> Deserialize for Date<F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn deserialize<D>(deserializer: D) -> Result<Date<F, M>, D::Error>
        where D: Deserializer
    {
        #[derive(Default)]
        struct DateTimeVisitor<F, M>
            where F: DateFormat,
                  M: DateMapping<Format = F>
        {
            _t: PhantomData<(M, F)>,
        }

        impl<F, M> Visitor for DateTimeVisitor<F, M>
            where F: DateFormat,
                  M: DateMapping<Format = F>
        {
            type Value = Date<F, M>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter,
                       "a json string or number containing a formatted date")
            }

            fn visit_str<E>(self, v: &str) -> Result<Date<F, M>, E>
                where E: Error
            {
                let result = Date::<F, M>::parse(v);
                result.map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Date<F, M>, E>
                where E: Error
            {
                let result = Date::<F, M>::parse(&v.to_string());
                result.map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Date<F, M>, E>
                where E: Error
            {
                let result = Date::<F, M>::parse(&v.to_string());
                result.map_err(|err| Error::custom(format!("{}", err)))
            }
        }

        deserializer.deserialize(DateTimeVisitor::<F, M>::default())
    }
}

#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct DateBrw<'a, F, M = DefaultDateMapping<F>>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    value: &'a ChronoDateTime,
    _t: PhantomData<(M, F)>,
}

impl<'a, F, M> DateBrw<'a, F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    #[doc(hidden)]
    pub fn new(date: &'a ChronoDateTime) -> DateBrw<'a, F, M> {
        DateBrw {
            value: date,
            _t: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn format(&self) -> String {
        F::format(&self.value)
    }
}

impl<'a, F, M> FieldType<M, DateFormatWrapper<F>> for DateBrw<'a, F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
}

impl<'a, F, M> Serialize for DateBrw<'a, F, M>
    where F: DateFormat,
          M: DateMapping<Format = F>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&self.format())
    }
}
