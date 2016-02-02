use chrono;
use chrono::{ DateTime, UTC };
use chrono::format::{ Parsed, Item };

/// A re-export of the `chrono::ParseError`.
pub type ParseError = chrono::ParseError;

/// A format used for parsing and formatting dates.
/// 
/// The format is specified as two functions; `parse` and `format`, which are backed by `chrono::format::Item`s.
/// Not all formats use the `Item`s though, for example `EpochMillis`, which is more efficient than other formats.
pub trait Format {
	/// Parses a date string to a `chrono::DateTime<UTC>` result.
	/// 
	/// The date string must match the format specified by `fmt()`.
	fn parse<'a>(date: &str) -> Result<DateTime<UTC>, ParseError> {
		let fmt = Self::fmt();

		let mut parsed = Parsed::new();
		match chrono::format::parse(&mut parsed, date, fmt.iter().cloned()) {
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

	/// Formats a given `chrono::DateTime<UTC>` as a string.
	/// 
	/// The resulting string is based off the format specified by `fmt()`.
	fn format<'a>(date: &DateTime<UTC>) -> String {
		let fmt = Self::fmt();

		date.format_with_items(fmt.iter().cloned()).to_string()
	}

	/// The format used for parsing and formatting dates.
	/// 
	/// This is specified as a collection of `chrono::format::Item`s for efficiency.
	/// To make it easier to build formats, you can use the `date_fmt` macro to convert a string format to `Item`s at compile time:
	/// 
	/// ```
	/// # #![feature(plugin)]
	/// # #![plugin(elastic_types_codegen)]
	/// # extern crate elastic_types;
	/// # extern crate chrono;
	/// # fn main() {
	/// use chrono::format::Item;
	/// fn fmt<'a>() -> Vec<Item<'a>> {
	/// 	date_fmt!("%Y%m%dT%H%M%SZ")
	/// 		.iter()
	/// 		.map(|t| *t)
	/// 		.collect()
	/// }
	/// # }
	/// ```
	fn fmt<'a>() -> Vec<Item<'a>>;

	/// The name of the format.
	/// 
	/// This is only used for debugging.
	fn name() -> &'static str;
}