#![feature(plugin)]
#![plugin(elastic_types_codegen)]
extern crate chrono;
extern crate elastic_types_codegen;

use elastic_types_codegen::date;

#[test]
fn can_generate_date_formats() {
	let _ = date_fmt!("yyyyMMddTHHmmss.SSSZ");
}

#[test]
fn can_parse_es_date_format_to_chrono() {
	let parse_result = date::to_tokens("yyyyMMddTHHmmss.SSSZ");
	let fmt = date::to_chrono_format(parse_result);

	assert_eq!("%Y%m%dT%H%M%S%.3fZ".to_string(), fmt);
}

#[test]
fn can_parse_chrono_date_format_to_es() {
	let parse_result = date::to_tokens("%Y%m%dT%H%M%S%.3fZ");
	let fmt = date::to_es_format(parse_result);

	assert_eq!("yyyyMMddTHHmmss.SSSZ".to_string(), fmt);
}

#[test]
fn can_get_es_format_from_tokens() {
	let parse_result = date::to_tokens("yyyyMMdd");
	let fmt = date::to_es_format(parse_result);

	assert_eq!("yyyyMMdd".to_string(), fmt);
}

#[test]
fn edgecase_can_parse_period_as_literal() {
	let parse_result = date::to_tokens("yyyy.MM.dd");
	let fmt = date::to_es_format(parse_result);

	assert_eq!("yyyy.MM.dd".to_string(), fmt);
}

#[test]
fn edgecase_can_parse_millis_after_literal() {
	let parse_result = date::to_tokens("T.SSS");
	let fmt = date::to_es_format(parse_result);

	assert_eq!("T.SSS".to_string(), fmt);
}