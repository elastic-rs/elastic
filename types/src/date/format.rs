use std::slice::Iter;
use chrono;
use chrono::DateTime;
use chrono::UTC;
use chrono::format::{ Parsed, Item };

//TODO: Proper error trait

pub trait Format {
	fn parse<'a>(date: &str) -> Result<DateTime<UTC>, String> {
		let fmt = Self::fmt();

		let mut parsed = Parsed::new();
		match chrono::format::parse(&mut parsed, date, fmt.iter().cloned())
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
			Err(e) => return Err(e)
		}
	}

	fn format<'a>(date: &DateTime<UTC>) -> String {
		let fmt = Self::fmt();

		date.format_with_items(fmt.iter().cloned()).to_string()
	}

	fn fmt<'a>() -> Vec<Item<'a>>;
	fn name() -> &'static str;
}
