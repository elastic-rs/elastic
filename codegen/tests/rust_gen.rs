#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use syntax::ast::*;
use elastic_codegen::gen::rust::*;

struct MyStruct;

#[test]
fn can_build_rust_fn_from_ast() {
	//Define a lifetime 'a
	let lifetime = lifetime("'a");

	//Get a function signature
	let mut fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1"),
		arg_ptr::<i32>("arg2", Mutability::MutMutable, Some(lifetime)),
		build_arg("arg3", build_ty_ptr("str", Mutability::MutImmutable, Some(lifetime)))
	]);

	fun.add_lifetime(lifetime);

	//fn my_fun<'a>(arg1: MyStruct, arg2: &'a mut i32, arg3: &'a str){ }
	println!("{}", fun.to_string());
}

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