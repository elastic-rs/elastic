use std::slice::Iter;
use chrono;
use chrono::{ DateTime, NaiveDateTime, UTC, Timelike };
use chrono::format::Item;
use super::{ Format, ParseError };

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

/// Format for `epoch_millis`.
/// 
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(Clone)]
pub struct EpochMillis;
impl Format for EpochMillis {
	fn fmt<'a>() -> Vec<Item<'a>>{
		Vec::new()
	}
	fn name() -> &'static str {
		"epoch_millis"
	}

	fn parse<'a>(date: &str) -> Result<DateTime<UTC>, ParseError> {
		//TODO: Determine if this is correct
		let (secs, msecs) = match date.chars().next().unwrap() {
			'-' => (date[0..11].parse::<i64>(), date[11..14].parse::<u32>()),
			_ => (date[0..10].parse::<i64>(), date[10..13].parse::<u32>())
		};
		
		let _secs = secs.unwrap();
		let _msecs = msecs.unwrap();

		Ok(DateTime::from_utc(NaiveDateTime::from_num_seconds_from_unix_epoch(_secs, _msecs * 1000000), UTC))
	}

	fn format<'a>(date: &DateTime<UTC>) -> String {
		let mut fmtd = String::with_capacity(13);

		let sec = date.timestamp().to_string();
		fmtd.push_str(&sec);

		let msec = (date.nanosecond() / 1000000).to_string();
		fmtd.push_str(&msec);

		fmtd
	}
}