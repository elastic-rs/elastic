//! Rust Codegen Helpers
//! 
//! Contains helpers that use the `libsyntax` crate for generating Rust code.
//! The functions and structures in this module are consumed by both the higher-level API and Test codegen modules.

mod fun;
mod ty;
mod item;

pub use self::item::*;
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
/// 
/// The path is split by '::' and each segment is added in order.
/// 
/// # Examples
/// 
/// Parse a path:
/// 
/// ```
/// use elastic_codegen::gen::rust::parse_path;
/// 
/// let parsed = parse_path("crate::mod_a::mod_b::fn");
/// ```
pub fn parse_path(path: &str) -> Vec<String> {
	parse_only(|i| many(i, |i| parse_path_segment(i)), path.as_bytes()).unwrap()
}
