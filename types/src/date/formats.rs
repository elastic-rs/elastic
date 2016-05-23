use chrono;
use chrono::{ DateTime, NaiveDateTime, UTC, Timelike };
use chrono::format::Item;
use std::error::Error;
use super::{ DateFormat, ParseError };

/// Format for `basic_date_time_no_millis`.
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(Debug, Default, Clone, Copy)]
pub struct BasicDateTimeNoMillis;
impl_date_fmt!(BasicDateTimeNoMillis, "%Y%m%dT%H%M%SZ", "basic_date_time_no_millis");

/// Format for `basic_date_time`.
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
#[derive(Debug, Default, Clone, Copy)]
pub struct BasicDateTime;
impl_date_fmt!(BasicDateTime, "%Y%m%dT%H%M%S%.3fZ", "basic_date_time");

/// Format for `epoch_millis`.
///
/// Takes up to a 13 digit string of millis since the epoch and converts to a `DateTime`.
/// This is an efficient formatter, so is a good choice for storing timestamps.
///
/// # Links
/// - [Elasticsearch Doc](https://www.elastic.co/guide/en/elasticsearch/reference/current/mapping-date-format.html#built-in-date-formats)
//TODO: Should simply parse a long instead of checking the length. See: https://github.com/elastic/elasticsearch/issues/17936
#[derive(Debug, Default, Clone, Copy)]
pub struct EpochMillis;
impl DateFormat for EpochMillis {
	fn fmt<'a>() -> Vec<Item<'a>>{
		Vec::new()
	}
	fn name() -> &'static str {
		"epoch_millis"
	}

	fn parse(date: &str) -> Result<DateTime<UTC>, ParseError> {
		let c = try!(date.chars().next().ok_or("Date input was empty".to_string()));
		let (secs, msecs) = match (date.len(), c) {
			//For negative timestamps
			(n, '-') if n <= 4 => {
				let (s, m) = (Ok(-1i64), date[1..].parse::<u32>());
				let mu = try!(m.map_err(|e| e.description().to_string()));

				(s, Ok(1000 - mu))
			},
			(n, '-') if n <= 11 => {
				let (s, m) = (date[0..n-3].parse::<i64>(), date[n-3..].parse::<u32>());
				let mu = try!(m.map_err(|e| e.description().to_string()));

				(s, Ok(1000 - mu))
			},
			(14, '-') => {
				let (s, m) = (date[0..11].parse::<i64>(), date[11..14].parse::<u32>());
				let mu = try!(m.map_err(|e| e.description().to_string()));

				(s, Ok(1000 - mu))
			},
			//For positive timestamps
			(n, _) if n <= 3 => (Ok(0i64), date.parse::<u32>()),
			(n, _) if n <= 10 => (date[0..n-3].parse::<i64>(), date[n-3..].parse::<u32>()),
			(13, _) => (date[0..10].parse::<i64>(), date[10..13].parse::<u32>()),
			_ => return Err("unexpected format".to_owned().into())
		};

		let s = try!(secs.map_err(|e| e.description().to_string()));
		let m = try!(msecs.map_err(|e| e.description().to_string()));

		Ok(DateTime::from_utc(NaiveDateTime::from_num_seconds_from_unix_epoch(s, m * 1000000), UTC))
	}

	fn format(date: &DateTime<UTC>) -> String {
		let mut fmtd = String::with_capacity(13);

		let sec = date.timestamp();

		//For positive timestamps
		if sec >= 0 {
			if sec != 0 {
				let s = sec.to_string();
				fmtd.push_str(&s);
			}

			let msec = (date.nanosecond() / 1000000).to_string();
			fmtd.push_str(&msec);

			fmtd
		}
		//For negative timestamps
		else {
			if sec == -1 {
				fmtd.push_str("-");
			}
			else {
				let s = sec.to_string();
				fmtd.push_str(&s);
			}

			let msec = (1000 - (date.nanosecond() / 1000000)).to_string();
			fmtd.push_str(&msec);

			fmtd
		}
	}
}
