//! Elasticsearch API Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to a common format for code generation.

pub mod rust;

use std::str;
use chomp;
use chomp::*;

/// Rexport of the chomp `ParseError`.
pub type ApiParseError<'a> = chomp::ParseError<'a, u8, chomp::parsers::Error<u8>>;

/// Finds the Params that make up an Elasticsearch URL Part.
pub fn parse_path_params(url: &str) -> Result<Vec<String>, ApiParseError> {
	parse_only(|i| many(i, |i| parse_path_param(i)), url.as_bytes())
}

/// Finds the Parts that make up an Elasticsearch URL.
pub fn parse_path_parts(url: &str) -> Result<Vec<String>, ApiParseError> {
	parse_only(|i| many(i, |i| parse_path_part(i)), url.as_bytes())
}

/// Builds a format string that can be used by the `format!` macro.
pub fn parse_fmt(url: &str) -> Result<String, ApiParseError> {
	let res: Vec<String> = try!(parse_only(|i| many(i, |i| parse_fmt_seg(i)), url.as_bytes()));

	Ok(res.join(""))
}

/// Finds the module path tree for an Elasticsearch Endpoint.
/*pub fn parse_mod_path(path: &str) -> Result<Vec<String>, ApiParseError> {
	parse_only(|i| many(i, |i| parse_mod_path_part(i)), path.as_bytes())
}*/
pub fn parse_mod_path(path: &str) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    parse_mod_path_parts(path.as_bytes(), &mut parts);
    
    Ok(parts)
}

fn parse_mod_path_parts(path: &[u8], parts: &mut Vec<String>) {
	if path.len() == 0 {
		return;
	}
    
    let (remainder, part) = prs::take_while1(path, |c| c != b'.');
    parts.push(part.to_string());
    
    if (remainder.len() != 0) {
        parse_mod_path_parts(prs::shift(remainder, 1), parts);
    }
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

/*fn parse_mod_path_part(i: Input<u8>) -> U8Result<String> {
    parse!{i;
		//Read to '.'
		let path = take_while1(|c| c != b'.');
		let _ = take(1);

		ret str::from_utf8(path).unwrap().to_string()
	}
}*/

//TODO: Tidy this up, testing non-chomp impl
mod prs {
    use std::str;
    
    pub fn shift_while<F>(i: &[u8], f: F) -> &[u8] where F: Fn(u8) -> bool {
        let mut ctr = 0;
        for c in i {
            if f(*c) {
                ctr += 1;
            }
            else {
                break;
            }
        }

        &i[ctr..]
    }

    pub fn take_while1<F>(i: &[u8], f: F) -> (&[u8], &str) where F: Fn(u8) -> bool {
        let mut ctr = 0;

        for c in i {
            if f(*c) || ctr == 0 {
                ctr += 1;
            }
            else {
                break;
            }
        }

        (&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
    }

    pub fn take_first<F>(i: &[u8], f: F) -> (&[u8], u8) where F: Fn(u8) -> bool {
        let size = i.len();

        let mut ctr = 0;

        for c in i {
            if f(*c) || ctr == size - 1 {
                break;
            }
            else {
                ctr += 1;
            }
        }

        (&i[ctr..], i[ctr])
    }

    pub fn shift(i: &[u8], c: usize) -> &[u8] {
        match c {
            c if c >= i.len() => &[],
            _ => &i[c..]
        }
    }
}