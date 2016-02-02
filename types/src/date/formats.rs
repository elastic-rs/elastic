use std::slice::Iter;
use chrono;
use chrono::format::Item;
use super::{ Format };

/// Format for `basic_date_time_no_millis`.
/// 
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(Clone)]
pub struct BasicDateTimeNoMillis;
impl Format for BasicDateTimeNoMillis {
	fn fmt<'a>() -> Vec<Item<'a>> {
		date_fmt!("%Y%m%dT%H%M%SZ")
			.iter()
			.map(|t| *t)
			.collect()
	}
	fn name() -> &'static str {
		"basic_date_time_no_millis"
	}
}

/// Format for `basic_date_time`.
/// 
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(Clone)]
pub struct BasicDateTime;
impl Format for BasicDateTime {
	fn fmt<'a>() -> Vec<Item<'a>>{
		date_fmt!("%Y%m%dT%H%M%S%.3fZ")
			.iter()
			.map(|t| *t)
			.collect()
	}
	fn name() -> &'static str {
		"basic_date_time"
	}
}