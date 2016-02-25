//! Elasticsearch API Codegen
//! 
//! Utilities for parsing the Elasticsearch API spec to a common format for code generation.

pub mod rust;

use ::parsers::*;

/// An error encountered while parsing.
pub type ApiParseError = String;

/// Finds the Params that make up an Elasticsearch URL Part.
pub fn parse_path_params(url: &str) -> Result<Vec<String>, ApiParseError> {
	let mut parts = Vec::new();
    parse_path_param_parts(url.as_bytes(), &mut parts);
    
    Ok(parts)
}

fn parse_path_param_parts(path: &[u8], parts: &mut Vec<String>) {
	if path.len() == 0 {
		return;
	}
    
    //Read to '{' and trim
    let pre_part = shift_while(path, |c| c != b'{');
    let part_start = shift(pre_part, 1);

    //Read until '}' encountered
    let (remainder, param) = take_while1(part_start, |c| c != b'}');

    if param.len() > 0 {
        parts.push(param.to_string());
    }
    
    parse_path_param_parts(remainder, parts);
}

/// Finds the Parts that make up an Elasticsearch URL.
pub fn parse_path_parts(url: &str) -> Result<Vec<String>, ApiParseError> {
	let mut parts = Vec::new();
    parse_path_part_parts(url.as_bytes(), &mut parts);
    
    Ok(parts)
}

fn parse_path_part_parts(path: &[u8], parts: &mut Vec<String>) {
	if path.len() == 0 {
		return;
	}
    
    //Read to '{' and trim
    let (remainder, part) = take_while1(path, |c| c != b'{');
    let skip_param = shift_while(remainder, |c| c != b'}');
    
    if part.len() > 0 {
        parts.push(part.to_string());
    }
    
    if skip_param.len() != 0 {
        parse_path_part_parts(shift(skip_param, 1), parts);
    }
}

/// Builds a format string that can be used by the `format!` macro.
pub fn parse_fmt(url: &str) -> Result<String, ApiParseError> {
	let mut parts = Vec::new();
    parse_fmt_parts(url.as_bytes(), &mut parts);
    
    Ok(parts.join(""))
}

fn parse_fmt_parts(path: &[u8], parts: &mut Vec<String>) {
	if path.len() == 0 {
		return;
	}
    
    //Read to '{' and trim
    let (remainder, part) = take_while1(path, |c| c != b'{');
    let skip_param = shift_while(remainder, |c| c != b'}');
    
    if part.len() > 0 {
        parts.push(format!("{}{{}}", part));
    }
    
    if skip_param.len() != 0 {
        parse_fmt_parts(shift(skip_param, 1), parts);
    }
}

/// Finds the module path tree for an Elasticsearch Endpoint.
pub fn parse_mod_path(path: &str) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    parse_mod_path_parts(path.as_bytes(), &mut parts);
    
    Ok(parts)
}

fn parse_mod_path_parts(path: &[u8], parts: &mut Vec<String>) {
	if path.len() == 0 {
		return;
	}
    
    let (remainder, part) = take_while1(path, |c| c != b'.');
    
    if part.len() > 0 {
        parts.push(part.to_string());
    }
    
    if remainder.len() != 0 {
        parse_mod_path_parts(shift(remainder, 1), parts);
    }
}