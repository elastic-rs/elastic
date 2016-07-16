#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;
pub mod formats;

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate elastic_types;

use chrono::format::Item;
use chrono::offset::TimeZone;

use elastic_types::date::prelude::*;

#[derive(Default, Serialize, Deserialize)]
struct MyType {
	pub date: ElasticDate<DefaultDateFormat>
}

#[derive(Default, Serialize, Deserialize)]
struct MyTypeFmtd {
	pub date: ElasticDate<TestDateFormat1>
}

const MYTYPE_DATE_FMT_1: &'static str = "%Y/%m/%d %H:%M:%S";
const MYTYPE_DATE_FMT_2: &'static str = "%d/%m/%Y %H:%M:%S";

//A date format based on a chrono format string
#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy)]
pub struct TestDateFormat1;
impl DateFormat for TestDateFormat1 {
	fn fmt<'a>() -> Vec<Item<'a>> {
		date_fmt!("%Y/%m/%d %H:%M:%S")
			.iter()
			.cloned()
			.collect()
	}
	fn name() -> &'static str {
		"test_date_1"
	}
}

//A date format based on an elasticsearch formart string
#[allow(non_camel_case_types)]
#[derive(Default, Clone, Copy)]
pub struct TestDateFormat2;
impl DateFormat for TestDateFormat2 {
	fn fmt<'a>() -> Vec<Item<'a>> {
		date_fmt!("yyyyMMdd")
			.iter()
			.cloned()
			.collect()
	}
	fn name() -> &'static str {
		"test_date_2"
	}
}

#[test]
fn dates_should_use_chrono_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = _dt.format(MYTYPE_DATE_FMT_1).to_string();

	let dt = ElasticDate::<TestDateFormat1>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
}

#[test]
fn dates_should_use_es_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = "20150513".to_string();

	let dt = ElasticDate::<TestDateFormat2>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
}

#[test]
fn can_change_date_mapping() {
	fn takes_epoch_millis(_: ElasticDate<EpochMillis>) -> bool {
		true
	}

	let date: ElasticDate<BasicDateTime> = ElasticDate::now();

	assert!(takes_epoch_millis(date.remap()));
}

#[test]
fn can_build_date_from_chrono() {
	let date: ElasticDate<DefaultDateFormat> = ElasticDate::new(
		chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap()
	);

	assert_eq!((2015, 5, 13, 0, 0, 0), (
		date.year(),
		date.month(),
		date.day(),
		date.hour(),
		date.minute(),
		date.second()
	));
}

#[test]
fn can_build_date_from_prim() {
	let date: ElasticDate<DefaultDateFormat> = ElasticDate::build(
		2015, 5, 13, 0, 0, 0, 0
	);

	assert_eq!((2015, 5, 13, 0, 0, 0), (
		date.year(),
		date.month(),
		date.day(),
		date.hour(),
		date.minute(),
		date.second()
	));
}

#[test]
fn serialise_elastic_date() {
	let date = ElasticDate::<BasicDateTime>::new(
		chrono::UTC.datetime_from_str(
			"13/05/2015 00:00:00", MYTYPE_DATE_FMT_2
		).unwrap()
	);

	let ser = serde_json::to_string(&date).unwrap();

	assert_eq!(r#""20150513T000000.000Z""#, ser);
}

#[test]
fn deserialise_elastic_date() {
	let date: ElasticDate<BasicDateTime> = serde_json::from_str(r#""20150513T000000.000Z""#).unwrap();

	assert_eq!((2015, 5, 13), (
		date.year(),
		date.month(),
		date.day()
	));
}

#[test]
fn serialise_elastic_date_brw() {
	let chrono_date = chrono::UTC.datetime_from_str(
		"13/05/2015 00:00:00", MYTYPE_DATE_FMT_2
	).unwrap();

	let date = ElasticDateBrw::<BasicDateTime>::new(&chrono_date);

	let ser = serde_json::to_string(&date).unwrap();

	assert_eq!(r#""20150513T000000.000Z""#, ser);
}
