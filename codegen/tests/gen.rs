#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use syntax::ast::*;
use syntax::codemap::DUMMY_SP;
use syntax::parse::token::intern;
use elastic_codegen::api::gen::*;

#[test]
fn can_build_rust_fn_from_ast() {
	//Create a builder
	let bldr = RustApiFnBldr::new();

	//Define a lifetime 'a
	let lifetime = Lifetime {
		id: DUMMY_NODE_ID,
		span: DUMMY_SP,
		name: intern("'a")
	};

	//Get a function signature
	let mut fun = bldr.gen_fn("my_fun", vec![
		bldr.gen_arg("arg1", bldr.gen_ty_ptr::<i32>(Mutability::MutMutable, Some(lifetime))),
		bldr.gen_arg("arg2", bldr.gen_ty_ptr_as("str", Mutability::MutImmutable, Some(lifetime)))
	]);

	//Add the 'a lifetime to the function declaration
	fun.generics.lifetimes.push(LifetimeDef {
		lifetime: lifetime,
		bounds: Vec::new()
	});

	//Write the function
	let fun_str = fun.to_string();
	
	//TODO: This is a crummy way to test a crummy ast builder. But is ok to just verify things are moving in the right direction
	assert_eq!("fn my_fun<'a>(arg1: &'a mut i32, arg2: &'a str){ }", &fun_str);
}