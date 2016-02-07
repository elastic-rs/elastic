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
/// Takes up to a 13 digit string of millis since the epoch and converts to a `DateTime`.
/// This is an efficient formatter, so is a good choice for storing timestamps.
/// It's not recommended to use for historical dates, especially those close to or before 01/01/1970,
/// which may produce imprecise results.
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
		let (secs, msecs) = match (date.len(), date.chars().next().unwrap()) {
			//4 or less chars with minus sign. This is a special case between 31/12/69 23:59:99.900 and 01/01/70 00:00:00.000
			(n, '-') if n <= 4 => {
				let (s, m) = (Ok(-1i64), date[1..].parse::<u32>());
				(s, Ok(1000 - m.unwrap()))
			},
			//11 or less chars with minus sign
			(n, '-') if n <= 11 => (date[0..n-3].parse::<i64>(), date[n-3..].parse::<u32>()),
			//minus sign
			(14, '-') => (date[0..11].parse::<i64>(), date[11..14].parse::<u32>()),
			//3 or less chars
			(n, _) if n <= 3 => (Ok(0i64), date.parse::<u32>()),
			//10 or less chars
			(n, _) if n <= 10 => (date[0..n-3].parse::<i64>(), date[n-3..].parse::<u32>()),
			//Standard
			(_, _) => (date[0..10].parse::<i64>(), date[10..13].parse::<u32>())
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