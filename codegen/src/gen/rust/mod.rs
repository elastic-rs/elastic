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

use ::parsers::*;

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
	let mut parts = Vec::new();
    parse_path_parts(path.as_bytes(), &mut parts);
    
    parts
}

fn parse_path_parts(path: &[u8], parts: &mut Vec<String>) {
	if path.len() == 0 {
		return;
	}
    
    let trim_colons = shift_while(path, |c| c == b':');
    let (remainder, seg) = take_while1(trim_colons, |c| c != b':');
    
    parts.push(seg.to_string());
    
    parse_path_parts(remainder, parts);
}