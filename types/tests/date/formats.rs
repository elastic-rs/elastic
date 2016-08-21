pub extern crate chrono;
extern crate elastic_types;

use chrono::{ DateTime, UTC };
use elastic_types::date::prelude::*;

#[test]
fn chrono() {
	let date = ElasticDate::<ChronoFormat>::parse("2015-07-03T14:55:02Z").unwrap();

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

	let fmtd = date.format();
	assert_eq!("2015-07-03T14:55:02Z", &fmtd);
}

#[test]
fn basic_datetime_no_millis() {
	let date = ElasticDate::<BasicDateTimeNoMillis>::parse("20150703T145502Z").unwrap();

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

	let fmtd = date.format();
	assert_eq!("20150703T145502Z", &fmtd);
}

#[test]
fn basic_date_time() {
	let date = ElasticDate::<BasicDateTime>::parse("20150703T145502.478Z").unwrap();

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

	let fmtd = date.format();
	assert_eq!("20150703T145502.478Z", &fmtd);
}

#[test]
fn epoch_millis() {
	let date = ElasticDate::<EpochMillis>::parse("1435935302478").unwrap();

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

	let fmtd = date.format();
	assert_eq!("1435935302478", &fmtd);
}

#[test]
fn epoch_millis_no_millis() {
	let date = ElasticDate::<EpochMillis>::parse("1435935302000").unwrap();

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

	let fmtd = date.format();
	assert_eq!("1435935302000", &fmtd);
}

#[test]
fn epoch_millis_minus() {
	let date = ElasticDate::<EpochMillis>::parse("-8031171898478").unwrap();

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

	let fmtd = date.format();
	assert_eq!("-8031171898478", &fmtd);
}

#[test]
fn epoch_millis_minus_no_millis() {
	let date = ElasticDate::<EpochMillis>::parse("-8031171898000").unwrap();

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

	let fmtd = date.format();
	assert_eq!("-8031171898000", &fmtd);
}

#[test]
fn epoch_millis_very_short() {
	let date = ElasticDate::<EpochMillis>::parse("100").unwrap();

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

	let fmtd = date.format();
	assert_eq!("100", &fmtd);
}

#[test]
fn epoch_millis_short() {
	let date = ElasticDate::<EpochMillis>::parse("5100").unwrap();

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

	let fmtd = date.format();
	assert_eq!("5100", &fmtd);
}

#[test]
fn epoch_millis_very_short_minus() {
	let date = ElasticDate::<EpochMillis>::parse("-100").unwrap();

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

	let fmtd = date.format();
	assert_eq!("-100", &fmtd);
}

#[test]
fn epoch_millis_short_minus() {
	let date = ElasticDate::<EpochMillis>::parse("-5100").unwrap();

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

	let fmtd = date.format();
	assert_eq!("-5100", &fmtd);
}

#[test]
fn custom_format() {
	struct MyCustomFormat;
	impl CustomDateFormat for MyCustomFormat {
		fn name() -> &'static str { "yyyy-MM-dd'T'HH:mm:ssZ" }
	
		fn format(date: &DateTime<UTC>) -> String {
			date.to_rfc3339()
		}
		
		fn parse(date: &str) -> Result<DateTime<UTC>, ParseError> {
			let date = try!(DateTime::parse_from_rfc3339(date).map_err(|e| ParseError::from(e)));

			Ok(DateTime::from_utc(date.naive_local(), UTC))
		}
	}

	let date = ElasticDate::<MyCustomFormat>::parse("2015-07-03T14:55:02+00:00").unwrap();

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

	let fmtd = date.format();
	assert_eq!("2015-07-03T14:55:02+00:00", &fmtd);
}