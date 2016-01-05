pub mod parse;

#[macro_export]
macro_rules! date_fmt(
	($fmt:expr) => ({
		$crate::date::format::parse::items(
			$crate::date::format::parse::format($fmt)
		)
	});
);

mod formats;
pub use self::formats::*;

use chrono;
use chrono::DateTime;
use chrono::UTC;
use chrono::format::Parsed;

pub trait Format {
	fn parse(date: &str) -> Result<DateTime<UTC>, String> {
		let fmts = Self::fmt();

		let mut errors: Vec<String> = Vec::with_capacity(fmts.len());
		let mut result: Result<DateTime<UTC>, String> = Err(String::new());

		for fmt in fmts {
			//TODO: Cache this
			let f = parse::to_tokens(fmt);
			let mut parsed = Parsed::new();

			match chrono::format::parse(&mut parsed, date, f.into_iter())
			.map_err(|err| format!("{} : {}", fmt, err).to_string()) {
				Ok(_) => {
					//If the parsed result doesn't contain any time, set it to the default
					if parsed.hour_mod_12.is_none() {
						let _ = parsed.set_hour(0);
						let _ = parsed.set_minute(0);
					}

					//Set the DateTime result
					result = Ok(
						chrono::DateTime::from_utc(
							parsed.to_naive_datetime_with_offset(0).unwrap(), 
							chrono::UTC
						)	
					);
					break;
				},
				Err(e) => errors.push(e)
			}
		}

		result.map_err(|_| errors.join(", "))
	}

	fn fmt() -> Vec<&'static str>;
	fn es_fmt() -> &'static str;
}
