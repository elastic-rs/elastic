mod formats;
pub use self::formats::*;

use chrono;
use chrono::DateTime;
use chrono::UTC;
use chrono::format::{ Parsed, Item };

pub trait Format {
	fn parse<'a>(date: &str) -> Result<DateTime<UTC>, String> {
		let fmts = Self::fmt();

		let mut errors: Vec<String> = Vec::with_capacity(fmts.len());

		for fmt in fmts {
			let mut parsed = Parsed::new();

			match chrono::format::parse(&mut parsed, date, fmt.into_iter())
			.map_err(|err| format!("Date parse error: {}", err).to_string()) {
				Ok(_) => {
					//If the parsed result doesn't contain any time, set it to the default
					if parsed.hour_mod_12.is_none() {
						let _ = parsed.set_hour(0);
						let _ = parsed.set_minute(0);
					}

					//Set the DateTime result
					return Ok(
						chrono::DateTime::from_utc(
							parsed.to_naive_datetime_with_offset(0).unwrap(), 
							chrono::UTC
						)	
					);
				},
				Err(e) => errors.push(e)
			}
		}

		Err(String::new()).map_err(|_| errors.join(", "))
	}

	fn fmt<'a>() -> Vec<Vec<Item<'a>>>;
	fn fmt_str() -> &'static str;
	fn name() -> &'static str;
}
