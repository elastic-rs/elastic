#![feature(quote, rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use syntax::ast::*;
use syntax::ext::base::{ ExtCtxt, DummyMacroLoader };
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::intern;
use elastic_codegen::gen::rust::*;
use elastic_codegen::gen::rust::parse::*;

macro_rules! get_ctxt {
    ($cx:ident, $ps:ident, $ml:ident) => {
		$cx = ExtCtxt::new(
			&$ps, vec![],
			syntax::ext::expand::ExpansionConfig::default("qquote".to_string()),
            &mut $ml
		);
		$cx.bt_push(syntax::codemap::ExpnInfo {
			call_site: DUMMY_SP,
			callee: syntax::codemap::NameAndSpan {
				format: syntax::codemap::MacroBang(intern("")),
				allow_internal_unstable: false,
				span: None,
			}
		});
    };
}

struct MyStruct;

#[test]
fn can_add_lifetime_to_fn() {
	//Define a lifetime 'a
	let lifetime = lifetime("'a");

	let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1"),
		arg_ptr::<i32>("arg2", Mutability::Mutable, Some(lifetime)),
		build_arg("arg3", build_ty_ptr("str", Mutability::Immutable, Some(lifetime)))
	])
    .add_lifetime(lifetime);

	assert_eq!(1, fun.generics.lifetimes.len());
}

#[test]
fn can_set_generics_of_fn() {
    let generic = "I";

    let fun = build_fn("my_fun", vec![
		build_arg("arg", build_ty(generic))
	])
    .set_generic_params(vec![
		build_ty_param(generic, vec![
			"Into<Body<'a>>"
		])
	]);

	assert_eq!(1, fun.generics.ty_params.len());
}

#[test]
fn can_set_return_type_of_fn() {
	let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	])
    .set_return::<i32>();

	let retty = match fun.decl.output {
		FunctionRetTy::Ty(t) => Some(t),
		_ => None
	};

	assert!(retty.is_some());
}

#[test]
fn can_add_arg_to_fn() {
    let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	])
    .add_arg(arg::<MyStruct>("arg2"));

    assert_eq!(2, fun.decl.inputs.len());
}

#[test]
fn can_add_args_to_fn() {
    let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	])
    .add_args(vec![
		arg::<MyStruct>("arg2"),
		arg::<MyStruct>("arg3")
	]);

    assert_eq!(3, fun.decl.inputs.len());
}

#[test]
fn can_add_body_stmt_to_fn() {
	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
    let mut mc = DummyMacroLoader;
	let mut cx;
	get_ctxt!(cx, ps, mc);
    let cx = &mut cx;

	//Build a function
	let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	])
    .add_body_stmt(quote_stmt!(cx, let x = 1;).unwrap());

	assert_eq!(1, fun.stmts.len());
}

#[test]
fn can_add_body_stmts_to_fn() {
	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
    let mut mc = DummyMacroLoader;
	let mut cx;
	get_ctxt!(cx, ps, mc);
    let cx = &mut cx;

	//Build a function
	let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	])
    .add_body_stmts(vec![
		quote_stmt!(cx, let x = 1;).unwrap(),
		quote_stmt!(cx, let y = 1;).unwrap()
	]);

	assert_eq!(2, fun.stmts.len());
}

#[test]
fn can_add_body_block_to_fn() {
	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
    let mut mc = DummyMacroLoader;
	let mut cx;
	get_ctxt!(cx, ps, mc);
    let cx = &mut cx;

	//Build a function
	let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	])
    .add_body_block(quote_block!(cx, {
		let x = 1;
		let y = 1;
		x
	}));

	//Assert the statements are added
	assert_eq!(2, fun.stmts.len());
}

#[test]
fn can_set_return_expr_when_adding_body_block_if_fn_has_return_ty() {
	//Build an execution context
	let ps = syntax::parse::ParseSess::new();
    let mut mc = DummyMacroLoader;
	let mut cx;
	get_ctxt!(cx, ps, mc);
    let cx = &mut cx;

	//Build a function that returns i32
	let fun = build_fn("my_fun", vec![
		arg::<MyStruct>("arg1")
	])
    .set_return::<i32>()
    .add_body_block(quote_block!(cx, {
		let x = 1;
		let y = 1;
		x
	}));

	assert!(fun.expr.is_some());
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
		TyKind::Path(_, path) => {
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
	assert_eq!("std::string::String", name);
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
