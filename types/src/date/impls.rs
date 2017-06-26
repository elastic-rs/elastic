use std::marker::PhantomData;
use std::borrow::Cow;
use std::fmt::{Display, Result as FmtResult, Formatter};
use chrono::{DateTime, Utc, NaiveDateTime, NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{Visitor, Error};
use super::formats::ChronoFormat;
use super::mapping::{DateType, DateFieldType, DateMapping, DefaultDateMapping};

pub use chrono::{Datelike, Timelike};

/** A re-export of the `chrono::DateTime` struct with `Utc` timezone. */
pub type ChronoDateTime = DateTime<Utc>;

impl DateFieldType<DefaultDateMapping<ChronoFormat>> for ChronoDateTime {}

impl DateType for ChronoDateTime {
    type Format = ChronoFormat;

    fn to_raw_date<'a>(&'a self) -> Cow<'a, DateTime<Utc>> {
        Cow::Borrowed(self)
    }

    fn from_raw_date(date: DateTime<Utc>) -> Self {
        date
    }
}

/**
A convenient default date with a configurable format.
*/
pub type DefaultDate<F> = Date<DefaultDateMapping<F>>;

/**
An Elasticsearch `date` type with a required `time` component.

The [format](format/index.html) is provided as a generic parameter.
This struct wraps up a `chrono::DateTime<Utc>` struct, meaning storing time in `Utc` is required.

# Examples

Defining a date using the default format:

```
# use elastic_types::prelude::*;
let date: Date<DefaultDateFormat> = Date::now();
```

Defining a date using a named format:

```
# use elastic_types::prelude::*;
let date = Date::<BasicDateTime>::now();
```

Defining a date using a custom mapping:

```
# use elastic_types::prelude::*;
let date: Date<BasicDateTime, DefaultDateMapping<_>> = Date::now();
```

Accessing the values of a date:

```
# use elastic_types::prelude::*;
let date = Date::<BasicDateTime>::now();

//eg: 2010/04/30 13:56:59.372
println!("{}/{}/{} {}:{}:{}.{}",
    date.year(),
    date.month(),
    date.day(),
    date.hour(),
    date.minute(),
    date.second(),
    date.nanosecond() / 1000000
);
```

# Links

- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/date.html)
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Date<M> {
    value: ChronoDateTime,
    _m: PhantomData<M>,
}

impl<M> Date<M> where M: DateMapping
{
    /**
    Creates a new `Date` from the given `chrono::DateTime<Utc>`.
    
    This function will consume the provided `chrono` date.
    
    # Examples
    
    Create an `Date` from the given `chrono::DateTime`:
    
    ```
    # extern crate elastic_types;
    # extern crate chrono;
    # fn main() {
    use chrono::Utc;
    use elastic_types::date::{ Date, DefaultDateFormat };
    
    //Create a chrono DateTime struct
    let chronoDate = Utc::now();
    
    //Give it to the Date struct
    let esDate: Date<DefaultDateFormat> = Date::new(chronoDate);
    # }
    ```
    */
    pub fn new(date: ChronoDateTime) -> Self {
        Date {
            value: date,
            _m: PhantomData,
        }
    }

    /**
    Creates an `Date` from the given Utc primitives:
    
    ```
    # use elastic_types::prelude::*;
    let esDate: Date<DefaultDateFormat> = Date::build(2015, 5, 14, 16, 45, 8, 886);
    ```
    */
    pub fn build(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, milli: u32) -> Self {
        let ndate = NaiveDate::from_ymd(year, month, day);
        let ntime = NaiveTime::from_hms_milli(hour, minute, second, milli);

        Date {
            value: ChronoDateTime::from_utc(NaiveDateTime::new(ndate, ntime), Utc),
            _m: PhantomData,
        }
    }

    /**
    Gets the current system time.
    
    # Examples
    
    ```
    # use elastic_types::prelude::*;
    let date: Date<DefaultDateFormat> = Date::now();
    ```
    */
    pub fn now() -> Self {
        Date {
            value: Utc::now(),
            _m: PhantomData,
        }
    }

    /**
    Change the format/mapping of this date.
    
    # Examples
    
    ```
    # use elastic_types::prelude::*;
    //Get the current datetime formatted as basic_date_time
    let date: Date<BasicDateTime> = Date::now();
    
    //Change the format to epoch_millis
    let otherdate: Date<EpochMillis> = date.remap();
    ```
    */
    pub fn remap<MInto>(self) -> Date<MInto>
        where MInto: DateMapping
    {
        Date::new(self.value)
    }
}

impl<M> DateFieldType<M> for Date<M>
    where M: DateMapping
{
}

impl<M> DateType for Date<M> 
    where M: DateMapping
{
    type Format = M::Format;

    fn to_raw_date<'a>(&'a self) -> Cow<'a, DateTime<Utc>> {
        Cow::Borrowed(&self.value)
    }

    fn from_raw_date(date: DateTime<Utc>) -> Self {
        Self::from(date)
    }
}

impl_mapping_type!(ChronoDateTime, Date, DateMapping);

impl<M> Default for Date<M>
    where M: DateMapping
{
    fn default() -> Self {
        Date::now()
    }
}

impl<M> Display for Date<M>
    where M: DateMapping
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.format())
    }
}

impl<M> Serialize for Date<M>
    where M: DateMapping
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&self)
    }
}

impl<'de, M> Deserialize<'de> for Date<M>
    where M: DateMapping
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Default)]
        struct DateTimeVisitor<M>
            where M: DateMapping
        {
            _t: PhantomData<M>,
        }

        impl<'de, M> Visitor<'de> for DateTimeVisitor<M>
            where M: DateMapping
        {
            type Value = Date<M>;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter,
                       "a json string or number containing a formatted date")
            }

            fn visit_str<E>(self, v: &str) -> Result<Date<M>, E>
                where E: Error
            {
                let result = Date::parse(v);
                result.map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Date<M>, E>
                where E: Error
            {
                let result = Date::parse(v.to_string().as_ref());
                result.map_err(|err| Error::custom(format!("{}", err)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Date<M>, E>
                where E: Error
            {
                let result = Date::parse(v.to_string().as_ref());
                result.map_err(|err| Error::custom(format!("{}", err)))
            }
        }

        deserializer.deserialize_any(DateTimeVisitor::<M>::default())
    }
}

/// A date expression.
#[derive(Debug, Clone, PartialEq)]
pub struct DateExpr {
    anchor: DateExprAnchor,
    ops: Vec<DateExprOp>
}

impl Display for DateExpr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.anchor.fmt(f)?;

        for op in &self.ops {
            op.fmt(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DateExprAnchor {
    Now,
    Value(String)
}

impl Display for DateExprAnchor {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            DateExprAnchor::Now => "now".fmt(f),
            DateExprAnchor::Value(ref date) => write!(f, "{}||", date),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DateExprOp {
    Add(usize, DateExprOpUnit),
    Sub(usize, DateExprOpUnit),
    Round(DateExprOpUnit)
}

impl Display for DateExprOp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            DateExprOp::Add(size, unit) => write!(f, "+{}{}", size, unit),
            DateExprOp::Sub(size, unit) => write!(f, "-{}{}", size, unit),
            DateExprOp::Round(unit) => write!(f, "/{}", unit)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DateExprOpUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second
}

impl Display for DateExprOpUnit {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let fmtd = match *self {
            DateExprOpUnit::Year => "y",
            DateExprOpUnit::Month => "M",
            DateExprOpUnit::Week => "w",
            DateExprOpUnit::Day => "d",
            DateExprOpUnit::Hour => "h",
            DateExprOpUnit::Minute => "m",
            DateExprOpUnit::Second => "s"
        };

        fmtd.fmt(f)
    }
}

macro_rules! impl_expr_ops {
    ($op:path, $add:ident, $sub:ident, $round:ident) => (
        /// Add to the anchored date.
        pub fn $add(mut self, value: usize) -> Self {
            self.ops.push(DateExprOp::Add(value, $op));
            self
        }

        /// Subtract from the anchored date.
        pub fn $sub(mut self, value: usize) -> Self {
            self.ops.push(DateExprOp::Sub(value, $op));
            self
        }

        /// Round the anchored date.
        pub fn $round(mut self) -> Self {
            self.ops.push(DateExprOp::Round($op));
            self
        }
    )
}

impl DateExpr {
    /// Create a new date expression for `now`.
    /// 
    /// This value is different from `DateTime::now()` because it doesn't calculate the current date from the system clock.
    /// It serialises to the literal `now`, which is interpreted by Elasticsearch when indexing.
    pub fn now() -> Self {
        DateExpr {
            anchor: DateExprAnchor::Now,
            ops: Vec::new(),
        }
    }
    
    /// Create a new date expression from a concrete date value.
    pub fn value<D>(date: D) -> Self where D: DateType {
        DateExpr {
            anchor: DateExprAnchor::Value(date.format().to_string()),
            ops: Vec::new(),
        }
    }

    impl_expr_ops!(DateExprOpUnit::Year, add_years, sub_years, round_year);
    impl_expr_ops!(DateExprOpUnit::Month, add_months, sub_months, round_month);
    impl_expr_ops!(DateExprOpUnit::Week, add_weeks, sub_weeks, round_week);
    impl_expr_ops!(DateExprOpUnit::Day, add_days, sub_days, round_day);
    impl_expr_ops!(DateExprOpUnit::Hour, add_hours, sub_hours, round_hour);
    impl_expr_ops!(DateExprOpUnit::Minute, add_minutes, sub_minutes, round_minute);
    impl_expr_ops!(DateExprOpUnit::Second, add_seconds, sub_seconds, round_second);
}

impl Serialize for DateExpr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(&self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json;
    use chrono;
    use chrono::offset::TimeZone;

    use prelude::*;

    #[derive(ElasticDateFormat, Default, Clone)]
    #[elastic(date_format="yyyy/MM/dd HH:mm:ss", date_format_name="test_date_1")]
    pub struct NamedDateFormat;

    #[derive(ElasticDateFormat, Default, Clone, Copy)]
    #[elastic(date_format="yyyyMMdd")]
    pub struct UnNamedDateFormat;

    #[test]
    fn date_format_uses_name_if_supplied() {
        assert_eq!("test_date_1", NamedDateFormat::name());
    }

    #[test]
    fn date_format_uses_format_if_name_not_supplied() {
        assert_eq!("yyyyMMdd", UnNamedDateFormat::name());
    }

    #[test]
    fn dates_should_use_chrono_format() {
        let dt = chrono::Utc
            .datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S")
            .unwrap();
        let expected = dt.format("%Y/%m/%d %H:%M:%S").to_string();

        let dt = Date::<NamedDateFormat>::new(dt.clone());
        let actual = dt.format();

        assert_eq!(expected, actual);
    }

    #[test]
    fn dates_should_use_es_format() {
        let dt = chrono::Utc
            .datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S")
            .unwrap();
        let expected = "20150513".to_string();

        let dt = Date::<UnNamedDateFormat>::new(dt.clone());
        let actual = dt.format();

        assert_eq!(expected, actual);
    }

    #[test]
    fn can_change_date_mapping() {
        fn takes_epoch_millis(_: Date<EpochMillis>) -> bool {
            true
        }

        let date: Date<BasicDateTime> = Date::now();

        assert!(takes_epoch_millis(date.remap()));
    }

    #[test]
    fn can_build_date_from_chrono() {
        let date: Date<DefaultDateFormat> = Date::new(chrono::Utc
                                                          .datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S")
                                                          .unwrap());

        assert_eq!((2015, 5, 13, 0, 0, 0),
                   (date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second()));
    }

    #[test]
    fn can_build_date_from_prim() {
        let date: Date<DefaultDateFormat> = Date::build(2015, 5, 13, 0, 0, 0, 0);

        assert_eq!((2015, 5, 13, 0, 0, 0),
                   (date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second()));
    }

    #[test]
    fn serialise_elastic_date() {
        let date = Date::<BasicDateTime>::new(chrono::Utc
                                                  .datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S")
                                                  .unwrap());

        let ser = serde_json::to_string(&date).unwrap();

        assert_eq!(r#""20150513T000000.000Z""#, ser);
    }

    #[test]
    fn deserialise_elastic_date() {
        let date: Date<BasicDateTime> = serde_json::from_str(r#""20150513T000000.000Z""#).unwrap();

        assert_eq!((2015, 5, 13), (date.year(), date.month(), date.day()));
    }

    #[test]
    fn serialise_elastic_date_brw() {
        let chrono_date = chrono::Utc
            .datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S")
            .unwrap();

        let date = DateBrw::<BasicDateTime>::new(&chrono_date);

        let ser = serde_json::to_string(&date).unwrap();

        assert_eq!(r#""20150513T000000.000Z""#, ser);
    }

    #[test]
    fn serialise_date_expr_now() {
        let expr = DateExpr::now();

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now""#, ser);
    }

    #[test]
    fn serialise_date_expr_value() {
        let expr = DateExpr::value(Date::<BasicDateTime>::build(2015, 5, 13, 0, 0, 0, 0));

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""20150513T000000.000Z||""#, ser);
    }

    #[test]
    fn serialise_date_expr_value_with_ops() {
        let expr = DateExpr::value(Date::<BasicDateTime>::build(2015, 5, 13, 0, 0, 0, 0))
            .add_days(2)
            .round_week();

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""20150513T000000.000Z||+2d/w""#, ser);
    }

    #[test]
    fn serialise_date_expr_add() {
        let expr = DateExpr::now()
            .add_years(1)
            .add_months(2)
            .add_weeks(3)
            .add_days(4)
            .add_hours(5)
            .add_minutes(6)
            .add_seconds(7);

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now+1y+2M+3w+4d+5h+6m+7s""#, ser);
    }

    #[test]
    fn serialise_date_expr_sub() {
        let expr = DateExpr::now()
            .sub_years(1)
            .sub_months(2)
            .sub_weeks(3)
            .sub_days(4)
            .sub_hours(5)
            .sub_minutes(6)
            .sub_seconds(7);

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now-1y-2M-3w-4d-5h-6m-7s""#, ser);
    }

    #[test]
    fn serialise_date_expr_round() {
        let expr = DateExpr::now()
            .round_year()
            .round_month()
            .round_week()
            .round_day()
            .round_hour()
            .round_minute()
            .round_second();

        let ser = serde_json::to_string(&expr).unwrap();

        assert_eq!(r#""now/y/M/w/d/h/m/s""#, ser);
    }
}
