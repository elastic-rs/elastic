#![feature(rustc_private)]

extern crate elastic_codegen;
extern crate syntax;

use elastic_codegen::api::gen::RustApiFnGen;

#[test]
fn can_build_rust_fn_from_ast() {
	let (fun, safety, constness, token, gen) = RustApiFnGen::gen("my_fun");
	let fun = syntax::print::pprust::fun_to_string(&fun, safety, constness, token, None, &gen);

	println!("{}", fun);
	panic!("implement");
}