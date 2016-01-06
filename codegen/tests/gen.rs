#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use elastic_codegen::api::gen::RustApiFnGen;

#[test]
fn can_build_rust_fn_from_ast() {
	let fun = RustApiFnGen::gen_fn("my_fun");
	let fun_str = syntax::print::pprust::fun_to_string(
		&fun.decl, 
		fun.unsafety, 
		fun.constness, 
		fun.identifier, 
		None, 
		&fun.generics
	);

	println!("{}", fun_str);
	panic!("implement");
}