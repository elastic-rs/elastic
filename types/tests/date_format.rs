extern crate chrono;
extern crate elastic_types;

use elastic_types::date::{ DateTime, Datelike, Timelike };
use elastic_types::date::format::*;

#[test]
fn basic_datetime_no_millis() {
	let date = DateTime::<BasicDateTimeNoMillis>::parse("20150703T145502Z").unwrap();

	assert_eq!(
		(2015i32, 7u32, 3u32, 14u32, 55u32, 2u32),
		(
			date.value.year(), 
			date.value.month(), 
			date.value.day(),
			date.value.hour(), 
			date.value.minute(), 
			date.value.second()
		)
	)
}

#[test]
fn basic_date_time() {
	let date = DateTime::<BasicDateTime>::parse("20150703T145502.478Z").unwrap();

	assert_eq!(
		(2015i32, 7u32, 3u32, 14u32, 55u32, 2u32, 478u32),
		(
			date.value.year(), 
			date.value.month(), 
			date.value.day(),
			date.value.hour(), 
			date.value.minute(), 
			date.value.second(),
			date.value.nanosecond() / 1000000
		)
	)
}