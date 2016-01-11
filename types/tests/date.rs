#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate elastic_types;

use chrono::format::Parsed;
use chrono::offset::TimeZone;
use elastic_types::date::{ DateTime, Datelike, Timelike, Format };
use elastic_types::date::format::{ BasicDateTime, BASIC_DATE_TIME, BasicDateTimeNoMillis };
use elastic_types::date::format::parse;

//MyType -> MyTypeFmtd
//yyyy/mm/dd -> %Y/%m/%dT%H:%M:%SZ
//2015/05/13 -> 2015/05/13T00:00:00Z

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

const MYTYPE_DATE_FMT_1: &'static str = "%Y/%m/%d %H:%M:%S";
const MYTYPE_DATE_FMT_2: &'static str = "%d/%m/%Y %H:%M:%S";

#[allow(non_camel_case_types)]
struct MyType_date_fmt;
impl Format for MyType_date_fmt {
	fn fmt() -> Vec<&'static str> {
		vec![MYTYPE_DATE_FMT_1, MYTYPE_DATE_FMT_2]
	}
	fn es_fmt() -> &'static str {
		"yyyy/MM/dd HH:mm:ss||dd/MM/yyyy HH:mm:ss"
	}
}

#[test]
fn dates_with_format_specified_should_use_first_provided() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = _dt.format(MYTYPE_DATE_FMT_1).to_string();

	let dt = DateTime::<MyType_date_fmt>::new(_dt.clone());
	let actual = dt.to_string();

	assert_eq!(expected, actual);
}

#[test]
fn dates_with_multi_formats_should_use_first_successful() {
	let dt = DateTime::<MyType_date_fmt>::parse("13/05/2015 00:00:00").unwrap();

	assert_eq!(
		(2015i32, 5u32, 13u32), 
		(
			dt.value.year(),
			dt.value.month(),
			dt.value.day()
		)
	);
}

#[test]
fn dates_with_multi_formats_should_return_all_errors_if_none_successful() {
	let dt = DateTime::<MyType_date_fmt>::parse("this is not a date");

	assert_eq!(
		"%Y/%m/%d %H:%M:%S : input contains invalid characters, %d/%m/%Y %H:%M:%S : input contains invalid characters".to_string(), 
		format!("{}", dt.err().unwrap())
	);
}

#[test]
fn dates_use_specified_format_when_serialising() {
	let my_type = MyType {
		date: DateTime::new(
			chrono::UTC.datetime_from_str(
				"13/05/2015 00:00:00", MYTYPE_DATE_FMT_2
			).unwrap()
		)
	};

	let ser = serde_json::to_string(&my_type).unwrap();

	assert_eq!(r#"{"date":"20150513T000000.000Z"}"#, ser);
}

#[test]
fn dates_use_specified_format_when_deserialising() {
	let my_type: MyType = serde_json::from_str(r#"{"date":"20150513T000000.000Z"}"#).unwrap();

	assert_eq!((2015, 5, 13), (my_type.date.value.year(), my_type.date.value.month(), my_type.date.value.day()));
}

#[test]
fn can_parse_es_date_format_to_chrono() {
	let parse_result = parse::to_tokens("yyyyMMddTHHmmss.SSSZ");
	let fmt = parse::to_chrono_format(parse_result);

	assert_eq!("%Y%m%dT%H%M%S%.3fZ".to_string(), fmt);
}

#[test]
fn can_parse_dates_with_no_time() {
	let date = DateTime::<BasicDateTimeNoMillis>::parse("20150310T000000Z").unwrap();

	assert_eq!((0, 0), (date.value.hour(), date.value.minute()));
}

#[test]
fn can_parse_chrono_date_format_to_es() {
	let parse_result = parse::to_tokens("%Y%m%dT%H%M%S%.3fZ");
	let fmt = parse::to_es_format(parse_result);

	assert_eq!("yyyyMMddTHHmmss.SSSZ".to_string(), fmt);
}

#[test]
fn can_get_es_format_from_tokens() {
	let parse_result = parse::to_tokens("yyyyMMdd");
	let fmt = parse::to_es_format(parse_result);

	assert_eq!("yyyyMMdd".to_string(), fmt);
}

#[test]
fn edgecase_can_parse_period_as_literal() {
	let parse_result = parse::to_tokens("yyyy.MM.dd");
	let fmt = parse::to_es_format(parse_result);

	assert_eq!("yyyy.MM.dd".to_string(), fmt);
}

#[test]
fn edgecase_can_parse_millis_after_literal() {
	let parse_result = parse::to_tokens("T.SSS");
	let fmt = parse::to_es_format(parse_result);

	assert_eq!("T.SSS".to_string(), fmt);
}