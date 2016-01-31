use std::slice::Iter;
use chrono;
use chrono::format::Item;
use super::{ Format };

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