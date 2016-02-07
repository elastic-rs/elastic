#![feature(quote, rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use std::ops::Deref;
use syntax::ast::*;
use syntax::ext::quote;
use syntax::parse::ParseSess;
use syntax::feature_gate::GatedCfgAttr;
use syntax::ext::base::ExtCtxt;
use syntax::ext::expand::ExpansionConfig;
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::intern;
use elastic_codegen::gen::rust::*;

struct MyStruct;

#[test]
fn can_add_lifetime_to_fn() {	 
	//Define a lifetime 'a
	let lifetime = lifetime("'a");

	let mut fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1"),
		arg_ptr::<i32>("arg2", Mutability::MutMutable, Some(lifetime)),
		build_arg("arg3", build_ty_ptr("str", Mutability::MutImmutable, Some(lifetime)))
	]);
	fun.add_lifetime(lifetime);

	assert_eq!(1, fun.generics.lifetimes.len());
}

#[test]
fn can_add_body_stmt_to_fn() {
	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
	let mut feature_gated_cfgs = vec![];
	let mut cx = syntax::ext::base::ExtCtxt::new(
		&ps, vec![],
		syntax::ext::expand::ExpansionConfig::default("qquote".to_string()),
		&mut feature_gated_cfgs
	);
	cx.bt_push(syntax::codemap::ExpnInfo {
		call_site: DUMMY_SP,
		callee: syntax::codemap::NameAndSpan {
			format: syntax::codemap::MacroBang(intern("")),
			allow_internal_unstable: false,
			span: None,
		}
	});
	let cx = &mut cx;

	//Build a function
	let mut fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	]);

	//Add a statement to the function body
	let stmt = quote_stmt!(cx, let x = 1;).unwrap();
	fun.add_body_stmt(stmt);

	assert_eq!(1, fun.body.stmts.len());
}

#[test]
fn can_set_return_type_of_fn() {
	let mut fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	]);

	fun.set_return::<i32>();

	let retty = match fun.decl.output {
		FunctionRetTy::Return(t) => Some(t),
		_ => None
	};

	assert!(retty.is_some());
}

#[test]
fn can_get_type_name_of_t() {
	let name = type_of::<i32>();
	assert_eq!("i32", name);
}

#[test]
fn can_get_type_name_of_param() {
	let param: i32 = 1;
	let name = infer_type_of(&param);
	assert_eq!("i32", name);
}

#[test]
fn can_build_type_with_name_only() {
	let string_type = ty::<MyStruct>(TyPathOpts::NameOnly);

	let success = match string_type.node {
		Ty_::TyPath(_, path) => {
			path.segments.iter().any(|seg| {
				seg.identifier.to_string() == "MyStruct".to_string()
			})
		},
		_ => false
	};

	assert_eq!(true, success);
}

#[test]
fn can_build_type_with_full_path() {
	let name = type_of::<String>();
	assert_eq!("collections::string::String", name);
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