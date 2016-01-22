#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use syntax::ast::*;
use elastic_codegen::gen::rust::*;

struct MyStruct;

#[test]
fn can_parse_path() {
	let parsed = parse_path("std::thread::Thread");

	let expected = vec![
		"std".to_string(),
		"thread".to_string(),
		"Thread".to_string()
	];

	let mut success = true;
	for i in 0..parsed.len() {
		if expected[i] != parsed[i] {
			success = false;
			break;
		}
	}

	assert!(success);
}

#[test]
fn can_build_type_from_generic_param_for_std_type() {
	let success = match ty::<i32>().node {
		Ty_::TyPath(_, path) => {
			path.segments.iter().any(|seg| seg.identifier.to_string() == "i32".to_string())
		},
		_ => false
	};

	assert!(success);
}

#[test]
fn can_build_type_from_generic_param_for_custom_type() {
	let success = match ty::<MyStruct>().node {
		Ty_::TyPath(_, path) => {
			path.segments.iter().any(|seg| seg.identifier.to_string() == "MyStruct".to_string())
		},
		_ => false
	};

	assert!(success);
}