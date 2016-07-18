//! Traits for parsing and formatting `ElasticDate`s for Elasticsearch.
//! 
//! # Examples
//! 
//! Create a custom date format for `yyyy-MM-dd'T'HH:mm:ss`.
//! To make it easier to build formats, use the `date_fmt` macro 
//! from [`elastic_date_macros`](http://kodraus.github.io/rustdoc/elastic_date_macros/index.html) to convert a string format to `Item`s.
//! 
//! ```
//! # #![feature(plugin)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #![plugin(elastic_date_macros)]
//! # extern crate elastic_types;
//! # extern crate chrono;
//! # use elastic_types::date::DateFormat;
//! # fn main() {
//! #[derive(Default, Clone)]
//! struct MyFormat;
//! impl DateFormat for MyFormat {
//! 	fn fmt<'a>() -> Vec<chrono::format::Item<'a>> {
//! 		date_fmt!("yyyy-MM-ddTHH:mm:ss")
//! 	}
//! 
//! 	fn name() -> &'static str { "yyyy-MM-dd'T'HH:mm:ss" }
//! }
//! # }
//! ```
//! 
//! You can also avoid having to use date formats by implementing `CustomDateFormat` yourself:
//!
//! ```
//! # #![feature(plugin)]
//! # #![plugin(json_str, elastic_types_macros)]
//! # #![plugin(elastic_date_macros)]
//! # extern crate elastic_types;
//! # extern crate chrono;
//! # use chrono::{ DateTime, UTC };
//! # use elastic_types::date::{ CustomDateFormat, ParseError };
//! # fn main() {
//! #[derive(Default, Clone)]
//! struct MyCustomFormat;
//! impl CustomDateFormat for MyCustomFormat {
//! 	fn name() -> &'static str { "yyyy-MM-dd'T'HH:mm:ssZ" }
//! 
//! 	fn format(date: &DateTime<UTC>) -> String {
//! 		date.to_rfc3339()
//! 	}
//! 	
//! 	fn parse(date: &str) -> Result<DateTime<UTC>, ParseError> {
//! 		let date = try!(DateTime::parse_from_rfc3339(date).map_err(|e| ParseError::from(e)));
//! 
//!			Ok(DateTime::from_utc(date.naive_local(), UTC))
//!		}
//!	}
//! # }
//! ```

use chrono;
use chrono::{ DateTime, UTC };
use chrono::format::{ Parsed, Item };
use std::error::Error;
use std::fmt;

/// A format used for parsing and formatting dates.
///
/// The format is specified as two functions; `parse` and `format`, which are backed by `chrono::format::Item`s.
/// Not all formats use the `Item`s though, for example `EpochMillis`, which is more efficient than other formats.
pub trait DateFormat
where Self : Default + Clone {
	/// Parses a date string to a `chrono::DateTime<UTC>` result.
	///
	/// The date string must match the format specified by `fmt()`.
	fn parse(date: &str) -> Result<DateTime<UTC>, ParseError> {
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
				let dt = try!(parsed.to_naive_datetime_with_offset(0));
				Ok(chrono::DateTime::from_utc(dt, chrono::UTC))
			},
			Err(e) => Err(e.into())
		}
	}

	/// Formats a given `chrono::DateTime<UTC>` as a string.
	///
	/// The resulting string is based off the format specified by `fmt()`.
	fn format(date: &DateTime<UTC>) -> String {
		let fmt = Self::fmt();

		date.format_with_items(fmt.iter().cloned()).to_string()
	}

	/// The format used for parsing and formatting dates.
	///
	/// This is specified as a collection of `chrono::format::Item`s for efficiency.
	fn fmt<'a>() -> Vec<Item<'a>>;

	/// The name of the format.
	///
	/// This is the string used when defining the format in the field mapping.
	/// 
	/// It's important to remember that Elasticsearch expects non-symbolic literals in date formats to be escaped,
	/// so `yyyy-MM-ddTHH:mm` needs to be represented as `yyyy-MM-dd'T'HH:mm`.
	fn name() -> &'static str;
}

/// A format used for parsing and formatting dates.
///
/// This trait implements `DateFormat` for you, but requires you parse and format dates yourself.
pub trait CustomDateFormat
where Self : DateFormat {
	/// Parses a date string to a `chrono::DateTime<UTC>` result.
	fn parse(date: &str) -> Result<DateTime<UTC>, ParseError>;

	/// Formats a given `chrono::DateTime<UTC>` as a string.
	fn format(date: &DateTime<UTC>) -> String;

	/// The name of the format.
	///
	/// This is the string used when defining the format in the field mapping.
	/// 
	/// It's important to remember that Elasticsearch expects non-symbolic literals in date formats to be escaped,
	/// so `yyyy-MM-ddTHH:mm` needs to be represented as `yyyy-MM-dd'T'HH:mm`.
	fn name() -> &'static str;
}

impl <T: CustomDateFormat> DateFormat for T {
	/// Parses a date string to a `chrono::DateTime<UTC>` result.
	///
	/// The date string must match the format specified by `fmt()`.
	fn parse(date: &str) -> Result<DateTime<UTC>, ParseError> {
		<Self as CustomDateFormat>::parse(date)
	}

	/// Formats a given `chrono::DateTime<UTC>` as a string.
	///
	/// The resulting string is based off the format specified by `fmt()`.
	fn format(date: &DateTime<UTC>) -> String {
		<Self as CustomDateFormat>::format(date)
	}

	/// The format used for parsing and formatting dates.
	///
	/// This is specified as a collection of `chrono::format::Item`s for efficiency.
	fn fmt<'a>() -> Vec<Item<'a>> {
		Vec::new()
	}

	/// The name of the format.
	///
	/// This is the string used when defining the format in the field mapping.
	/// 
	/// It's important to remember that Elasticsearch expects non-symbolic literals in date formats to be escaped,
	/// so `yyyy-MM-ddTHH:mm` needs to be represented as `yyyy-MM-dd'T'HH:mm`.
	fn name() -> &'static str {
		<Self as CustomDateFormat>::name()
	}
}

/// Represents an error encountered during parsing.
#[derive(Debug)]
pub struct ParseError {
	kind: ParseErrorKind
}

#[derive(Debug)]
enum ParseErrorKind {
    Chrono(chrono::ParseError),
    Other(String)
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.kind {
			ParseErrorKind::Chrono(ref err) => write!(f, "Chrono error: {}", err),
			ParseErrorKind::Other(ref err) => write!(f, "Error: {}", err)
		}
	}
}

impl Error for ParseError {
	fn description(&self) -> &str {
		match self.kind {
			ParseErrorKind::Chrono(ref err) => err.description(),
			ParseErrorKind::Other(ref err) => &err[..]
		}
	}

	fn cause(&self) -> Option<&Error> {
		match self.kind {
			ParseErrorKind::Chrono(ref err) => Some(err),
			ParseErrorKind::Other(_) => None
		}
	}
}

impl From<chrono::ParseError> for ParseError {
	fn from(err: chrono::ParseError) -> ParseError {
		ParseError {
			kind: ParseErrorKind::Chrono(err)
		}
	}
}

impl From<String> for ParseError {
	fn from(err: String) -> ParseError {
		ParseError {
			kind: ParseErrorKind::Other(err)
		}
	}
}
