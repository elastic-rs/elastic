use super::{
    DateFormat,
    DateValue,
    FormattedDate,
    ParseError,
};
use chrono::{
    DateTime,
    NaiveDateTime,
    Timelike,
    Utc,
};
use std::error::Error;

use elastic_derive::ElasticDateFormat;

/** The default `date` format (`BasicDateTime`). */
pub type DefaultDateFormat = BasicDateTime;

/** Format for default `chrono::DateTime`. */
#[derive(ElasticDateFormat, PartialEq, Debug, Default, Clone, Copy)]
#[elastic(crate_root = "crate::types")]
#[elastic(date_format = "yyyy-MM-dd'T'HH:mm:ssZ")]
pub struct ChronoFormat;

/**
Format for `basic_date_time_no_millis`.

# Links
- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
*/
#[derive(ElasticDateFormat, PartialEq, Debug, Default, Clone, Copy)]
#[elastic(crate_root = "crate::types")]
#[elastic(
    date_format = "yyyyMMdd'T'HHmmssZ",
    date_format_name = "basic_date_time_no_millis"
)]
pub struct BasicDateTimeNoMillis;

/**
Format for `basic_date_time`.

# Links
- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
*/
#[derive(ElasticDateFormat, PartialEq, Debug, Default, Clone, Copy)]
#[elastic(crate_root = "crate::types")]
#[elastic(
    date_format = "yyyyMMdd'T'HHmmss.SSSZ",
    date_format_name = "basic_date_time"
)]
pub struct BasicDateTime;

/**
Format for `epoch_millis`.

Takes up to a 13 digit string of millis since the epoch and converts to a `DateTime`.
This is an efficient formatter, so is a good choice for storing timestamps.

# Links
- [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
*/
#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct EpochMillis;

impl DateFormat for EpochMillis {
    fn name() -> &'static str {
        "epoch_millis"
    }

    fn parse(date: &str) -> Result<DateValue, ParseError> {
        let millis = date
            .parse::<i64>()
            .map_err(|e| e.description().to_string())?;

        let (s, m) = {
            // For positive timestamps:
            // Extract the millis straight off the timestamp (how many millis since the last second?)
            if millis >= 0 {
                ((millis / 1000), (millis % 1000))
            }
            // For negative timestamps:
            // The millis need to be inverted (how many millis before the next second?)
            else {
                ((millis / 1000) - 1, (1000 + (millis % 1000)))
            }
        };

        let date = DateTime::from_utc(NaiveDateTime::from_timestamp(s, m as u32 * 1000000), Utc);

        Ok(date.into())
    }

    fn format<'a>(date: &'a DateValue) -> FormattedDate<'a> {
        let msec = (date.timestamp() * 1000) + (date.nanosecond() as i64 / 1000000);
        msec.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::*,
        types::date::{
            format,
            parse,
            ParseError,
        },
    };
    use chrono::{
        DateTime,
        Utc,
    };

    #[test]
    fn chrono() {
        let date = parse::<DefaultDateMapping<ChronoFormat>>("2015-07-03T14:55:02Z").unwrap();

        assert_eq!(
            (2015i32, 7u32, 3u32, 14u32, 55u32, 2u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second()
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("2015-07-03T14:55:02Z", &fmtd);
    }

    #[test]
    fn chrono_name() {
        assert_eq!("yyyy-MM-dd'T'HH:mm:ssZ", ChronoFormat::name());
    }

    #[test]
    fn basic_datetime_no_millis() {
        let date = parse::<DefaultDateMapping<BasicDateTimeNoMillis>>("20150703T145502Z").unwrap();

        assert_eq!(
            (2015i32, 7u32, 3u32, 14u32, 55u32, 2u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second()
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("20150703T145502Z", &fmtd);
    }

    #[test]
    fn basic_datetime_no_millis_name() {
        assert_eq!("basic_date_time_no_millis", BasicDateTimeNoMillis::name());
    }

    #[test]
    fn basic_date_time() {
        let date = parse::<DefaultDateMapping<BasicDateTime>>("20150703T145502.478Z").unwrap();

        assert_eq!(
            (2015i32, 7u32, 3u32, 14u32, 55u32, 2u32, 478u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("20150703T145502.478Z", &fmtd);
    }

    #[test]
    fn basic_date_time_name() {
        assert_eq!("basic_date_time", BasicDateTime::name());
    }

    #[test]
    fn epoch_millis() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("1435935302478").unwrap();

        assert_eq!(
            (2015i32, 7u32, 3u32, 14u32, 55u32, 2u32, 478u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("1435935302478", &fmtd);
    }

    #[test]
    fn epoch_millis_name() {
        assert_eq!("epoch_millis", EpochMillis::name());
    }

    #[test]
    fn epoch_millis_no_millis() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("1435935302000").unwrap();

        assert_eq!(
            (2015i32, 7u32, 3u32, 14u32, 55u32, 2u32, 0u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("1435935302000", &fmtd);
    }

    #[test]
    fn epoch_millis_minus() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("-8031171898478").unwrap();

        assert_eq!(
            (1715i32, 7u32, 3u32, 14u32, 55u32, 1u32, 522u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("-8031171898478", &fmtd);
    }

    #[test]
    fn epoch_millis_minus_no_millis() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("-8031171898000").unwrap();

        assert_eq!(
            (1715i32, 7u32, 3u32, 14u32, 55u32, 1u32, 1000u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("-8031171898000", &fmtd);
    }

    #[test]
    fn epoch_millis_very_short() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("100").unwrap();

        assert_eq!(
            (1970i32, 1u32, 1u32, 0u32, 0u32, 0u32, 100u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("100", &fmtd);
    }

    #[test]
    fn epoch_millis_short() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("5100").unwrap();

        assert_eq!(
            (1970i32, 1u32, 1u32, 0u32, 0u32, 5u32, 100u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("5100", &fmtd);
    }

    #[test]
    fn epoch_millis_very_short_minus() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("-100").unwrap();

        assert_eq!(
            (1969i32, 12u32, 31u32, 23u32, 59u32, 59u32, 900u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("-100", &fmtd);
    }

    #[test]
    fn epoch_millis_short_minus() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("-5100").unwrap();

        assert_eq!(
            (1969i32, 12u32, 31u32, 23u32, 59u32, 54u32, 900u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("-5100", &fmtd);
    }

    #[test]
    fn epoch_millis_zero() {
        let date = parse::<DefaultDateMapping<EpochMillis>>("0").unwrap();

        assert_eq!(
            (1970i32, 1u32, 1u32, 0u32, 0u32, 0u32, 0u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second(),
                date.nanosecond() / 1000000
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("0", &fmtd);
    }

    #[test]
    fn custom_format() {
        #[derive(Default)]
        struct MyCustomFormat;
        impl DateFormat for MyCustomFormat {
            fn name() -> &'static str {
                "yyyy-MM-dd'T'HH:mm:ssZ"
            }

            fn format<'a>(date: &'a DateValue) -> FormattedDate<'a> {
                date.to_rfc3339().into()
            }

            fn parse(date: &str) -> Result<DateValue, ParseError> {
                let date = DateTime::parse_from_rfc3339(date).map_err(|e| ParseError::from(e))?;

                Ok(DateTime::from_utc(date.naive_local(), Utc).into())
            }
        }

        let date =
            parse::<DefaultDateMapping<MyCustomFormat>>("2015-07-03T14:55:02+00:00").unwrap();

        assert_eq!(
            (2015i32, 7u32, 3u32, 14u32, 55u32, 2u32),
            (
                date.year(),
                date.month(),
                date.day(),
                date.hour(),
                date.minute(),
                date.second()
            )
        );

        let fmtd = format(&date).to_string();
        assert_eq!("2015-07-03T14:55:02+00:00", &fmtd);
    }
}
