//! API Spec Parser
//! 
//! A simple parser that buffers API spec files into memory and uses `serde_json` to deserialise.

extern crate serde_json;

use std::error;
use std::fmt;
use std::io::Read;
use std::fs::File;
use std::fs::read_dir;
use serde_json::{ Value, value };
use super::ast::Endpoint;

use std::io::Error as IoError;
use serde_json::Error as JsonError;

#[derive(Debug)]
enum ParseErrorKind {
	Io(IoError),
	Parse(JsonError),
	Other(String)
}

/// Represents an error encountered during parsing.
/// 
/// This could include errors while reading the file or deserialising the contents.
#[derive(Debug)]
pub struct ParseError {
	kind: ParseErrorKind
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.kind {
			ParseErrorKind::Io(ref err) => write!(f, "IO error: {}", err),
			ParseErrorKind::Parse(ref err) => write!(f, "Parse error: {}", err),
			ParseErrorKind::Other(ref err) => write!(f, "Error: {}", err)
		}
	}
}

impl error::Error for ParseError {
	fn description(&self) -> &str {
		match self.kind {
			ParseErrorKind::Io(ref err) => err.description(),
			ParseErrorKind::Parse(ref err) => err.description(),
			ParseErrorKind::Other(ref err) => &err[..]
		}
	}

	fn cause(&self) -> Option<&error::Error> {
		match self.kind {
			ParseErrorKind::Io(ref err) => Some(err),
			ParseErrorKind::Parse(ref err) => Some(err),
			ParseErrorKind::Other(_) => None
		}
	}
}

impl From<IoError> for ParseError {
	fn from(err: IoError) -> ParseError {
		ParseError {
			kind: ParseErrorKind::Io(err)
		}
	}
}

impl From<JsonError> for ParseError {
	fn from(err: JsonError) -> ParseError {
		ParseError {
			kind: ParseErrorKind::Parse(err)
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

/// The result of parsing an API spec.
pub type ParseResult<T> = Result<T, ParseError>;

/// Parses a Reader from an Elasticsearch API spec to an Endpoint.
/// 
/// Anything that implements `Read` can be used as a source for the spec.
/// 
/// # Examples
/// 
/// Parse from a file:
/// 
/// ```
/// use std::fs::File;
/// use elastic_codegen::api::parse;
/// 
/// let mut f = File::open("spec/api/bulk.json").unwrap();
/// let parsed = parse::from_reader(&mut f).unwrap();
/// ```
pub fn from_reader<R>(rdr: &mut R) -> ParseResult<Endpoint> where R: Read {
	//Read the file to string
	let mut json = String::new();
	let _ = try!(rdr.read_to_string(&mut json));

	let root: Value = try!(serde_json::from_str(&json[..]));

	match root {
		Value::Object(data) => {
			//Get the name and value of the rest of the ast separately
			let (name, tree) = try!(
				data.iter()
				.next()
				.ok_or("unexpected format".to_owned())
			);

			//Deserialise the api ast and set the name
			let mut endpoint = try!(value::from_value::<Endpoint>(tree.to_owned()));
			endpoint.name = Some(name.to_owned());

			Ok(endpoint)
		},
		_ => Err(ParseError::from("unexpected format".to_owned()))
	}
}

/// Parses all Elasticsearch API spec files in a directory to Endpoints.
/// 
/// The parsed endpoints are not necessarily in alphabetical order.
/// 
/// # Examples
/// 
/// ```
/// use elastic_codegen::api::parse;
/// 
/// let parsed = parse::from_dir("spec/api").unwrap();
/// ```
pub fn from_dir(path: &str) -> ParseResult<Vec<Endpoint>> {
	let mut all_parsed: Vec<Endpoint> = Vec::new();

	let paths = read_dir(path).unwrap();
	for path in paths {
		let p = path.unwrap().path();
		println!("parsing: {}", p.display());

		let mut f = File::open(p).unwrap();
		let parsed = try!(from_reader(&mut f));

		all_parsed.push(parsed);
	}

	Ok(all_parsed)
}