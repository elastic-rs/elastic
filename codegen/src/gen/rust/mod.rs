mod fun;
mod ty;

pub use self::fun::*;
pub use self::ty::*;

use std::str;
use chomp::*;

fn parse_path_segment(i: Input<u8>) -> U8Result<String> {
	parse!{i;
		//Trim leading ':'
		let _ = take_while(|c| c == b':');
		let seg = take_while1(|c| c != b':');

		ret str::from_utf8(seg).unwrap().to_string()
	}
}

/// Parses a Rust path to its segments.
pub fn parse_path(path: &str) -> Vec<String> {
	parse_only(|i| many(i, |i| parse_path_segment(i)), path.as_bytes()).unwrap()
}