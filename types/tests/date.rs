#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate elastic_types;

use chrono::Datelike;
use chrono::offset::TimeZone;
use serde::{ Serialize, Deserialize };
use elastic_types::date::*;

//MyType -> MyTypeFmtd
//yyyy/mm/dd -> %Y/%m/%d %H:%M:%S
//2015/05/13 -> 2015/05/13 00:00:00

//Unexpanded
#[derive(Default, Serialize, Deserialize)]
struct MyType {
	//#[es_date_format("yyyy/mm/dd", "dd/mm/yyyy")]
	pub date: DateTime
}

//Expanded
#[derive(Default, Serialize, Deserialize)]
struct MyTypeFmtd {
	pub date: DateTime<MyType_date_fmt>
}

#[allow(non_camel_case_types)]
struct MyType_date_fmt;
impl Format for MyType_date_fmt {
	fn fmt() -> Vec<&'static str> {
		vec!["%Y/%m/%d %H:%M:%S", "%d/%m/%Y %H:%M:%S"]
	}
}

#[test]
fn dates_without_format_specified_use_default() {
	let _dt = chrono::UTC::now();
	let expected = _dt.format("%Y/%m/%d %H:%M:%S").to_string();

	let dt = DateTime::<MyType_date_fmt>::new(_dt.clone());
	let actual = dt.to_string();

	assert_eq!(expected, actual);
}

#[test]
fn dates_with_format_specified_should_use_first_provided() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = _dt.format("%Y/%m/%d %H:%M:%S").to_string();

	let dt = DateTime::<MyType_date_fmt>::new(_dt.clone());
	let actual = dt.to_string();

	assert_eq!(expected, actual);
}

#[test]
fn dates_with_multi_formats_should_use_first_successful() {
	//Match %d/%m/%Y
	let dt = DateTime::<MyType_date_fmt>::parse("13/05/2015 00:00:00");

	assert_eq!(2015, dt.value.year());
	assert_eq!(5, dt.value.month());
	assert_eq!(13, dt.value.day());
}

#[test]
fn dates_use_specified_format_when_serialising() {
	let my_type = MyType {
		date: DateTime::new(
			chrono::UTC.datetime_from_str(
				"13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S"
			).unwrap()
		)
	};

	let ser = serde_json::to_string(&my_type).unwrap();

	assert_eq!(r#"{"date":"2015-05-13T00:00:00Z"}"#, ser);
}

#[test]
fn dates_use_specified_format_when_deserialising() {
	let my_type: MyType = serde_json::from_str(r#"{"date":"2015-05-13T00:00:00Z"}"#).unwrap();

	assert_eq!(2015, my_type.date.value.year());
	assert_eq!(5, my_type.date.value.month());
	assert_eq!(13, my_type.date.value.day());
}