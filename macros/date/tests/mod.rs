#![cfg_attr(feature = "nightly", feature(plugin, custom_derive))]
#![cfg_attr(feature = "nightly", plugin(elastic_date_macros))]

#[cfg_attr(feature = "nightly", allow(plugin_as_library))]
#[macro_use]
extern crate elastic_date_macros;

extern crate chrono;

#[test]
fn can_generate_date_formats() {
	let _ = date_fmt!("yyyyMMddTHHmmss.SSSZ");
}

#[test]
fn can_parse_es_date_format_to_chrono() {
	let parse_result = elastic_date_macros::to_tokens("yyyyMMddTHHmmss.SSSZ");
	let fmt = elastic_date_macros::to_chrono_format(parse_result);

	assert_eq!("%Y%m%dT%H%M%S%.3fZ".to_string(), fmt);
}

#[test]
fn can_parse_chrono_date_format_to_es() {
	let parse_result = elastic_date_macros::to_tokens("%Y%m%dT%H%M%S%.3fZ");
	let fmt = elastic_date_macros::to_es_format(parse_result);

	assert_eq!("yyyyMMddTHHmmss.SSSZ".to_string(), fmt);
}

#[test]
fn can_get_es_format_from_tokens() {
	let parse_result = elastic_date_macros::to_tokens("yyyyMMdd");
	let fmt = elastic_date_macros::to_es_format(parse_result);

	assert_eq!("yyyyMMdd".to_string(), fmt);
}

#[test]
fn edgecase_can_parse_period_as_literal() {
	let parse_result = elastic_date_macros::to_tokens("yyyy.MM.dd");
	let fmt = elastic_date_macros::to_es_format(parse_result);

	assert_eq!("yyyy.MM.dd".to_string(), fmt);
}

#[test]
fn edgecase_can_parse_millis_after_literal() {
	let parse_result = elastic_date_macros::to_tokens("T.SSS");
	let fmt = elastic_date_macros::to_es_format(parse_result);

	assert_eq!("T.SSS".to_string(), fmt);
}
