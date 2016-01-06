#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use elastic_codegen::api::gen::*;

#[test]
fn can_build_rust_fn_from_ast() {
	//Create a builder
	let bldr = RustApiFnBldr::new();

	//Get a function signature
	let fun = bldr.gen_fn("my_fun", vec![
		bldr.gen_arg("arg1", bldr.gen_ty_ptr::<i32>(Mutability::MutMutable)),
		bldr.gen_arg("arg2", bldr.gen_ty_as("String"))
	]);

	//Write the function
	let fun_str = fun.to_string();
	
	//TODO: This is a crummy way to test a crummy ast builder. But is ok to just verify things are moving in the right direction
	assert_eq!("fn my_fun(arg1: &mut i32, arg2: String){ }", &fun_str);
}