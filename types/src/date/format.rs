use chrono;
use chrono::{DateTime, UTC};
use chrono::format::{Parsed, Item};
use std::error::Error;
use std::fmt;

/// A format used for parsing and formatting dates.
///
/// The format is specified as two functions: `parse` and `format`.
pub trait DateFormat
    where Self: Default
{
    /// Parses a date string to a `chrono::DateTime<UTC>` result.
    fn parse(date: &str) -> Result<DateTime<UTC>, ParseError>;

    /// Formats a given `chrono::DateTime<UTC>` as a string.
    fn format(date: &DateTime<UTC>) -> String;

    /// The name of the format.
    ///
    /// This is the string used when defining the format in the field mapping.
    fn name() -> &'static str;
}

/// Parse a date string using an owned slice of items.
pub fn parse_from_tokens<'a>(date: &str, fmt: Vec<Item<'a>>) -> Result<DateTime<UTC>, ParseError> {
    let mut parsed = Parsed::new();
    match chrono::format::parse(&mut parsed, date, fmt.into_iter()) {
        Ok(_) => {
            // If the parsed result doesn't contain any time, set it to the default
            if parsed.hour_mod_12.is_none() {
                let _ = parsed.set_hour(0);
                let _ = parsed.set_minute(0);
            }

            // Set the DateTime result
            let dt = try!(parsed.to_naive_datetime_with_offset(0));
            Ok(chrono::DateTime::from_utc(dt, chrono::UTC))
        }
        Err(e) => Err(e.into()),
    }
}

/// Format a date string using an owned slice of items.
pub fn format_with_tokens<'a>(date: &DateTime<UTC>, fmt: Vec<Item<'a>>) -> String {
    date.format_with_items(fmt.into_iter()).to_string()
}

/// Represents an error encountered during parsing.
#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
}

#[derive(Debug)]
enum ParseErrorKind {
    Chrono(chrono::ParseError),
    Other(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => write!(f, "Chrono error: {}", err),
            ParseErrorKind::Other(ref err) => write!(f, "Error: {}", err),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => err.description(),
            ParseErrorKind::Other(ref err) => &err[..],
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ParseErrorKind::Chrono(ref err) => Some(err),
            ParseErrorKind::Other(_) => None,
        }
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(err: chrono::ParseError) -> ParseError {
        ParseError { kind: ParseErrorKind::Chrono(err) }
    }
}

impl From<String> for ParseError {
    fn from(err: String) -> ParseError {
        ParseError { kind: ParseErrorKind::Other(err) }
    }
}
