#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros)]
#![plugin(elastic_macros)]

extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate elastic_types;

use chrono::format::{ Parsed, Item };
use chrono::offset::TimeZone;
use elastic_types::mapping::*;
use elastic_types::date::*;

#[derive(Default, Serialize, Deserialize)]
struct MyType {
	pub date: DateTime
}

#[derive(Default, Serialize, Deserialize)]
struct MyTypeFmtd {
	pub date: DateTime<TestDateFormat1>
}

const MYTYPE_DATE_FMT_1: &'static str = "%Y/%m/%d %H:%M:%S";
const MYTYPE_DATE_FMT_2: &'static str = "%d/%m/%Y %H:%M:%S";

//A date format based on a chrono format string
#[allow(non_camel_case_types)]
pub struct TestDateFormat1;
impl Format for TestDateFormat1 {
	fn fmt<'a>() -> Vec<Item<'a>> {
		date_fmt!("%Y/%m/%d %H:%M:%S")
			.iter()
			.map(|t| *t)
			.collect()
	}
	fn name() -> &'static str {
		"test_date_1"
	}
}

//A date format based on an elasticsearch formart string
#[allow(non_camel_case_types)]
pub struct TestDateFormat2;
impl Format for TestDateFormat2 {
	fn fmt<'a>() -> Vec<Item<'a>> {
		date_fmt!("yyyyMMdd")
			.iter()
			.map(|t| *t)
			.collect()
	}
	fn name() -> &'static str {
		"test_date_2"
	}
}

//A custom date mapping
struct MyDateMapping;
impl ElasticMapping for MyDateMapping {
	fn get_boost() -> Option<f32> {
		Some(1.01)
	}
}
impl ElasticDateMapping<TestDateFormat2> for MyDateMapping {
	fn get_null_value() -> Option<NullValue> {
		Some(NullValue::Default("20150701"))
	}
}

impl serde::Serialize for MyDateMapping {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_struct("mapping", ElasticDateMappingVisitor::<TestDateFormat2, MyDateMapping>::default())
    }
}

#[test]
fn dates_should_use_chrono_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = _dt.format(MYTYPE_DATE_FMT_1).to_string();

	let dt = DateTime::<TestDateFormat1>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
}

#[test]
fn dates_should_use_es_format() {
	let _dt = chrono::UTC.datetime_from_str("13/05/2015 00:00:00", "%d/%m/%Y %H:%M:%S").unwrap();
	let expected = "20150513".to_string();

	let dt = DateTime::<TestDateFormat2>::new(_dt.clone());
	let actual = dt.format();

	assert_eq!(expected, actual);
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

	assert_eq!((2015, 5, 13), (my_type.date.year(), my_type.date.month(), my_type.date.day()));
}

#[test]
fn can_cast_chrono_date_into_elastic_date() {
	fn takes_es_date<T: Into<DateTime<BasicDateTime>>>(_: T) {
		
	}

	let date = chrono::UTC.datetime_from_str(
		"13/05/2015 00:00:00", MYTYPE_DATE_FMT_2
	).unwrap();

	takes_es_date(date);
}

#[test]
fn mapping_serialises_overriden_params() {
	let mapping = MyDateMapping;
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"boost":1.01,"format":"test_date_2","null_value":"20150701"}"#, ser);
}

#[test]
fn default_mapping_serialises_only_format() {
	let mapping = DefaultDateMapping::<BasicDateTime>::new();
	let ser = serde_json::to_string(&mapping).unwrap();

	assert_eq!(r#"{"format":"basic_date_time"}"#, ser);
}