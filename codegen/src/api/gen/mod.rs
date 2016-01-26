//! Elasticsearch API Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to a common format for code generation.

pub mod rust;

use std::str;
use chomp;
use chomp::*;

/// Rexport of the chomp `ParseError`.
pub type UrlParseError<'a> = chomp::ParseError<'a, u8, chomp::parsers::Error<u8>>;

/// Finds the Params that make up an Elasticsearch URL Part.
pub fn parse_path_params(url: &str) -> Result<Vec<String>, UrlParseError> {
	parse_only(|i| many(i, |i| parse_path_param(i)), url.as_bytes())
}

/// Finds the Parts that make up an Elasticsearch URL.
pub fn parse_path_parts(url: &str) -> Result<Vec<String>, UrlParseError> {
	parse_only(|i| many(i, |i| parse_path_part(i)), url.as_bytes())
}

/// Builds a format string that can be used by the `format!` macro.
pub fn parse_fmt(url: &str) -> Result<String, UrlParseError> {
	let res: Vec<String> = try!(parse_only(|i| many(i, |i| parse_fmt_seg(i)), url.as_bytes()));

	Ok(res.join(""))
}

fn parse_path_param(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		//Read to '{' and trim
		let _ = take_while(|c| c != b'{');
		let _ = take(1);

		//Read until '}' encountered
		let param = take_while1(|c| c != b'}');

		ret str::from_utf8(param).unwrap().to_string()
	}
}

fn parse_path_part(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		//Read to '{'
		let path = take_while(|c| c != b'{');
		//Read until '}' encountered
		let _ = take_while1(|c| c != b'}');
		let _ = take(1);

		ret str::from_utf8(path).unwrap().to_string()
	}
}

fn parse_fmt_seg(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		//Read to '{'
		let path = take_while(|c| c != b'{');
		//Read until '}' encountered
		let _ = take_while1(|c| c != b'}');
		let _ = take(1);

		//Build the format replacement part + '{}'
		ret format!("{}{{}}", str::from_utf8(path).unwrap())
	}
}