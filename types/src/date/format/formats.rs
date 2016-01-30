use chrono;
use chrono::format::Item;
use super::{ Format };

macro_rules! own {
    ($items:ident) => {
    	$items.iter().map(|t| t.clone()).collect()
    }
}

pub const BASIC_DATE_TIME_NO_MILLIS: &'static str = "%Y%m%dT%H%M%SZ";
const BASIC_DATE_TIME_NO_MILLIS_ITEMS: [Item<'static>; 8] = date_fmt!("%Y%m%dT%H%M%SZ");
#[derive(Clone)]
pub struct BasicDateTimeNoMillis;
impl Format for BasicDateTimeNoMillis {
	fn fmt<'a>() -> Vec<Vec<Item<'a>>> {
		vec![own!(BASIC_DATE_TIME_NO_MILLIS_ITEMS)]
	}
	fn fmt_str() -> &'static str {
		BASIC_DATE_TIME_NO_MILLIS
	}
	fn name() -> &'static str {
		"basic_date_time_no_millis"
	}
}

pub const BASIC_DATE_TIME: &'static str = "%Y%m%dT%H%M%S%.3fZ";
const BASIC_DATE_TIME_ITEMS: [Item<'static>; 9] = date_fmt!("%Y%m%dT%H%M%S%.3fZ");
#[derive(Clone)]
pub struct BasicDateTime;
impl Format for BasicDateTime {
	fn fmt<'a>() -> Vec<Vec<Item<'a>>> {
		vec![own!(BASIC_DATE_TIME_ITEMS)]
	}
	fn fmt_str() -> &'static str {
		BASIC_DATE_TIME
	}
	fn name() -> &'static str {
		"basic_date_time"
	}
}