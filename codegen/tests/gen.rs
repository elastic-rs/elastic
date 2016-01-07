#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use syntax::ast::*;
use elastic_codegen::api::gen::*;

#[test]
fn can_build_rust_fn_from_ast() {
	//Create a builder
	let fnbldr = RustApiFnBldr::new();

	//Define a lifetime 'a
	let lifetime = fnbldr.lifetime("'a");

	//Get a function signature
	let mut fun = fnbldr.build_fn("my_fun", vec![
		fnbldr.arg_ptr::<i32>("arg1", Mutability::MutMutable, Some(lifetime)),
		fnbldr.build_arg("arg2", fnbldr.build_ty_ptr("str", Mutability::MutImmutable, Some(lifetime)))
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